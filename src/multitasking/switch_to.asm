global switch_to
global isr_return

section .text
bits 64
; fn switch_to(old: *mut *mut Context, new: *mut Context)
; old = rsp
; new = rsi
switch_to:
    pushfq			; push regs to current ctx
    push rax
    push rcx
    push rdx
    push rbp
    push rbx
    push r12
    push r13
    push r14
    push r15

    mov [rdi], rsp	; update old ctx ptr with current stack ptr
    mov rsp, rsi	; switch to new stack

    pop r15
    pop r14
    pop r13
    pop r12
    pop rbx
    pop rbp
    pop rdx
    pop rcx
    pop rax
    popfq

    ret

isr_return:
    iretq
