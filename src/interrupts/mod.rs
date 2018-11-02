#[macro_use]
mod idt;


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


extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\n[KERNEL] EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}


pub extern "C" fn double_fault_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\n[KERNEL] EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: &ExceptionStackFrame) -> ! {
    let stack_frame = unsafe { &*stack_frame };
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}", stack_frame.instruction_pointer, stack_frame);
    loop {}
}

extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame, error_code: u64) -> ! {
    println!(
        "\nEXCEPTION: PAGE FAULT with error code {:?}\n{:#?}",
        error_code, unsafe { &*stack_frame });
    loop {}
}


macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("mov rdi, rsp
                      sub rsp, 8 // align the stack pointer
                      call $0"
                      :: "i"($name as extern "C" fn(&ExceptionStackFrame) -> !)
                      : "rdi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}


macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!("pop rsi // pop error code into rsi
                      mov rdi, rsp
                      sub rsp, 8 // align the stack pointer
                      call $0"
                      :: "i"($name as extern "C" fn(
                          &ExceptionStackFrame, u64) -> !)
                      : "rdi","rsi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}


lazy_static! {
    static ref GLOBAL_IDT: idt::IDT = {
        let mut idt = idt::IDT::new();

        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_handler));

        idt
    };
}
