# 7月14日，Day 18

## Lab 4 完成

因为浏览器缓存问题遇到了一个内核栈的坑, 前天的学习分享里讲了页表自动释放的坑. 今天记录下内核栈的坑吧.

[Issue #65](https://github.com/rcore-os/rCore-Tutorial/issues/65)

原因是我看的教程代码与最新版本教程代码不一致导致的(可能是因为浏览器缓存???)

主要涉及的代码如下:

```Rust
pub fn prepare(&self) -> *mut Context {
        self.process.read().memory_set.flush();
        let parked_frame = self.inner().context.take().unwrap();

        if self.process.read().is_user {
            KERNEL_STACK.push_context(parked_frame)
            // 用户线程会将 `sscratch` 的值设置为内核栈
        } else {
            let context = (parked_frame.sp() - size_of::<Context>()) as *mut Context;
            unsafe { *context = parked_frame }
            context
            // 内核线程直接使用自己的栈
        }
    }
```

这里将用户线程和内核线程分开, 但是汇编代码上并没有分开. 直接导致一个很奇葩的问题出现了.

```asm
    # swap(sp, sscratch)
    # csrrw   sp, sscratch, sp 不 work
    # stack = stack - 34 * 8
    addi    sp, sp, -34 * 8  # 直接使用 sp 正常
    # stack[1] = x1
    SAVE    x1, 1
    # stack[2] = sscratch
    # csrr    x1, sscratch
    # SAVE    x1, 2
    addi    x1, sp, 34 * 8
    SAVE    x1, 2
```

在上图代码中, 注释部分为不一致代码, 在这种情况下, 用户线程是正常的, 因为所有用户线程共用内核栈, 但是内核线程使用自己的栈, 在 `interrupt.asm` 触发时, 确实将 内核线程 自己的栈 `sp` 存入了 `sscratch`, (只是和用户线程存入的栈不同) 理应work, 但是在内核线程中, 内核线程会使用 `sp` 栈指针, 导致其被修改, 于是在 `__restore` 时, 原来的, 未被修改的栈指针被取出, 导致整体被破坏.

这样其实还会有个非常有意思的现象, 就是仅当被破坏的用户线程被 恢复 时才会出现错误, 而如果不断的切换到新的进程而一直不恢复的话就不会报错.

#### 修复方法

直接统一使用内核栈即可.

```rust
pub fn prepare(&self) -> *mut Context {
        self.process.read().memory_set.flush();
        let parked_frame = self.inner().context.take().unwrap();
        KERNEL_STACK.push_context(parked_frame)
    }
```

并且将不一致代码修改回来. 就可以 work 了