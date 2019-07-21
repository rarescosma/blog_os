// don't link the Rust standard library + disable all Rust-level entry points
#![no_std]
#![no_main]

// initialize test framework
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use blog_os::{print, println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello from RaresOS{}", "!");

    #[cfg(test)]
    test_main();

    println!("Still going strong.");
    loop {}
}

// our existing panic handler
#[cfg(not(test))]
#[panic_handler]
fn _panic(info: &PanicInfo) -> ! { blog_os::panic_handler(info) }

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn _test_panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
