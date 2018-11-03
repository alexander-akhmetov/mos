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

fn kprintln_exception(name: &str, stack_frame: &ExceptionStackFrame) {
    let stack_frame = unsafe { &*stack_frame };
    kprintln!("\n[KERNEL] EXCEPTION: {}: {:#x}\n{:#?}", name, stack_frame.instruction_pointer, stack_frame);
}

extern "C" fn divide_by_zero_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Divide by zero", stack_frame);
    loop {}
}


pub extern "C" fn double_fault_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Double fault", stack_frame);
    loop {}
}

extern "C" fn invalid_opcode_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Invalid Opcode", stack_frame);
    loop {}
}

extern "C" fn page_fault_irq(stack_frame: &ExceptionStackFrame, error_code: u64) {
    kprintln!(
        "\nEXCEPTION: PAGE FAULT with error code {:?}\n{:#?}",
        error_code, unsafe { &*stack_frame });
    loop {}
}

extern "C" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Breakpoint", stack_frame);
}

extern "C" fn timer_interrupt_irq(stack_frame: &ExceptionStackFrame) {
    kprint!(".");
    unsafe { pic8259::PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

extern "C" fn general_protection_fault_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("General protection fault", stack_frame);
    loop {}
}

extern "C" fn debug_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Debug", stack_frame);
    loop {}
}

extern "C" fn overflow_irq_handler(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Overflow", stack_frame);
    loop {}
}

extern "C" fn non_maskable_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Non Maskable", stack_frame);
    loop {}
}

extern "C" fn bound_range_exceeded_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Bound Range Exceeded", stack_frame);
    loop {}
}

extern "C" fn invalid_tss_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Invalid TSS", stack_frame);
    loop {}
}

extern "C" fn segment_not_present_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Segment Not Present", stack_frame);
    loop {}
}

extern "C" fn stack_segment_fault_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Stack-Segment Fault", stack_frame);
    loop {}
}

extern "C" fn device_not_available_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Device Not Available", stack_frame);
    loop {}
}


macro_rules! default_handler {
    ($name: expr) => {{

        extern "C" fn hndlr(stack_frame: &ExceptionStackFrame) {
            let stack_frame = unsafe { &*stack_frame };
            kprintln_exception($name, stack_frame);
            loop {}
        }

        extern "C" fn wrapper() {
            unsafe {
                asm!("mov rdi, rsp
                      sub rsp, 8 // align the stack pointer
                      call $0"
                      :: "i"(hndlr as extern "C" fn(&ExceptionStackFrame))
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
        idt.set_handler(9, default_handler!("Exception 9"));
        idt.set_handler(10, handler!(invalid_tss_irq));
        idt.set_handler(11, handler!(segment_not_present_irq));
        idt.set_handler(12, handler!(stack_segment_fault_irq));
        idt.set_handler(13, handler!(general_protection_fault_irq));
        idt.set_handler(14, handler_with_error_code!(page_fault_irq));
        idt.set_handler(15, default_handler!("Exception 15"));
        idt.set_handler(16, default_handler!("Floating point"));
        idt.set_handler(17, default_handler!("Alignment Check"));
        idt.set_handler(18, default_handler!("Machine Check"));
        idt.set_handler(19, default_handler!("SIMD Floating-Point Exception"));
        idt.set_handler(20, default_handler!("Virtualization Exception"));
        idt.set_handler(21, default_handler!("Exception 21"));
        idt.set_handler(22, default_handler!("Exception 22"));
        idt.set_handler(23, default_handler!("Exception 23"));
        idt.set_handler(24, default_handler!("Exception 24"));
        idt.set_handler(25, default_handler!("Exception 25"));
        idt.set_handler(26, default_handler!("Exception 26"));
        idt.set_handler(27, default_handler!("Exception 27"));
        idt.set_handler(28, default_handler!("Exception 28"));
        idt.set_handler(29, default_handler!("Exception 29"));
        idt.set_handler(30, default_handler!("Security Exception"));

        // PIC
        idt.set_handler(TIMER_INTERRUPT_ID, handler!(timer_interrupt_irq));
        idt.set_handler(31, default_handler!("Exception 31"));
        idt.set_handler(32, default_handler!("Exception 32"));
        idt.set_handler(33, default_handler!("Exception 33"));
        idt.set_handler(34, default_handler!("Exception 34"));
        idt.set_handler(35, default_handler!("Exception 35"));
        idt.set_handler(36, default_handler!("Exception 36"));
        idt.set_handler(37, default_handler!("Exception 37"));
        idt.set_handler(38, default_handler!("Exception 38"));
        idt.set_handler(39, default_handler!("Exception 39"));
        idt.set_handler(40, default_handler!("Exception 40"));
        idt.set_handler(41, default_handler!("Exception 41"));
        idt.set_handler(42, default_handler!("Exception 42"));
        idt.set_handler(43, default_handler!("Exception 43"));
        idt.set_handler(44, default_handler!("Exception 44"));
        idt.set_handler(45, default_handler!("Exception 45"));
        idt.set_handler(46, default_handler!("Exception 46"));
        idt.set_handler(47, default_handler!("Exception 47"));

        idt
    };
}


pub fn enable() {
    unsafe {
        asm!("sti");
    }
}


// breakpoint exception
pub fn int3() {
    unsafe {
        asm!("int3");
    }
}
