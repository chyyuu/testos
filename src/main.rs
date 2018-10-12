#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![no_std] // don't link the Rust standard library
//#![no_main] // disable all Rust-level entry points
#![cfg_attr(not(test), no_main)] // instead of `#![no_main]`
extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;

pub mod vga_buffer;



use core::panic::PanicInfo;

// This function is called on panic.
#[cfg(not(test))] // only compile when the test flag is not set
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}