// Test for stackoverflows

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    custom_os::test_panic_handler(info)
}

use custom_os::serial_print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  serial_print!("stack_overflow::stack_overflow...\t");

  custom_os::gdt::init();
  init_test_idt();

  stack_overflow();

  panic!("Execution continued after stack overflow!");
}

#[allow(unconditional_recursion)] // Prevent warning
fn stack_overflow() {
  stack_overflow(); // Create recursion to let a stack overflow happen
  volatile::Volatile::new(0).read(); // No recursion optimization
}


/// Create custom IDT for the test
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(custom_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

/// Create custom double fault handler for the test
use custom_os::{exit_qemu, QemuExitCode, serial_println};
use x86_64::structures::idt::InterruptStackFrame;

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
