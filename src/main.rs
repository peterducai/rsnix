#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static OS_VERSION: &str = "0.0.1";
static OS_BUILD_VERSION: u128 = 20092024;
static OS_CODENAME: &str = "Starter";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("RSNix OS {} codename: {}", OS_VERSION, OS_CODENAME);
    println!("build version {}", OS_BUILD_VERSION);
    println!("Created by Daemonna");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}