#![no_std]
#![no_main]
static HELLO: &[u8] = b"Hello World!";

use core::panic::PanicInfo;


// *Disables name mangling i.e technique used to solve probs caused
// *by name clashes in the code.



// *basically like an exception handler but without std
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
    // ! the entry point as linker is looking for _start func by default

}

// ! run with this [cargo build --target thumbv7em-none-eabihf]



