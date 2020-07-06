.macro SAVE reg, offset
    sd  \reg, \offset * 8(sp)
.endm

.macro LOAD reg, offset
    ld  \reg, \offset * 8(sp)
.endm
    .section .text
    .global __interrupt

__interrupt:
    # stack = stack - 34 * 8
    addi    sp, sp, -34 * 8
    # stack[1] = x1
    SAVE    x1, 1
    # x1 = stack + 34 * 8
    addi    x1, sp, 34 * 8
    # stack[2] = x1 = stack + 34 * 8 (original stack pointer)
    SAVE    x1, 2
    # stack[n] = xn, n = 3 ~ 31
    SAVE    x3, 3
    SAVE    x4, 4
    SAVE    x5, 5
    SAVE    x6, 6
    SAVE    x7, 7
    SAVE    x8, 8
    SAVE    x9, 9
    SAVE    x10, 10
    SAVE    x11, 11
    SAVE    x12, 12
    SAVE    x13, 13
    SAVE    x14, 14
    SAVE    x15, 15
    SAVE    x16, 16
    SAVE    x17, 17
    SAVE    x18, 18
    SAVE    x19, 19
    SAVE    x20, 20
    SAVE    x21, 21
    SAVE    x22, 22
    SAVE    x23, 23
    SAVE    x24, 24
    SAVE    x25, 25
    SAVE    x26, 26
    SAVE    x27, 27
    SAVE    x28, 28
    SAVE    x29, 29
    SAVE    x30, 30
    SAVE    x31, 31

    # stack[32] = sstatus
    csrr    s1, sstatus
    SAVE    s1, 32

    # stack[33] = sepc
    csrr    s2, sepc
    SAVE    s2, 33


    # handler(sp, scause, stval);
    mv      a0, sp
    csrr    a1, scause
    csrr    a2, stval
    jal     handle_interrupt


    .global __restore
__restore:
    # stack = &context
    mv      sp, a0
    # sstatus = stack[32]
    LOAD    s1, 32
    csrw    sstatus, s1
    # sepc = stack[33]
    LOAD    s2, 33
    csrw    sepc, s2

    # xn = stack[n], n != 2, n = 1 ~ 31
    LOAD    x1, 1
    LOAD    x3, 3
    LOAD    x4, 4
    LOAD    x5, 5
    LOAD    x6, 6
    LOAD    x7, 7
    LOAD    x8, 8
    LOAD    x9, 9
    LOAD    x10, 10
    LOAD    x11, 11
    LOAD    x12, 12
    LOAD    x13, 13
    LOAD    x14, 14
    LOAD    x15, 15
    LOAD    x16, 16
    LOAD    x17, 17
    LOAD    x18, 18
    LOAD    x19, 19
    LOAD    x20, 20
    LOAD    x21, 21
    LOAD    x22, 22
    LOAD    x23, 23
    LOAD    x24, 24
    LOAD    x25, 25
    LOAD    x26, 26
    LOAD    x27, 27
    LOAD    x28, 28
    LOAD    x29, 29
    LOAD    x30, 30
    LOAD    x31, 31

    # sp = x2 = stack[2]
    LOAD    x2, 2

    # return
    sret