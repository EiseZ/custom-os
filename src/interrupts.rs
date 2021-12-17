use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = { // Create IDT
        let mut idt = InterruptDescriptorTable::new();
        // Define handlers
        idt.breakpoint.set_handler_fn(breakpoint_handler); 
        unsafe {
        idt.double_fault.set_handler_fn(double_fault_handler)
          .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // Make IDT use different stack to prevent reboots
        }
        idt
    };
}

pub fn init_idt() {
  IDT.load();
}

// Print breakpoint info when it occurs
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// Idem
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
  panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
