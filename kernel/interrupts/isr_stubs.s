////////// ISR Stubs //////////
// These stubs define the behavior of interrupts
// If the CPU exception did not push an error code, push 0
// Sequence: Normalize stack -> Push interrupt number -> Save current state of registers -> Handle the interrupt -> Restore all the registers -> Return

.macro ISR_NOERR num
.global isr_stub_\num
isr_stub_\num:
    push 0
    push \num
    jmp isr_common
.endm

.macro ISR_ERR num
.global isr_stub_\num
isr_stub_\num:
    push \num
    jmp isr_common
.endm

// CPU exceptions 0-31
ISR_NOERR 0
ISR_NOERR 1
ISR_NOERR 2
ISR_NOERR 3
ISR_NOERR 4
ISR_NOERR 5
ISR_NOERR 6
ISR_NOERR 7
ISR_ERR   8   /* #DF double fault */
ISR_NOERR 9
ISR_ERR   10  // TS invalid TSS 
ISR_ERR   11  // NP segment not present 
ISR_ERR   12  // SS stack faul 
ISR_ERR   13  // GP general protection fault 
ISR_ERR   14  // PF page fault 
ISR_NOERR 15
ISR_NOERR 16
ISR_ERR   17  // AC alignment check 
ISR_NOERR 18
ISR_NOERR 19
ISR_NOERR 20
ISR_ERR   21  // CP control protection 
ISR_NOERR 22
ISR_NOERR 23
ISR_NOERR 24
ISR_NOERR 25
ISR_NOERR 26
ISR_NOERR 27
ISR_NOERR 28
ISR_ERR   29  // VC VMM communication (AMD)
ISR_ERR   30  // SX security exception
ISR_NOERR 31

// Hardware IRQs: PIC remapped to vectors 32-47
ISR_NOERR 32  // IRQ0  timer
ISR_NOERR 33  // IRQ1  keyboard
ISR_NOERR 34
ISR_NOERR 35
ISR_NOERR 36
ISR_NOERR 37
ISR_NOERR 38
ISR_NOERR 39
ISR_NOERR 40
ISR_NOERR 41
ISR_NOERR 42
ISR_NOERR 43
ISR_NOERR 44
ISR_NOERR 45
ISR_NOERR 46
ISR_NOERR 47

// Common entry: save all GP registers, call Rust handler, restore, iretq
isr_common:
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rdi
    push rsi
    push rbp
    push rsp
    push rbx
    push rdx
    push rcx
    push rax

    mov rdi, rsp
    call interrupt_dispatch

    pop rax
    pop rcx
    pop rdx
    pop rbx
    add rsp, 8          // skip stale _rsp_unused
    pop rbp
    pop rsi
    pop rdi
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15

    add rsp, 16         // skip interrupt_number + error_code
    iretq
