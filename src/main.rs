#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
extern crate bootloader_precompiled;
extern crate volatile;

pub mod vga_buffer;

use core::panic::PanicInfo;


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
	vga_buffer::print_something();

    loop {}
}