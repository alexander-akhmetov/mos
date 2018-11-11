extern main

global long_mode_start

section .text
bits 64
long_mode_start:
    ; call the rust main
    call clear_data_segment_registers
    call main


clear_data_segment_registers:
    ; load 0 into all data segment registers
    xor rax, rax
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
