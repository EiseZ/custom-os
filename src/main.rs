#![no_std] // No std library
#![no_main] // No rust entry points
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(custom_os::test_runner)]
#![reexport_test_harness_main = "test_main"] // change generated main functions name

use core::panic::PanicInfo;
// Panic function (not testing)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
// Panic function (testing)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  custom_os::test_panic_handler(info)
}

mod vga_buffer;
#[no_mangle] // Don't change name of function on compilation
pub extern "C" fn _start() -> ! {
    // Entry point function
    println!("Hello how you doing{}", "?");

    custom_os::init(); // Init the kernel
    
    fn overflow() {
      overflow();
    }
    overflow();

    #[cfg(test)]
    test_main();

    panic!("Aborting");
}
