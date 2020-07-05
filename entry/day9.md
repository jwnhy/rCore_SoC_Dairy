# 7月5日，Day 9

## Test Framework

今天探索了在 rCore 上添加单元测试的可行性，结果是不太行，似乎外国老哥的教程依赖的是一个自定义版本的 `cargo`，移植性问题没有那么好解决。

不过还是改良了昨天的 `memory_check` 函数，它能够检查新内核页表的权限是否与旧内核页表一致，这样避免了出现内核栈无法不具有写权限造成的死循环情况。

```Rust
pub fn kernel_memory_check(memory_set: &MemorySet) {
    for segment in &memory_set.segments {
        let flags = segment.flags;
        for vpn in segment.page_range.iter() {
            let va = VirtualAddress::from(vpn);
            let (pa, entry) = Mapping::lookup(Some(memory_set.mapping.root_ppn.0), va).unwrap();
            // 检查每个页映射是否正确
            assert_eq!(pa, PhysicalAddress::from(va));
            // 检查每个页的权限是否和预期权限一致
            assert_eq!(entry.flags(), flags);
        }
    }
    // 检查启动栈权限
    // 避免因为页表映射错误导致启动栈进入只读区域
    // 之所以不需要检查其他的寄存器是因为上面已经可以覆盖主要的区段，而 boot_stack 区段是在 .bss 区段内，
    // 如果放错不会被发现。
    let mut sp = 0;
    unsafe {
        llvm_asm!("mv $0, sp":"=r"(sp):::);
        let (_, entry) = Mapping::lookup(Some(memory_set.mapping.root_ppn.0), VirtualAddress(sp)).unwrap();
        assert_eq!(entry.flags(), Flags::VALID|Flags::READABLE|Flags::WRITABLE);
    }
}
```

