use memory;
use memory::FrameAllocator;
use multiboot2::BootInformation;
use x86;

pub fn get_multiboot_info(multiboot_information_address: usize) -> &'static BootInformation {
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    boot_info
}

pub fn print_multiboot_info(multiboot_information_address: usize) {
    /// prints multiboot2 information parsed from memory by multiboot_information_address
    let boot_info = get_multiboot_info(multiboot_information_address);

    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag
        .sections()
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();
    system_log!(
        "       kernel:     [0x{:x} - 0x{:x}]    bytes: {}",
        kernel_start,
        kernel_end,
        kernel_end - kernel_start
    );

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    system_log!(
        "       multiboot:  [0x{:x} - 0x{:x}]    bytes: {}",
        multiboot_start,
        multiboot_end,
        multiboot_end - multiboot_start
    );
    for module in boot_info.module_tags() {
        system_log!(
            "       module:     [0x{:x} - 0x{:x}]    name: {}",
            module.start_address(),
            module.end_address(),
            module.name(),
        );
        // let ptr = module.start_address() as *const func;
        // unsafe { (*ptr)() };
        // unsafe { x86::jmp(module.start_address() as u64) };
    }

    // let (initrd_start, initrd_end) = get_module(boot_info, "initrd");
    // let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    // let frame_allocator = memory::SimpleFrameAllocator::new(
    //     kernel_start as usize,
    //     kernel_end as usize,
    //     multiboot_start,
    //     multiboot_end,
    //     initrd_start as usize,
    //     initrd_end as usize,
    //     memory_map_tag.memory_areas(),
    // );
    // for i in 0..2 {
    //     if let None = frame_allocator.allocate_frame() {
    //         system_log!("allocated {} frames", i);
    //         break;
    //     }
    // }
}

pub fn get_module(boot_info: &BootInformation, name: &str) -> (u64, u64) {
    let module = boot_info
        .module_tags()
        .find(|m| m.name() == name)
        .expect("Can't find module");

    (module.start_address() as u64, module.end_address() as u64)
}
