#![no_std]

pub mod abi;
pub mod applications;
pub mod desktop;
pub mod devices;
pub mod drivers;
pub mod firmware;
pub mod graphics;
pub mod hal;
pub mod hw_info;
pub mod input;
pub mod pkg;
pub mod power;
pub mod runtime;
pub mod security;
pub mod services;
pub mod software_center;
pub mod syscall;
pub mod window_manager;

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::desktop::Desktop;
    use crate::devices::pci::{enumerate_bus, PciDevice, PCI_CONFIG_SIZE};
    use crate::devices::registry::{
        DeviceEntry, DeviceRegistry, DeviceState, RegistryError, MAX_DEVICES,
    };
    use crate::devices::resource::Resource;
    use crate::graphics::color::Rgba;
    use crate::graphics::framebuffer::{
        Framebuffer, FramebufferError, FramebufferInfo, PixelFormat,
    };
    use crate::graphics::renderer::{draw_rect, Rect};
    use crate::hal::memory::MmioRange;
    use crate::pkg::database::{DatabaseError, InstallState, PackageDatabase, MAX_INSTALLED};
    use crate::pkg::installer::{InstallError, Installer, TransactionState};
    use crate::pkg::metadata::{
        validate_archive_path, ManifestError as PackageManifestError, PackageManifest,
    };
    use crate::pkg::repository::{RepositoryError, RepositorySystem};
    use crate::pkg::resolver::{ResolveError, Resolver};
    use crate::runtime::launcher::{self, LaunchError};
    use crate::runtime::manifest::{AppManifest, ManifestError};
    use crate::runtime::registry::{AppRegistry, AppState, RegistryError as RuntimeRegistryError};
    use crate::security::accounts::{
        AccountDatabase, AccountError, AccountStatus, UserId, MAX_ACCOUNTS,
    };
    use crate::security::auth::{self, AuthError};
    use crate::security::crypto::{self, CryptoError};
    use crate::security::encryption::{self, EncryptionError};
    use crate::security::fs_perms::{FileAccess, FsPermission, PermissionError};
    use crate::security::integrity::{self, IntegrityError};
    use crate::security::logging::{LogError, SecurityEvent, SecurityLog, LOG_CAPACITY};
    use crate::security::privilege::PrivilegeContext;
    use crate::security::recovery::{self, RecoveryError};
    use crate::services::notification::{
        Notification, NotificationError, NotificationService, NOTIFICATION_CAPACITY,
    };
    use crate::services::settings::{Settings, Theme};
    use crate::software_center;
    use crate::window_manager::WindowManager;

    #[test]
    #[ignore = "host-side microbenchmark; run explicitly to record software framebuffer throughput"]
    fn benchmark_framebuffer_fill_rect() {
        use std::time::Instant;

        const WIDTH: usize = 640;
        const HEIGHT: usize = 480;
        const FRAMES_PER_SAMPLE: usize = 25;
        const SAMPLE_COUNT: usize = 5;

        let mut pixels = vec![0; WIDTH * HEIGHT * 4];
        let info = FramebufferInfo {
            width: WIDTH,
            height: HEIGHT,
            stride: WIDTH,
            format: PixelFormat::Rgb,
        };
        let mut framebuffer =
            unsafe { Framebuffer::new(pixels.as_mut_ptr(), pixels.len(), info).unwrap() };
        let mut durations = [0_u128; SAMPLE_COUNT];

        for duration in &mut durations {
            let started = Instant::now();
            for _ in 0..FRAMES_PER_SAMPLE {
                std::hint::black_box(framebuffer.fill_rect(
                    0,
                    0,
                    WIDTH,
                    HEIGHT,
                    Rgba::new(1, 2, 3, 255),
                ))
                .unwrap();
            }
            *duration = started.elapsed().as_nanos();
        }

        durations.sort_unstable();
        let median_ns = durations[SAMPLE_COUNT / 2];
        let bytes = (WIDTH * HEIGHT * 4 * FRAMES_PER_SAMPLE) as u128;
        let bytes_per_second = bytes.saturating_mul(1_000_000_000) / median_ns.max(1);
        std::println!(
            "host framebuffer fill_rect: {}x{}, {} full fills/sample, median={} us, throughput={} MB/s",
            WIDTH,
            HEIGHT,
            FRAMES_PER_SAMPLE,
            median_ns / 1_000,
            bytes_per_second / 1_000_000
        );
    }

    #[test]
    fn framebuffer_writes_rgb_pixels_using_stride() {
        let mut pixels = vec![0; 24];
        let info = FramebufferInfo {
            width: 2,
            height: 2,
            stride: 3,
            format: PixelFormat::Rgb,
        };
        let mut framebuffer =
            unsafe { Framebuffer::new(pixels.as_mut_ptr(), pixels.len(), info).unwrap() };

        framebuffer
            .write_pixel(1, 1, Rgba::new(1, 2, 3, 4))
            .unwrap();

        assert_eq!(&pixels[16..20], &[1, 2, 3, 4]);
    }

    #[test]
    fn framebuffer_rejects_out_of_bounds_pixels() {
        let mut pixels = vec![0; 16];
        let info = FramebufferInfo {
            width: 2,
            height: 2,
            stride: 2,
            format: PixelFormat::Bgr,
        };
        let mut framebuffer =
            unsafe { Framebuffer::new(pixels.as_mut_ptr(), pixels.len(), info).unwrap() };

        assert_eq!(
            framebuffer.write_pixel(2, 0, Rgba::WHITE),
            Err(FramebufferError::OutOfBounds)
        );
    }

    #[test]
    fn framebuffer_converts_channels_for_bgr_pixels() {
        let mut pixels = vec![0; 4];
        let info = FramebufferInfo {
            width: 1,
            height: 1,
            stride: 1,
            format: PixelFormat::Bgr,
        };
        let mut framebuffer =
            unsafe { Framebuffer::new(pixels.as_mut_ptr(), pixels.len(), info).unwrap() };

        framebuffer
            .write_pixel(0, 0, Rgba::new(1, 2, 3, 4))
            .unwrap();

        assert_eq!(pixels, [3, 2, 1, 4]);
    }

    #[test]
    fn framebuffer_fills_rectangles_without_touching_neighbors_or_row_padding() {
        let mut pixels = vec![0; 6 * 3 * 4];
        let info = FramebufferInfo {
            width: 4,
            height: 3,
            stride: 6,
            format: PixelFormat::Bgr,
        };
        let mut framebuffer =
            unsafe { Framebuffer::new(pixels.as_mut_ptr(), pixels.len(), info).unwrap() };

        framebuffer
            .fill_rect(1, 1, 2, 1, Rgba::new(1, 2, 3, 4))
            .unwrap();
        framebuffer.fill_rect(4, 2, 0, 1, Rgba::WHITE).unwrap();

        let row_start = 6 * 4;
        assert_eq!(&pixels[row_start..row_start + 4], &[0; 4]);
        assert_eq!(
            &pixels[row_start + 4..row_start + 12],
            &[3, 2, 1, 4, 3, 2, 1, 4]
        );
        assert_eq!(&pixels[row_start + 12..row_start + 24], &[0; 12]);
        assert_eq!(&pixels[2 * 6 * 4..], &[0; 6 * 4]);
    }

    #[test]
    fn renderer_clips_rectangles_to_framebuffer_bounds() {
        let mut pixels = vec![0; 4 * 4 * 4];
        let info = FramebufferInfo {
            width: 4,
            height: 4,
            stride: 4,
            format: PixelFormat::Rgb,
        };
        let mut framebuffer =
            unsafe { Framebuffer::new(pixels.as_mut_ptr(), pixels.len(), info).unwrap() };

        draw_rect(
            &mut framebuffer,
            &Rect::new(3, 3, usize::MAX, usize::MAX),
            Rgba::RED,
        )
        .unwrap();

        assert_eq!(&pixels[60..64], &[220, 50, 50, 255]);
        assert_eq!(&pixels[0..4], &[0, 0, 0, 0]);
    }

    #[test]
    fn rectangle_contains_uses_half_open_bounds_without_overflow() {
        let rect = Rect::new(usize::MAX - 1, 4, 8, 2);

        assert!(rect.contains(usize::MAX - 1, 4));
        assert!(!rect.contains(1, 4));
        assert!(!rect.contains(usize::MAX - 1, 6));
    }

    #[test]
    fn desktop_keeps_wallpaper_visible_outside_panel_and_dock() {
        let mut pixels = vec![0; 320 * 200 * 4];
        let info = FramebufferInfo {
            width: 320,
            height: 200,
            stride: 320,
            format: PixelFormat::Rgb,
        };
        let mut framebuffer =
            unsafe { Framebuffer::new(pixels.as_mut_ptr(), pixels.len(), info).unwrap() };

        Desktop::new()
            .render(&mut framebuffer, &WindowManager::new())
            .unwrap();

        let wallpaper_pixel = 40 * 320 * 4;
        assert_eq!(
            &pixels[wallpaper_pixel..wallpaper_pixel + 4],
            &[20, 20, 30, 255]
        );
    }

    #[test]
    fn manifest_parser_reads_required_fields_and_capabilities() {
        let manifest = r#"
            [application]
            id = "terminal"
            name = "Terminal"
            version = "0.1.0"
            exec = "/apps/terminal.bin"
            icon = "/icons/terminal.png"
            caps = ["fs.read", "graphics.window"]
        "#
        .parse::<AppManifest>()
        .unwrap();

        assert_eq!(manifest.id(), "terminal");
        assert_eq!(manifest.capability_count(), 2);
        assert_eq!(manifest.capability(1), Some("graphics.window"));
    }

    #[test]
    fn manifest_parser_rejects_unknown_capabilities_and_unsafe_paths() {
        let unknown_capability = r#"
            id = "app"
            name = "App"
            version = "1"
            exec = "/apps/app"
            caps = ["kernel.raw"]
        "#;
        assert_eq!(
            unknown_capability.parse::<AppManifest>(),
            Err(ManifestError::UnknownCapability)
        );

        let unsafe_path = r#"
            id = "app"
            name = "App"
            version = "1"
            exec = "/apps/../secret"
        "#;
        assert_eq!(
            unsafe_path.parse::<AppManifest>(),
            Err(ManifestError::InvalidPath)
        );
    }

    #[test]
    fn manifest_parser_rejects_missing_and_duplicate_fields() {
        let missing_field = r#"
            id = "app"
            name = "App"
            exec = "/apps/app"
        "#;
        assert_eq!(
            missing_field.parse::<AppManifest>(),
            Err(ManifestError::MissingField)
        );

        let duplicate_field = r#"
            id = "app"
            id = "other"
            name = "App"
            version = "1"
            exec = "/apps/app"
        "#;
        assert_eq!(
            duplicate_field.parse::<AppManifest>(),
            Err(ManifestError::DuplicateField)
        );
    }

    #[test]
    fn registry_tracks_application_lifecycle_without_global_mutable_state() {
        let manifest = r#"
            id = "terminal"
            name = "Terminal"
            version = "0.1"
            exec = "/apps/terminal"
        "#
        .parse::<AppManifest>()
        .unwrap();
        let mut registry = AppRegistry::new();
        let id = registry.register(manifest).unwrap();

        assert_eq!(registry.get(id).unwrap().state, AppState::Registered);
        assert_eq!(
            registry.register(manifest),
            Err(RuntimeRegistryError::DuplicateManifest)
        );
        assert_eq!(
            registry.mark_running(id, 0),
            Err(RuntimeRegistryError::InvalidState)
        );

        registry.mark_running(id, 7).unwrap();
        assert_eq!(registry.get(id).unwrap().pid, Some(7));
        registry.mark_finished(id, false).unwrap();
        assert_eq!(registry.get(id).unwrap().state, AppState::Exited);
        assert_eq!(
            launcher::launch(id),
            Err(LaunchError::ProcessExecutionUnavailable)
        );
    }

    #[test]
    fn registry_enforces_its_fixed_capacity() {
        let mut registry = AppRegistry::new();
        for index in 0..32 {
            let source = std::format!(
                "id = \"app-{index}\"\nname = \"App\"\nversion = \"1\"\nexec = \"/apps/app\""
            );
            let manifest = source.parse::<AppManifest>().unwrap();
            registry.register(manifest).unwrap();
        }

        let manifest = r#"
            id = "overflow"
            name = "Overflow"
            version = "1"
            exec = "/apps/overflow"
        "#
        .parse::<AppManifest>()
        .unwrap();
        assert_eq!(registry.register(manifest), Err(RuntimeRegistryError::Full));
    }

    #[test]
    fn notification_service_is_bounded_and_fifo() {
        let mut service = NotificationService::new();

        for app_id in 0..NOTIFICATION_CAPACITY as u32 {
            service
                .push(Notification::new(app_id, "ready", "system online").unwrap())
                .unwrap();
        }
        let too_long_title = "x".repeat(65);
        assert_eq!(
            Notification::new(0, &too_long_title, "body"),
            Err(NotificationError::TextTooLong)
        );
        assert_eq!(
            service.push(Notification::new(0, "overflow", "body").unwrap()),
            Err(NotificationError::QueueFull)
        );
        assert_eq!(service.pop().unwrap().app_id(), 0);
        service
            .push(Notification::new(16, "after wrap", "body").unwrap())
            .unwrap();
        assert_eq!(service.len(), NOTIFICATION_CAPACITY);
        assert_eq!(service.pop().unwrap().app_id(), 1);
    }

    #[test]
    fn settings_theme_is_explicitly_in_memory() {
        let mut settings = Settings::new();
        assert_eq!(settings.theme(), Theme::Light);
        settings.set_theme(Theme::Dark);
        assert_eq!(settings.theme(), Theme::Dark);
    }

    fn package_manifest(id: &str, version: &str, extra: &str) -> PackageManifest {
        std::format!(
            "[package]\nid = \"{id}\"\nname = \"Package\"\nversion = \"{version}\"\narchitecture = \"x86_64\"\n[exec]\npath = \"/usr/bin/{id}\"\n{extra}"
        )
        .parse()
        .unwrap()
    }

    #[test]
    fn package_manifest_parses_fields_and_minimum_dependencies() {
        let manifest = r#"
            [package]
            id = "com.example.terminal"
            name = "Terminal"
            version = "1.0.0"
            architecture = "x86_64"
            [exec]
            path = "/usr/bin/terminal"
            [desktop]
            icon = "/usr/share/icons/terminal.png"
            [dependencies]
            requires = ["nova-libs >= 1.2.0", "font-pack"]
        "#
        .parse::<PackageManifest>()
        .unwrap();

        assert_eq!(manifest.id().as_str(), "com.example.terminal");
        assert_eq!(manifest.name(), "Terminal");
        assert_eq!(manifest.version().as_str(), "1.0.0");
        assert_eq!(manifest.icon_path(), Some("/usr/share/icons/terminal.png"));
        assert_eq!(manifest.dependencies().len(), 2);
        assert_eq!(
            manifest.dependencies()[0]
                .minimum_version()
                .unwrap()
                .as_str(),
            "1.2.0"
        );
    }

    #[test]
    fn package_manifest_rejects_bad_architecture_and_paths() {
        let unsupported = r#"
            [package]
            id = "app"
            name = "App"
            version = "1"
            architecture = "aarch64"
            [exec]
            path = "/usr/bin/app"
        "#;
        assert_eq!(
            unsupported.parse::<PackageManifest>(),
            Err(PackageManifestError::UnsupportedArchitecture)
        );

        let traversal = r#"
            [package]
            id = "app"
            name = "App"
            version = "1"
            architecture = "x86_64"
            [exec]
            path = "/usr/bin/../etc/passwd"
        "#;
        assert_eq!(
            traversal.parse::<PackageManifest>(),
            Err(PackageManifestError::InvalidPath)
        );
        assert_eq!(
            validate_archive_path("usr/bin/../../etc/passwd"),
            Err(PackageManifestError::InvalidPath)
        );
        assert_eq!(
            validate_archive_path("/etc/passwd"),
            Err(PackageManifestError::InvalidPath)
        );
        assert!(validate_archive_path("usr/bin/app").is_ok());
    }

    #[test]
    fn package_database_tracks_active_records_and_reuses_removed_slots() {
        let mut database = PackageDatabase::new();
        let manifest = package_manifest("terminal", "1.0.0", "");
        database.install_record(manifest).unwrap();
        assert_eq!(database.len(), 1);
        assert_eq!(
            database.install_record(manifest),
            Err(DatabaseError::AlreadyInstalled)
        );

        database.remove(manifest.id()).unwrap();
        assert_eq!(database.len(), 0);
        assert!(database.find(manifest.id()).is_none());
        assert_eq!(
            database.install_record(package_manifest("editor", "1.0.0", "")),
            Ok(())
        );
        assert_eq!(database.len(), 1);
        assert_eq!(
            database
                .find(package_manifest("editor", "1.0.0", "").id())
                .unwrap()
                .state(),
            InstallState::Installed
        );
    }

    #[test]
    fn package_database_enforces_capacity() {
        let mut database = PackageDatabase::new();
        for index in 0..MAX_INSTALLED {
            let manifest = package_manifest(&std::format!("package-{index}"), "1.0.0", "");
            database.install_record(manifest).unwrap();
        }
        assert_eq!(database.len(), MAX_INSTALLED);
        assert_eq!(
            database.install_record(package_manifest("overflow", "1.0.0", "")),
            Err(DatabaseError::Full)
        );
    }

    #[test]
    fn dependency_resolver_checks_presence_version_and_cycles() {
        let library = package_manifest("nova-libs", "1.2.0", "");
        let mut database = PackageDatabase::new();
        database.install_record(library).unwrap();

        let app = package_manifest(
            "terminal",
            "1.0.0",
            "[dependencies]\nrequires = [\"nova-libs >= 1.1.0\"]\n",
        );
        assert_eq!(Resolver::new(&database).resolve(&app), Ok(()));

        let missing = package_manifest(
            "missing-app",
            "1.0.0",
            "[dependencies]\nrequires = [\"missing-lib\"]\n",
        );
        assert_eq!(
            Resolver::new(&database).resolve(&missing),
            Err(ResolveError::MissingDependency)
        );

        let too_new = package_manifest(
            "new-app",
            "1.0.0",
            "[dependencies]\nrequires = [\"nova-libs >= 2.0.0\"]\n",
        );
        assert_eq!(
            Resolver::new(&database).resolve(&too_new),
            Err(ResolveError::VersionConflict)
        );

        let cyclic_library = package_manifest(
            "cyclic-lib",
            "1.0.0",
            "[dependencies]\nrequires = [\"cyclic-app\"]\n",
        );
        database.install_record(cyclic_library).unwrap();
        let cyclic_app = package_manifest(
            "cyclic-app",
            "1.0.0",
            "[dependencies]\nrequires = [\"cyclic-lib\"]\n",
        );
        assert_eq!(
            Resolver::new(&database).resolve(&cyclic_app),
            Err(ResolveError::CycleDetected)
        );
    }

    #[test]
    fn repository_configuration_is_bounded_and_rejects_plain_http() {
        let mut repositories = RepositorySystem::new();
        repositories
            .add("stable", "https://packages.example.test")
            .unwrap();
        assert_eq!(repositories.enabled_count(), 1);
        assert_eq!(
            repositories.add("insecure", "http://packages.example.test"),
            Err(RepositoryError::InvalidUrl)
        );
        assert_eq!(
            repositories.add("empty-host", "https://"),
            Err(RepositoryError::InvalidUrl)
        );
        assert_eq!(
            repositories.add("stable", "https://mirror.example.test"),
            Err(RepositoryError::Duplicate)
        );
        repositories.set_enabled("stable", false).unwrap();
        assert_eq!(repositories.enabled_count(), 0);
        assert_eq!(
            repositories.get("stable").unwrap().url(),
            "https://packages.example.test"
        );
    }

    #[test]
    fn package_installer_fails_explicitly_without_storage() {
        let mut installer = Installer::new();
        let manifest = package_manifest("terminal", "1.0.0", "");
        assert_eq!(
            installer.install(&manifest),
            Err(InstallError::FilesystemUnavailable)
        );
        assert_eq!(installer.state(), TransactionState::Unavailable);
    }

    #[test]
    fn mmio_ranges_reject_overflow_and_use_half_open_bounds() {
        let range = MmioRange::new(0x1000, 0x100).unwrap();
        assert!(range.contains(0x1000));
        assert!(range.contains(0x10ff));
        assert!(!range.contains(0x1100));
        assert!(range.overlaps(MmioRange::new(0x10f0, 0x20).unwrap()));
        assert!(!range.overlaps(MmioRange::new(0x1100, 0x10).unwrap()));
        assert!(MmioRange::new(u64::MAX - 2, 4).is_none());
        assert!(MmioRange::new(0, 0).is_none());
    }

    #[test]
    fn device_registry_is_bounded_and_tracks_driver_state() {
        let mut registry = DeviceRegistry::new();
        let id = registry
            .register(DeviceEntry::new(0x1234, 0x5678, 0x02, 0x00))
            .unwrap();
        registry.set_driver(id, "virtio-net").unwrap();
        registry.set_state(id, DeviceState::DriverMatched).unwrap();
        let entry = registry.get(id).unwrap();
        assert_eq!(entry.vendor_id(), 0x1234);
        assert_eq!(entry.device_id(), 0x5678);
        assert_eq!(entry.driver_name(), "virtio-net");
        assert_eq!(entry.state(), DeviceState::DriverMatched);

        assert_eq!(
            registry.set_state(crate::devices::registry::DeviceId(31), DeviceState::Active),
            Err(RegistryError::InvalidDevice)
        );

        for _ in 1..MAX_DEVICES {
            registry.register(DeviceEntry::new(1, 1, 0, 0)).unwrap();
        }
        assert_eq!(
            registry.register(DeviceEntry::new(1, 1, 0, 0)),
            Err(RegistryError::Full)
        );
    }

    #[test]
    fn device_resources_detect_overlaps() {
        let first = Resource::Mmio(MmioRange::new(0x1000, 0x100).unwrap());
        let adjacent = Resource::Mmio(MmioRange::new(0x1100, 0x10).unwrap());
        assert!(!first.overlaps(adjacent));
        assert!(first.overlaps(Resource::Mmio(MmioRange::new(0x10f0, 0x10).unwrap())));
        assert!(Resource::port_range(0x20, 0x2f)
            .unwrap()
            .overlaps(Resource::port_range(0x2f, 0x3f).unwrap()));
        assert!(Resource::port_range(0x30, 0x20).is_none());
        assert!(Resource::Irq(5).overlaps(Resource::Irq(5)));
        assert!(!Resource::Irq(5).overlaps(Resource::Irq(6)));
    }

    #[test]
    fn pci_parser_decodes_config_and_ignores_absent_functions() {
        let mut config = [0; PCI_CONFIG_SIZE];
        config[0..2].copy_from_slice(&0x1234_u16.to_le_bytes());
        config[2..4].copy_from_slice(&0x5678_u16.to_le_bytes());
        config[8] = 0x9a;
        config[9] = 0x01;
        config[0x0a] = 0x02;
        config[0x0b] = 0x03;
        let pci = PciDevice::from_config(2, 4, 0, &config).unwrap();
        assert_eq!(pci.vendor_id, 0x1234);
        assert_eq!(pci.device_id, 0x5678);
        assert_eq!(pci.class_code, 0x03);
        assert_eq!(pci.subclass, 0x02);
        assert_eq!(pci.prog_if, 0x01);
        assert_eq!(pci.revision_id, 0x9a);
        assert!(PciDevice::from_config(0, 0, 0, &[0; PCI_CONFIG_SIZE]).is_none());

        config[0..2].copy_from_slice(&u16::MAX.to_le_bytes());
        assert!(PciDevice::from_config(0, 0, 0, &config).is_none());
    }

    #[test]
    fn pci_bus_scanner_respects_multifunction_header_and_callback_order() {
        let mut visited = std::vec::Vec::new();
        enumerate_bus(
            3,
            |bus, device, function, config| {
                if bus != 3 || device != 4 {
                    return false;
                }
                if function == 0 {
                    config[0..2].copy_from_slice(&0x1234_u16.to_le_bytes());
                    config[2..4].copy_from_slice(&0x5678_u16.to_le_bytes());
                    config[0x0e] = 0x80;
                    return true;
                }
                if function == 2 {
                    config[0..2].copy_from_slice(&0xabcd_u16.to_le_bytes());
                    config[2..4].copy_from_slice(&0xef01_u16.to_le_bytes());
                    return true;
                }
                false
            },
            |device| visited.push((device.bus, device.device, device.function)),
        );
        assert_eq!(visited, [(3, 4, 0), (3, 4, 2)]);
    }

    #[test]
    fn software_center_qemu_screen_fits_vga_text_mode() {
        let lines = software_center::text_screen();
        assert_eq!(lines.len(), 25);
        assert!(lines.iter().all(|line| line.is_ascii() && line.len() < 80));
        assert!(lines
            .iter()
            .any(|line| line.contains("INSTALLATION NOT AVAILABLE")));
    }

    #[test]
    fn account_database_validates_names_and_tracks_volatile_metadata() {
        let mut accounts = AccountDatabase::new();
        let alice = accounts.create_account("alice_1").unwrap();
        assert_eq!(alice, UserId(1000));
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts.find_by_uid(alice).unwrap().username(), "alice_1");
        assert_eq!(
            accounts.find_by_uid(alice).unwrap().home_directory(),
            "/home/alice_1"
        );
        assert_eq!(
            accounts.create_account("alice_1"),
            Err(AccountError::DuplicateUsername)
        );
        assert_eq!(
            accounts.create_account("../alice"),
            Err(AccountError::InvalidUsername)
        );
        accounts.set_status(alice, AccountStatus::Locked).unwrap();
        assert_eq!(
            accounts.find_by_uid(alice).unwrap().status(),
            AccountStatus::Locked
        );
        accounts.set_status(alice, AccountStatus::Disabled).unwrap();
        assert_eq!(
            accounts.find_by_username("alice_1").unwrap().status(),
            AccountStatus::Disabled
        );
        assert_eq!(
            accounts.set_status(UserId(9999), AccountStatus::Locked),
            Err(AccountError::AccountNotFound)
        );
    }

    #[test]
    fn account_database_is_bounded() {
        let mut accounts = AccountDatabase::new();
        for index in 0..MAX_ACCOUNTS {
            let username = std::format!("user{index}");
            accounts.create_account(&username).unwrap();
        }
        assert_eq!(accounts.len(), MAX_ACCOUNTS);
        assert_eq!(accounts.create_account("overflow"), Err(AccountError::Full));
    }

    #[test]
    fn authentication_and_cryptographic_services_fail_closed() {
        assert_eq!(
            auth::authenticate("alice"),
            Err(AuthError::AuthenticationUnavailable)
        );
        assert_eq!(
            auth::change_password(UserId(1000)),
            Err(AuthError::AuthenticationUnavailable)
        );
        assert_eq!(
            crypto::secure_random_fill(&mut [0; 8]),
            Err(CryptoError::SecureRandomUnavailable)
        );
        assert_eq!(
            crypto::constant_time_compare(b"same", b"same"),
            Err(CryptoError::ConstantTimePrimitiveUnavailable)
        );
        assert_eq!(
            encryption::encrypt(b"data", &[0; 32]),
            Err(EncryptionError::AuthenticatedEncryptionUnavailable)
        );
        assert_eq!(
            encryption::decrypt(b"ciphertext", &[0; 32]),
            Err(EncryptionError::AuthenticatedEncryptionUnavailable)
        );
        assert_eq!(
            integrity::verify_kernel_image(b"image", b"signature"),
            Err(IntegrityError::VerificationUnavailable)
        );
        assert_eq!(
            integrity::verify_package_signature(b"package", b"signature"),
            Err(IntegrityError::VerificationUnavailable)
        );
        assert_eq!(
            recovery::enter_recovery_mode(),
            Err(RecoveryError::RecoveryUnavailable)
        );
    }

    #[test]
    fn file_access_policy_checks_owner_other_and_admin_modes() {
        let access = FileAccess::new(UserId(1000), 0b110, 0b100).unwrap();
        let owner = PrivilegeContext::for_policy_user(UserId(1000));
        let other = PrivilegeContext::for_policy_user(UserId(1001));
        let administrator = PrivilegeContext::for_policy_administrator(UserId(0));
        assert!(access.check(owner, FsPermission::Read));
        assert!(access.check(owner, FsPermission::Write));
        assert!(!access.check(owner, FsPermission::Execute));
        assert!(access.check(other, FsPermission::Read));
        assert!(!access.check(other, FsPermission::Write));
        assert!(access.check(administrator, FsPermission::Execute));
        assert_eq!(
            FileAccess::new(UserId(1000), 0b1000, 0),
            Err(PermissionError::InvalidMode)
        );
    }

    #[test]
    fn security_log_is_bounded_and_fifo_with_wraparound() {
        let mut log = SecurityLog::new();
        for _ in 0..LOG_CAPACITY {
            log.push(SecurityEvent::AuthenticationFailed).unwrap();
        }
        assert_eq!(
            log.push(SecurityEvent::PermissionDenied),
            Err(LogError::Full)
        );
        assert_eq!(log.pop(), Some(SecurityEvent::AuthenticationFailed));
        log.push(SecurityEvent::PermissionDenied).unwrap();
        assert_eq!(log.len(), LOG_CAPACITY);
        for _ in 0..LOG_CAPACITY - 1 {
            assert_eq!(log.pop(), Some(SecurityEvent::AuthenticationFailed));
        }
        assert_eq!(log.pop(), Some(SecurityEvent::PermissionDenied));
        assert!(log.is_empty());
        assert_eq!(log.pop(), None);
    }

    #[test]
    fn public_abi_decodes_only_defined_numbers_and_dispatch_fails_closed() {
        assert_eq!(crate::abi::SYSCALL_API_VERSION, 1);
        assert_eq!(
            crate::abi::Syscall::from_number(0),
            Some(crate::abi::Syscall::WriteLog)
        );
        assert_eq!(
            crate::abi::Syscall::from_number(12),
            Some(crate::abi::Syscall::Exit)
        );
        assert_eq!(crate::abi::Syscall::from_number(13), None);
        assert!(crate::abi::is_valid_syscall(0));
        assert!(!crate::abi::is_valid_syscall(u32::MAX));

        let valid = crate::syscall::SyscallContext::new(
            crate::abi::SYSCALL_API_VERSION,
            crate::abi::Syscall::WriteLog.number(),
            [0; 6],
        );
        assert_eq!(
            crate::syscall::dispatch(&valid),
            Err(crate::abi::SyscallError::NotSupported)
        );

        let invalid_version = crate::syscall::SyscallContext::new(u32::MAX, 0, [0; 6]);
        assert_eq!(
            crate::syscall::dispatch(&invalid_version),
            Err(crate::abi::SyscallError::InvalidArgument)
        );

        let invalid_call =
            crate::syscall::SyscallContext::new(crate::abi::SYSCALL_API_VERSION, 13, [0; 6]);
        assert_eq!(
            crate::syscall::dispatch(&invalid_call),
            Err(crate::abi::SyscallError::InvalidArgument)
        );
    }
}
