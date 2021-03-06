// Interrupts Descriptor Table realization
use x86_64::instructions::segmentation;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;

const IDT_SIZE: usize = 256;
#[allow(dead_code)]
extern "C" {
    /// The offset of the main code segment in our GDT.  Exported by our
    /// assembly code.
    static gdt64_code_offset: u16;
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn minimal() -> Self {
        let mut options = 0;
        options |= 0b111 << 9; // "must be one" bits: 0b0000_0111_0000_0000
        EntryOptions(options)
    }

    fn new() -> Self {
        let mut options = Self::minimal();
        options.set_present(true).disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        // When set, the page fault was caused by a page-protection
        // violation. When not set, it was caused by a non-present page
        if present {
            self.0 |= 1 << 15;
        } else {
            self.0 &= 0 << 15;
        }
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        if disable {
            self.0 |= 0 << 8;
        } else {
            self.0 &= 1 << 8;
        }
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0 = ((self.0 >> 15) << 15) | dpl;
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0 = ((self.0 >> 3) << 3) | index;
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler_fn_pointer: u64) -> Self {
        Entry {
            gdt_selector,
            pointer_low: handler_fn_pointer as u16,
            pointer_middle: (handler_fn_pointer >> 16) as u16,
            pointer_high: (handler_fn_pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
        }
    }

    fn missing() -> Self {
        Entry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
        }
    }
}

pub struct IDT([Entry; IDT_SIZE]);

impl IDT {
    pub fn new() -> IDT {
        IDT([Entry::missing(); IDT_SIZE])
    }

    pub fn set_handler(&mut self, entry: u8, handler_pointer: u64) {
        self.0[entry as usize] = Entry::new(segmentation::cs(), handler_pointer);
    }

    pub fn load(&'static self) {
        use core::mem::size_of;
        use x86_64::instructions::tables::{lidt, DescriptorTablePointer};

        let ptr = DescriptorTablePointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) };
    }
}
