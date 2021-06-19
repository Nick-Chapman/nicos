
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(nicos::test_runner)]

use nicos::println;
use nicos::print;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nicos::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start () -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
