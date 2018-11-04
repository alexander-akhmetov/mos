pub unsafe fn read_eax() -> u32 {
    let result: u32;
    asm!("mov %eax, $0" : "=r" (result));
    result
}

pub unsafe fn make_system_call() {
    kprintln!("--- before system call ---");
    asm!("mov eax, 0x5; int 0x80;" ::: "eax" : "volatile", "intel");
    kprintln!("--- after system call ---");
}
