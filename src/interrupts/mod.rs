#[macro_use]
mod idt;

use pic8259;

pub fn init() {
    GLOBAL_IDT.load();
}


#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

// https://wiki.osdev.org/Exceptions

extern "C" fn divide_by_zero_irq(stack_frame: &ExceptionStackFrame) {
    kprintln!("\n[KERNEL] EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}


pub extern "C" fn double_fault_irq(stack_frame: &ExceptionStackFrame) {
    kprintln!("\n[KERNEL] EXCEPTION: DOUBLE FAULT: {:#x}\n{:#?}", stack_frame.instruction_pointer, stack_frame);
    loop {}
}

extern "C" fn invalid_opcode_irq(stack_frame: &ExceptionStackFrame) {
    let stack_frame = unsafe { &*stack_frame };
    kprintln!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}", stack_frame.instruction_pointer, stack_frame);
    loop {}
}

extern "C" fn page_fault_irq(stack_frame: &ExceptionStackFrame, error_code: u64) {
    kprintln!(
        "\nEXCEPTION: PAGE FAULT with error code {:?}\n{:#?}",
        error_code, unsafe { &*stack_frame });
    loop {}
}

extern "C" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    let stack_frame = unsafe { &*stack_frame };
    kprintln!("\nEXCEPTION: BREAKPOINT at {:#x}\n{:#?}", stack_frame.instruction_pointer, stack_frame);
}

extern "C" fn timer_interrupt_irq(stack_frame: &ExceptionStackFrame) {
    kprint!(".");
    unsafe { pic8259::PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

extern "C" fn cmos_interrupt_handler(stack_frame: &ExceptionStackFrame) {
    kprint!("c");
    unsafe { pic8259::PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

extern "C" fn general_protection_fault_irq(stack_frame: &ExceptionStackFrame) {
    let stack_frame = unsafe { &*stack_frame };
    kprintln!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}", stack_frame.instruction_pointer, stack_frame);
    loop {}
}

extern "C" fn debug_irq(stack_frame: &ExceptionStackFrame) {
    let stack_frame = unsafe { &*stack_frame };
    kprintln!("\nEXCEPTION: DEBUG at {:#x}\n{:#?}", stack_frame.instruction_pointer, stack_frame);
    loop {}
}

extern "C" fn overflow_irq_handler(stack_frame: &ExceptionStackFrame) {
    let stack_frame = unsafe { &*stack_frame };
    kprintln!("\nEXCEPTION: OVERFLOW at {:#x}\n{:#?}", stack_frame.instruction_pointer, stack_frame);
    loop {}
}
non_maskable_irq
bound_range_exceeded_irq
invalid_tss_irq
segment_not_present_irq
stack_segment_fault_irq
general_protection_fault_irq


macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() {
            unsafe {
                asm!("mov rdi, rsp
                      sub rsp, 8 // align the stack pointer
                      call $0"
                      :: "i"($name as extern "C" fn(&ExceptionStackFrame))
                      : "rdi" : "intel");

                 // undo stack pointer alignment
                asm!("add rsp, 8
                      iretq"
                      :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}


macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() {
            unsafe {
                asm!("pop rsi // pop error code into rsi
                      mov rdi, rsp
                      sub rsp, 8 // align the stack pointer
                      call $0"
                      :: "i"($name as extern "C" fn(&ExceptionStackFrame, u64))
                      : "rdi","rsi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}


const TIMER_INTERRUPT_ID: u8 = 32;


lazy_static! {
    static ref GLOBAL_IDT: idt::IDT = {
        let mut idt = idt::IDT::new();

        idt.set_handler(0, handler!(divide_by_zero_irq));
        idt.set_handler(1, handler!(debug_irq));
        idt.set_handler(2, handler!(non_maskable_irq));
        idt.set_handler(3, handler!(breakpoint_handler));
        idt.set_handler(4, handler!(overflow_irq_handler));
        idt.set_handler(5, handler!(bound_range_exceeded_irq));
        idt.set_handler(6, handler!(invalid_opcode_irq));
        idt.set_handler(7, handler!(device_not_available_irq));
        idt.set_handler(8, handler!(double_fault_irq));
        idt.set_handler(10, handler!(invalid_tss_irq));
        idt.set_handler(11, handler!(segment_not_present_irq));
        idt.set_handler(12, handler!(stack_segment_fault_irq));
        idt.set_handler(13, handler!(general_protection_fault_irq));
        idt.set_handler(14, handler_with_error_code!(page_fault_irq));
        idt.set_handler(16, handle!(page_fault_irq));

        // PIC
        idt.set_handler(TIMER_INTERRUPT_ID, handler!(timer_interrupt_irq));

        idt
    };
}
