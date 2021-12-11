#![no_std] // No std library
#![no_main] // No rust entry points

use core::panic::PanicInfo;
// Panic function
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod vga_buffer;

#[no_mangle] // Don't change name of function on compilation
pub extern "C" fn _start() -> ! {
    // Entry point function
    println!("Hello how you doing{}", "?");
    panic!("Aborting");
}
