global switch_to
global start_task

section .text
bits 64
; fn switch_to(old_rsp: u64, new_rsp: u64)
; old = RDI
; new = RSI
switch_to:
    mov rax, cr3  ; can't push cr3 directly
    push rax
    push r15
    push r14
    push r13
    push r12
    push rbx
    pushfq
    push rbp

    mov [rdi], rsp	; update old rsp to the current rsp
    mov rsp, rsi	; switch to new stack

    pop rbp
    popfq
    pop rbx
    pop r12
    pop r13
    pop r14
    pop r15

    pop rax  ; we can't pop directly into cr3
    mov cr3, rax

    sti

    ret


start_task:
    ; fn start_task(rsp: u64)
    ; task = RDI
    mov rsp, rdi	; switch to new stack

    pop rbp
    popfq
    pop rbx
    pop r12
    pop r13
    pop r14
    pop r15

    pop rax  ; we can't pop directly into cr3
    mov cr3, rax

    sti

    ret
