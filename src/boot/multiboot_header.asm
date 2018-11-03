section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number (from multiboot 2 specification)
    dd 0                         ; enable protected mode 0
    dd header_end - header_start ; header length
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; multiboot end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:
