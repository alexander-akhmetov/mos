global _start

_start:
    mov rdi, 0xD  ; why rdi, not rax?
    int 0x80

    mov rdi, 0x3C
    int 0x80

    ret
