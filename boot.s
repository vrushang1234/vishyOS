.global _start
.extern rust_main

.section .text
_start:
    cli

    lea stack_top(%rip), %rsp
    xor %rbp, %rbp

    call rust_main

.hang:
    hlt
    jmp .hang

.section .bss
.align 16
stack_bottom:
    .skip 16384
stack_top:

