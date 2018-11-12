global switch_to
global start_task

section .text
bits 64
; fn switch_to(old: *mut *mut Context, new: *mut Context)
; old = RDI
; new = RSI
switch_to:
    push rbp
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
    pop rbp

    ret


start_task:
    ; fn start_task(ctx: *mut Context)
    ; task = RDI
    mov rsp, rdi	; switch to new stack

    popfq
    pop rbx
    pop r12
    pop r13
    pop r14
    pop r15
    pop rbp

    ret
