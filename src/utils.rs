use sys::collections::linked_list::LinkedList;

pub fn divide_by_zero() {
    /// to raise divide by zero exception interrupt
    unsafe { asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel") }
}

pub unsafe fn exit_qemu() {
    /// writes 0 to port 0xf4 to exit from QEMU
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}


pub fn allocator_test() {
    // I could use
    //      alloc::collections::linked_list::LinkedList here
    // but wanted my own linked list
    let mut ll: LinkedList<u8> = LinkedList::new();
    let count = 1;
    for i in 0..count {
        ll.push_front(i);
    };
    kprintln!(">>> created linked list with {} elements: {:#?}", count, ll);
}
