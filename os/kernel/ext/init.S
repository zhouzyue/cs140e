#define EL0 0b00
#define EL1 0b01
#define EL2 0b10
#define EL3 0b11

.section .text.init

.global _start
_start:
    // read cpu affinity, start core 0, halt rest
    mrs     x1, MPIDR_EL1
    and     x1, x1, #3
    cbz     x1, setup

halt:
    // core affinity != 0, halt it
    wfe
    b       halt

setup:
    // store the desired EL1 stack pointer in x1
    adr     x1, _start

    // read the current exception level into x0 (ref: C5.2.1)
    mrs     x0, CurrentEL
    and     x0, x0, #0b1100
    lsr     x0, x0, #2

switch_to_el2:
    // switch to EL2 if we're in EL3. otherwise switch to EL1
    cmp     x0, EL3
    bne     switch_to_el1

    // set-up SCR_EL3 (bits 0, 4, 5, 7, 8, 10) (A53: 4.3.42)
    mov     x2, #0x5b1
    msr     SCR_EL3, x2

    // set-up SPSR and PL switch! (bits 0, 3, 6, 7, 8, 9) (ref: C5.2.20)
    mov     x2, #0x3c9
    msr     SPSR_EL3, x2
    adr     x2, switch_to_el1
    msr     ELR_EL3, x2
    eret

switch_to_el1:
    // switch to EL1 if we're not already in EL1. otherwise continue with start
    cmp     x0, EL1
    beq     set_stack

    // set the stack-pointer for EL1
    msr     SP_EL1, x1

    // enable CNTP for EL1/EL0 (ref: D7.5.2, D7.5.13)
    // NOTE: This doesn't actually enable the counter stream.
    mrs     x0, CNTHCTL_EL2
    orr     x0, x0, #0b11
    msr     CNTHCTL_EL2, x0
    msr     CNTVOFF_EL2, xzr

    // enable AArch64 in EL1 (A53: 4.3.36)
    mov     x0, #(1 << 31)      // Enable AArch64 for EL1
    orr     x0, x0, #(1 << 1)   // RES1 on A-53
    msr     HCR_EL2, x0
    mrs     x0, HCR_EL2

    // enable floating point and SVE (SIMD) (A53: 4.3.38, 4.3.34)
    msr     CPTR_EL2, xzr     // don't trap accessing SVE registers
    mrs     x0, CPACR_EL1
    orr     x0, x0, #(0b11 << 20)
    msr     CPACR_EL1, x0

    // Set SCTLR to known state (RES1: 11, 20, 22, 23, 28, 29) (A53: 4.3.30)
    mov     x2, #0x0800
    movk    x2, #0x30d0, lsl #16
    msr     SCTLR_EL1, x2

    // set up exception handlers
    // FIXME: load `_vectors` addr into appropriate register (guide: 10.4)
    adr     x2, _vectors
    msr     VBAR_EL1, x2

    // change execution level to EL1 (ref: C5.2.19)
    mov     x2, #0x3c5
    msr     SPSR_EL2, x2

    adr     x2, set_stack
    msr     ELR_EL2, x2
    eret

set_stack:
    // set the current stack pointer
    mov     sp, x1

zero_bss:
    // load the start address and number of bytes in BSS section
    ldr     x1, =__bss_start
    ldr     x2, =__bss_length

zero_bss_loop:
    // zero out the BSS section, 64-bits at a time
    cbz     x2, go_kmain
    str     xzr, [x1], #8
    sub     x2, x2, #8
    cbnz    x2, zero_bss_loop

go_kmain:
    // jump to kmain, which shouldn't return. halt if it does
    bl      kmain
    b       halt

