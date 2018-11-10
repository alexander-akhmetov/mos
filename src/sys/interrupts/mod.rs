#[macro_use]
mod idt;

pub mod tss;

use drivers::keyboard;
use pic8259;
use sys;

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
    system_log!(
        "EXCEPTION: {}: {:#x}\n{:#?}",
        name,
        stack_frame.instruction_pointer,
        &*stack_frame,
    );
}

extern "x86-interrupt" fn divide_by_zero_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Divide by zero", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn double_fault_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Double fault", stack_frame);
    loop {}
}

extern "x86-interrupt" fn invalid_opcode_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Invalid Opcode", stack_frame);
    loop {}
}

extern "x86-interrupt" fn page_fault_irq(stack_frame: &ExceptionStackFrame, error_code: u64) {
    kprintln!(
        "\nEXCEPTION: PAGE FAULT with error code 0b{:b}\n{:#?}",
        error_code,
        &*stack_frame,
    );
    loop {}
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Breakpoint", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_irq(_stack_frame: &ExceptionStackFrame) {
    // kprint!(".");
    unsafe {
        pic8259::PICS
            .lock()
            .notify_end_of_interrupt(TIMER_INTERRUPT_ID);
    }
}

extern "x86-interrupt" fn keyboard_irq(_stack_frame: &ExceptionStackFrame) {
    let character = keyboard::read_character();
    if let Some(character) = character {
        kprint!("{}", character);
    }
    unsafe {
        pic8259::PICS
            .lock()
            .notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID);
    }
}

extern "x86-interrupt" fn general_protection_fault_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("General protection fault", stack_frame);
    loop {}
}

extern "x86-interrupt" fn debug_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Debug", stack_frame);
    loop {}
}

extern "x86-interrupt" fn overflow_irq_handler(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Overflow", stack_frame);
    loop {}
}

extern "x86-interrupt" fn non_maskable_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Non Maskable", stack_frame);
    loop {}
}

extern "x86-interrupt" fn bound_range_exceeded_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Bound Range Exceeded", stack_frame);
    loop {}
}

extern "x86-interrupt" fn invalid_tss_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Invalid TSS", stack_frame);
    loop {}
}

extern "x86-interrupt" fn segment_not_present_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Segment Not Present", stack_frame);
    loop {}
}

extern "x86-interrupt" fn stack_segment_fault_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Stack-Segment Fault", stack_frame);
    loop {}
}

extern "x86-interrupt" fn device_not_available_irq(stack_frame: &ExceptionStackFrame) {
    kprintln_exception("Device Not Available", stack_frame);
    loop {}
}

macro_rules! default_handler {
    ($name: expr) => {{
        extern "x86-interrupt" fn hndlr(stack_frame: &ExceptionStackFrame) {
            kprintln_exception($name, stack_frame);
            loop {}
        }
        hndlr
    }};
}

extern "C" fn system_call(syscall_args: &sys::SyscallArgs) -> u64 {
    sys::SYSCALL_DISPATCHER
        .lock()
        .process_system_call(syscall_args)
}

macro_rules! save_registers {
    () => {
        asm!("  push rax
                push rbx
                push rcx
                push rdx
                push rsi
                push rdi

                mov rdi, rsp
        " :::: "intel", "volatile");
    }
}

macro_rules! restore_registers {
    () => {
        asm!("  add rsp, 4
                pop rdi
                pop rsi
                pop rdx
                pop rcx
                pop rbx
                add rsp, 4
            " :::: "intel", "volatile");
    }
}

macro_rules! system_call_handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() {
            unsafe {
                // https://en.wikibooks.org/wiki/X86_Assembly/Interfacing_with_Linux
                // change to rax
                save_registers!();
                asm!("call $0"
                      :: "i"($name as extern "C" fn(syscall_args: &sys::SyscallArgs) -> u64)
                      : "rax" : "intel",
                );
                restore_registers!();
                asm!("iretq" :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

const TIMER_INTERRUPT_ID: u8 = pic8259::PIC_1_OFFSET;
const KEYBOARD_INTERRUPT_ID: u8 = pic8259::PIC_1_OFFSET + 1;

lazy_static! {
    static ref GLOBAL_IDT: idt::IDT = {
        let mut idt = idt::IDT::new();

        idt.set_handler(0, divide_by_zero_irq as u64);
        idt.set_handler(1, debug_irq as u64);
        idt.set_handler(2, non_maskable_irq as u64);
        idt.set_handler(3, breakpoint_handler as u64);
        idt.set_handler(4, overflow_irq_handler as u64);
        idt.set_handler(5, bound_range_exceeded_irq as u64);
        idt.set_handler(6, invalid_opcode_irq as u64);
        idt.set_handler(7, device_not_available_irq as u64);
        idt.set_handler(8, double_fault_irq as u64);
        idt.set_handler(10, invalid_tss_irq as u64);
        idt.set_handler(11, segment_not_present_irq as u64);
        idt.set_handler(12, stack_segment_fault_irq as u64);
        idt.set_handler(13, general_protection_fault_irq as u64);
        idt.set_handler(14, page_fault_irq as u64);
        idt.set_handler(15, default_handler!("Exception 15") as u64);
        idt.set_handler(16, default_handler!("Floating point") as u64);
        idt.set_handler(17, default_handler!("Alignment Check") as u64);
        idt.set_handler(18, default_handler!("Machine Check") as u64);
        idt.set_handler(19, default_handler!("SIMD Floating-Point Exception")as u64);
        idt.set_handler(20, default_handler!("Virtualization Exception") as u64);
        idt.set_handler(30, default_handler!("Security Exception") as u64);

        // PIC
        idt.set_handler(TIMER_INTERRUPT_ID, timer_interrupt_irq as u64);
        idt.set_handler(KEYBOARD_INTERRUPT_ID, keyboard_irq as u64);

        // API
        idt.set_handler(128, system_call_handler!(system_call) as u64);

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

pub fn init() {
    GLOBAL_IDT.load();
}
