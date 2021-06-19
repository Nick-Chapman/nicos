#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(nicos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use nicos::println;
use nicos::print;

#[no_mangle]
pub extern "C" fn _start () -> ! {
    println!["Hello World{}", "!"];

    #[cfg(test)]
    test_main();

    println!["The numbers are {} and {}", 42, 1.0/3.0];
    panic!("Run out of things to do in _start");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    nicos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
