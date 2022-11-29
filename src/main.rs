#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Entry point function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[allow(clippy::empty_loop)]
    loop {}
}
