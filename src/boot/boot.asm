global start

section .text
bits 32
start:

    ; before enabling paging we have to create page table
    ; after, to enable paging we have to do 4 steps:
    ;   - 1) We have to put the address of the p4_table to a special register
    ;   - 2) Enable PAE (physical address extension)
    ;   - 3) Set the "long mode bit"
    ;   - 4) Enable paging

    ;;; page table creation

    ; p3 table
    mov eax, p3_table
    or eax, 0b11  ; ‘present’ and ‘writable’ bits
    mov dword [p4_table], eax

    ; p2 table
    mov eax, p2_table
    or eax, 0b11
    mov dword [p3_table], eax

    ; point each p2_table entry to a page
    ; and the loop begins
    mov ecx, 0 ; counter
.map_p2_table:
    mov eax, 0x200000 ; 2MB
    mul ecx  ; mul multiplies ecx by eax (always by eax), store result in eax
    or eax, 0b10000011 ; ‘huge page’, ‘present’ and ‘writable’ bits
    mov [p2_table + ecx * 8], eax
    ; inc our counter: ecx
    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ; enable paging, 4 steps begin here

    ; move page table address to cr3
    mov eax, p4_table
    mov cr3, eax  ; we can't mov directly into cr3, we must move data from another register, so let's do it via eax

    ; enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set long mode bit
    mov ecx, 0xC0000080
    rdmsr  ; read model specific register
    or eax, 1 << 8
    wrmsr  ; write model specific register

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31
    or eax, 1 << 16
    mov cr0, eax

    ;;; print hello world
    mov word [0xb8000], 0x0248 ; H
    mov word [0xb8002], 0x0265 ; e
    mov word [0xb8004], 0x026c ; l
    mov word [0xb8006], 0x026c ; l
    mov word [0xb8008], 0x026f ; o
    mov word [0xb800a], 0x022c ; ,
    mov word [0xb800c], 0x0220 ;
    mov word [0xb800e], 0x0277 ; w
    mov word [0xb8010], 0x026f ; o
    mov word [0xb8012], 0x0272 ; r
    mov word [0xb8014], 0x026c ; l
    mov word [0xb8016], 0x0264 ; d
    mov word [0xb8018], 0x0221 ; !
    ;;;
    ;;; end program
    hlt


; let's setup paging
section .bss  ; section "block started by symbol"

align 4096

p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
