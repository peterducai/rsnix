#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static BUILD_VERSION: u128 = 0;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("RSNix OS {}", "version 0.0.1");
    println!("{}", BUILD_VERSION);

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}