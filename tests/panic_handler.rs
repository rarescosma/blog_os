#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::fmt;
use core::panic::PanicInfo;

use blog_os::{qemu_exit, serial_print, serial_println};

const MSG: &str = "Wooot wooot it seems we've panicked!";
const PANIC_LINE: u32 = 16;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("itest_panic_handler... ");
    panic!(MSG);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    check_location(info);
    check_message(info);

    serial_println!("[ok]");
    qemu_exit::succeed();
    loop {}
}

fn fail(error: &str) -> ! {
    serial_println!("[failed]");
    serial_println!("{}", error);
    qemu_exit::fail();
    loop {}
}

fn check_location(info: &PanicInfo) {
    let loc = info.location().unwrap_or_else(|| fail("no location"));
    if loc.file() != file!() {
        fail("file name wrong")
    }
    if loc.line() != PANIC_LINE {
        fail("file line wrong")
    }
}

#[derive(Debug)]
struct CompareMessage {
    expected: &'static str,
}

impl fmt::Write for CompareMessage {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.expected.starts_with(s) {
            self.expected = &self.expected[s.len()..];
        } else {
            fail("message not equal to expected message");
        }
        Ok(())
    }
}

fn check_message(info: &PanicInfo) {
    use core::fmt::Write;

    let msg = info.message().unwrap_or_else(|| fail("no message"));
    let mut compare_msg = CompareMessage { expected: MSG };
    write!(&mut compare_msg, "{}", msg).unwrap_or_else(|_| fail("write failed"));
    if !compare_msg.expected.is_empty() {
        fail("panic message shorter than expected");
    }
}