context_save:
    //   |------| <x (original SP)
    //   |  x0  |
    //   |------|
    //   |  lr  |
    //   |------| <- SP
    sub SP, SP, #240
    stp x28, x29, [SP, #224]
    stp x26, x27, [SP, #208]
    stp x24, x25, [SP, #192]
    stp x22, x23, [SP, #176]
    stp x20, x21, [SP, #160]
    stp x18, x19, [SP, #144]
    stp x16, x17, [SP, #128]
    stp x14, x15, [SP, #112]
    stp x12, x13, [SP, #96]
    stp x10, x11, [SP, #80]
    stp x8, x9, [SP, #64]
    stp x6, x7, [SP, #48]
    stp x4, x5, [SP, #32]
    stp x2, x3, [SP, #16]
    str x1, [SP, #8]

    sub SP, SP, #512
    stp q30, q31, [SP, #480]
    stp q28, q29, [SP, #448]
    stp q26, q27, [SP, #416]
    stp q24, q25, [SP, #384]
    stp q22, q23, [SP, #352]
    stp q20, q21, [SP, #320]
    stp q18, q19, [SP, #288]
    stp q16, q17, [SP, #256]
    stp q14, q15, [SP, #224]
    stp q12, q13, [SP, #192]
    stp q10, q11, [SP, #160]
    stp q8, q9, [SP, #128]
    stp q6, q7, [SP, #96]
    stp q4, q5, [SP, #64]
    stp q2, q3, [SP, #32]
    stp q0, q1, [SP]

    mrs x4, TPIDR_EL0
    mrs x3, SP_EL0
    mrs x2, SPSR_EL1
    mrs x1, ELR_EL1

    sub SP, SP, #32
    stp x3, x4, [SP, #16]
    stp x1, x2, [SP]

    // handle_exception(info, esr, tf)
    //                    x0,  x1, x2
    mov x2, SP
    mrs x1, ESR_EL1
    str lr, [SP, #-16]!
    bl handle_exception
    ldr lr, [SP], #16

.global context_restore
context_restore:
    ldp x1, x2, [SP]
    ldp x3, x4, [SP, #16]
    add SP, SP, #32

    msr TPIDR_EL0, x4
    msr SP_EL0, x3
    msr SPSR_EL1, x2
    msr ELR_EL1, x1

    ldp q30, q31, [SP, #480]
    ldp q28, q29, [SP, #448]
    ldp q26, q27, [SP, #416]
    ldp q24, q25, [SP, #384]
    ldp q22, q23, [SP, #352]
    ldp q20, q21, [SP, #320]
    ldp q18, q19, [SP, #288]
    ldp q16, q17, [SP, #256]
    ldp q14, q15, [SP, #224]
    ldp q12, q13, [SP, #192]
    ldp q10, q11, [SP, #160]
    ldp q8, q9, [SP, #128]
    ldp q6, q7, [SP, #96]
    ldp q4, q5, [SP, #64]
    ldp q2, q3, [SP, #32]
    ldp q0, q1, [SP]
    add SP, SP, #512

    ldp x28, x29, [SP, #224]
    ldp x26, x27, [SP, #208]
    ldp x24, x25, [SP, #192]
    ldp x22, x23, [SP, #176]
    ldp x20, x21, [SP, #160]
    ldp x18, x19, [SP, #144]
    ldp x16, x17, [SP, #128]
    ldp x14, x15, [SP, #112]
    ldp x12, x13, [SP, #96]
    ldp x10, x11, [SP, #80]
    ldp x8, x9, [SP, #64]
    ldp x6, x7, [SP, #48]
    ldp x4, x5, [SP, #32]
    ldp x2, x3, [SP, #16]
    ldr x1, [SP, #8]
    add SP, SP, #240

    ret

#define HANDLER(source, kind) \
    .align 7; \
    stp     lr, x0, [SP, #-16]!; \
    mov     x0, ##source; \
    movk    x0, ##kind, LSL #16; \
    bl      context_save; \
    ldp     lr, x0, [SP], #16; \
    eret

#define HANDLER_SOURCE(source) \
    HANDLER(source, 0); \
    HANDLER(source, 1); \
    HANDLER(source, 2); \
    HANDLER(source, 3) \

.align 11
_vectors:
    HANDLER_SOURCE(0)
    HANDLER_SOURCE(1)
    HANDLER_SOURCE(2)
    HANDLER_SOURCE(3)
