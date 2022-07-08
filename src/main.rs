#![no_std]
#![no_main]

use core::panic::PanicInfo;


// *Disables name mangling i.e technique used to solve probs caused
// *by name clashes in the code.

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("Hello World!");
}

// *basically like an exception handler but without std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// ! run with this [cargo build --target thumbv7em-none-eabihf]



