#![no_std]
#![no_main]

mod console;

use core::panic::PanicInfo;
use x86_64::instructions::hlt;

bootloader::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    console::init();
    console::write_fmt(format_args!(
        "Project Nova | x86_64 experimental kernel\n\
         -----------------------------------------\n\
         Boot protocol: BIOS via bootloader 0.9\n\
         Memory regions: {}\n\
         Status: startup complete; subsystems are not yet implemented.\n",
        boot_info.memory_map.len()
    ))
    .expect("failed to format startup log");

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
