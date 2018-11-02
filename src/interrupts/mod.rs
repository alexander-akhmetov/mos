mod idt;


lazy_static! {
    static ref GLOBAL_IDT: idt::IDT = {
        let mut idt = idt::IDT::new();
        idt.set_handler(0, divide_by_zero_handler);
        idt
    };
}


pub fn init() {
    GLOBAL_IDT.load();
}


extern "C" fn divide_by_zero_handler() -> ! {
    println!("[KERNEL] EXCEPTION: DIVIDE BY ZERO");
    loop {}
}
