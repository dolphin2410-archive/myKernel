#![no_std]
#![no_main]

mod print;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("We can do it!\r");
    print!("We can do it!\r");
    println!("We can do it!");
    println!("We can do it!");
    loop {

    }
}