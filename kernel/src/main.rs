#![no_std]
#![no_main]

mod console;

use core::panic::PanicInfo;
use nova_kernel::{applications, devices, drivers, hal, input, runtime, security, services};
use x86_64::instructions::hlt;

bootloader::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    console::init();
    let _input_status = input::init();
    let _application_status = applications::init();
    let _runtime_status = runtime::init();
    let _services_status = services::init();
    let _hal_status = hal::init();
    let _device_status = devices::init();
    let _driver_status = drivers::init();
    let _security_status = security::init();
    let hardware_info = nova_kernel::devices::registry::DeviceRegistry::new();
    let reported_hardware = nova_kernel::hw_info::HardwareInfo::new(&hardware_info);

    console::write_fmt(format_args!(
        "Project Nova | x86_64 experimental kernel\n\
         -----------------------------------------\n\
         Boot protocol: BIOS via bootloader 0.9\n\
         Memory regions: {}\n\
         Graphics: framebuffer driver unavailable.\n\
         Input: keyboard and mouse drivers unavailable.\n\
         Applications: metadata registry available; process launch unavailable.\n\
         Desktop services: in-memory foundations; persistence unavailable.\n\
         Packages: metadata and planning foundations available; installation unavailable.\n\
         Hardware: PCI discovery and device drivers unavailable; registered devices: {}.\n\
         Security: account policy metadata only; authentication and crypto unavailable.\n\
         Status: kernel initialization complete.\n",
        boot_info.memory_map.len(),
        reported_hardware.device_count()
    ))
    .expect("failed to format startup log");

    console::show_software_center();

    halt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    console::write_panic(info);
    halt_loop()
}

fn halt_loop() -> ! {
    loop {
        hlt();
    }
}
