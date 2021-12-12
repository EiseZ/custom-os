#![no_std] // No std library
#![no_main] // No rust entry points
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(custom_os::test_runner)]
#![reexport_test_harness_main = "test_main"] // change generated main functions name

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    custom_os::test_panic_handler(info)
}

#[no_mangle] // Don't change name of function on compilation
pub extern "C" fn _start() -> ! {
    // Entry point function
    test_main();

    loop {};
}

/* TESTS */
use custom_os::println;

#[test_case]
fn test_println() {
  println!("test_println");
}
