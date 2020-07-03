    .section .text.entry
    .globl _start

_start:
    # t0[12:32) = boot_page_table[12:32), t0[0:12) = 0, sign extended to 64 bit
    lui     t0, %hi(boot_page_table)
    # t1 = 0xffffffff00000000
    li      t1, 0xffffffff00000000
    # t0 = t0 - t1
    sub     t0, t0, t1
    # t0 >> 12, sign extended
    srli    t0, t0, 12
    # t1 = 8 << 60
    li      t1, (8 << 60)
    # t0 = t0 | t1
    or      t0, t0, t1
    # PAGE_TABLE = t0
    csrw    satp, t0
    sfence.vma

    # sp[12:32) = boot_stack_top[12:32), sp[0:12) = 0, sign extended to 64 bit
    lui     sp, %hi(boot_stack_top)
    addi    sp, sp, %lo(boot_stack_top)

    # pc[12:32) = rust_main[12:32), pc[0:12) = 0, sign extended to 64 bit
    lui     t0, %hi(rust_main)
    addi    t0, t0, %lo(rust_main)
    # rust_main()
    jr      t0

boot_stack:
    .space 4096 * 16
    .global boot_stack_top

boot_stack_top:

    .section .data
    .align 12

# A Giga-Page
boot_page_table:
    .quad 0
    .quad 0
    # 0x8000_0000[VPN2] = (0x8000_0000 >> 30) = 0x10 = 2 (Second Entry)
    .quad (0x80000 << 10) | 0xcf    # entry: 0x8000_0000 -> 0x8000_0000
    .zero 507 * 8
    # 0xffff_ffff_8000_0000[VPN2] = (0xffff_ffff_8000_0000 >> 30)= 510
    .quad (0x80000 << 10) | 0xcf    # entry: 0xffff_ffff_8000_0000 -> 0x8000_0000
    .quad 0