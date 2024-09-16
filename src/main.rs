#![no_std]
#![no_main]

use core::panic::PanicInfo;

static RSNIX: &[u8] = b"RSNIX OS";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //buffer is located at address 0xb8000
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in RSNIX.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}