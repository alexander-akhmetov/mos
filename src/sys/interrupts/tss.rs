pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

/*NOT USED YET*/

#[derive(Debug)]
#[repr(C)]
pub struct TaskStateSegment {
    _reserved1: u32,
    _privilege_stack_table: [u64; 3],
    pub interrupt_stack_table: [u64; 7],
    _reserved2: u64,
    _reserved3: u16,
    _io_map_addr: u16,
}

impl TaskStateSegment {
    const fn new() -> TaskStateSegment {
        TaskStateSegment {
            _reserved1: 0,
            _privilege_stack_table: [0; 3],
            interrupt_stack_table: [0; 7],
            _reserved2: 0,
            _reserved3: 0,
            _io_map_addr: 0,
        }
    }
}

// lazy_static! {
//     static ref TSS: TaskStateSegment = {
//         let mut tss = TaskStateSegment::new();
//     };
// }
