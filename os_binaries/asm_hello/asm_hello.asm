global _start

_start:
    mov rax, 0xD
    int 0x80

    mov rax, 0x3C
    int 0x80

    ret
