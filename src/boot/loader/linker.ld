ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        KEEP(*(.multiboot_header))
        . = ALIGN(4K);
    }

    .text :
    {
        *(.text .text.*)
        . = ALIGN(4K);
    }

    .rodata : {
        *(.rodata .rodata.*)
        . = ALIGN(4K);
    }

    .data.rel.ro : {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
        . = ALIGN(4K);
    }
}
