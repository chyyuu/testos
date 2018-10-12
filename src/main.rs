#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
#![no_std] // don't link the Rust standard library
//#![no_main] // disable all Rust-level entry points
#![cfg_attr(not(test), no_main)] // instead of `#![no_main]`
extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;
extern crate uart_16550;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;

pub mod vga_buffer;
#[macro_use]
mod serial;

use core::panic::PanicInfo;

// This function is called on panic.
#[cfg(not(test))] // only compile when the test flag is not set
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

#[cfg(not(feature = "integration-test"))] // new
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!"); // prints to vga buffer
    serial_println!("Hello Host{}", "!");
    // normal execution
    unsafe { exit_qemu(); }
    loop {}
}

#[cfg(feature = "integration-test")] // new
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Hello Host{}", "!");

    run_test_1();
    run_test_2();
    // run more tests

    unsafe { exit_qemu(); }

    loop {}
}