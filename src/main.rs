#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &str = "Hello World!";
static COLOR_BYTE_LIGHT_CYAN: u8 = 0xb;

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Entry point function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, byte) in HELLO.bytes().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = COLOR_BYTE_LIGHT_CYAN;
        }
    }

    loop {}
}
