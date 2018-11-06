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
        "       multibooot: [0x{:x} - 0x{:x}]    bytes: {}",
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

    // let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    // let mut frame_allocator = memory::SimpleFrameAllocator::new(
    //     kernel_start as usize,
    //     kernel_end as usize,
    //     multiboot_start,
    //     multiboot_end,
    //     memory_map_tag.memory_areas(),
    // );
    // for i in 0.. {
    //     if let None = frame_allocator.allocate_frame() {
    //         system_log!("allocated {} frames", i);
    //         break;
    //     }
    // }
}
