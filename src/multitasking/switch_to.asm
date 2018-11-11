global switch_to
global switch_first_time

section .text
bits 64
; fn switch_to(old: *mut *mut Context, new: *mut Context)
; old = RDI
; new = RSI
switch_to:
    push rbp

    sub  rsp, 8  ; ignore rsp

    push r15
    push r14
    push r13
    push r12
    push rbx
    pushfq

    mov [rdi], rsp	; update old ctx ptr with current stack ptr
    mov rsp, rsi	; switch to new stack

    popfq
    pop rbx
    pop r12
    pop r13
    pop r14
    pop r15
    add rsp, 8
    pop rbp

    ret
