#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start () -> ! {

    println!["Hello World{}", "!"];
    println!["The numbers are {} and {}", 42, 1.0/3.0];

    panic!("Run out of things to do in _start");
}
