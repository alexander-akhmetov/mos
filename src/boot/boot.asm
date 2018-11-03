global start

section .text
bits 32
start:
    mov esp, stack_top
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

    ; GDT
    lgdt [gdt64.pointer]

    ; finish jumping to long mode
    ; update selectors
    mov ax, gdt64.data  ; ax - segment register, 16 bit version of eax
    mov ss, ax  ; stack segment
    mov ds, ax  ; data segment
    mov es, ax  ; extra segment

    ; jump to long mode!
    jmp gdt64.code:long_mode_start


; let's setup paging
section .bss  ; section "block started by symbol"
align 4096

p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096

section .bss
stack_bottom:
    resb 128
stack_top:



;;; Global descriptor table (GDT)
section .rodata
gdt64:
    dq 0  ; first entry in the GDT is special: it needs to be a zero value
.code: equ $ - gdt64  ; set the ."code" label's value to the current address minus the address of gdt64
    ; set 44, 47, 41, 43, 53 bits to 1
    ;   44: 'descriptor type': This has to be 1 for code and data segments
    ;   47: 'present': This is set to 1 if the entry is valid
    ;   41: 'read/write': If this is a code segment, 1 means that it’s readable
    ;   43: 'executable': Set to 1 for code segments
    ;   53: '64-bit': this is a 64-bit GDT
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.data: equ $ - gdt64
    ; for data segments 41=1 == writable
    dq (1<<44) | (1<<47) | (1<<41)
.pointer:
    dw .pointer - gdt64 - 1
    dq gdt64


section .text
bits 64
long_mode_start:
    ; call the rust main
    extern main
    call main
