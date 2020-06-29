# 6月29日，Day 3

今日在校工作，Productivity++。

顺利完成了lab0，可能会比预期的更早完成第一部分任务。

## lab0总结

#### 创建项目，工具链的区别

在使用 `cargo new` 创建新工程时，在工程文件夹的父文件夹中新建了一个名为 `rust-toolchain` 的文本文件。

其中内容只有简单的 `nightly-2020-06-27` ，这个文件是用来指定该项目使用的工具链。一般来说，Rust 工具链有下面三个版本。

- stable：稳定版，只有稳定的特性，不能使用形如#![feature()]等特性宏。
- beta：测试版，较少人使用，据了解在CI时使用。
- nightly：每日版：最激进的版本，在这里才能使用很多不稳定的特性，例如内联汇编。

#### 无标准库的最小程序

在无标准库的情况下，一个程序至少需要实现下面几个部分。

- `panic` 处理函数，并标记为`#[panic_handler]`，一个简单的最小化实现如下。

  ```rust
  #[panic_handler]
  fn panic(_info: &PanicInfo) -> ! {
      loop {}
  }
  ```

- `eh_personality` 这里的 eh 是 Exception Handling 的缩写，是用于标记一个函数用来实现**堆栈展开**功能的。在项目中我们直接在 `Cargo.toml` 中禁用它。

  ```toml
  # panic 时直接终止，因为我们没有实现堆栈展开的功能
  [profile.dev]
  panic = "abort"
  
  [profile.release]
  panic = "abort"
  ```

- `_start` 入口函数，重写 `crt0` 的入口地址，一个简单实现如下。

  ```rust
  #[no_mangle]
  pub extern "C" fn _start() -> ! {
      loop {}
  }
  ```

  至少需要上面三个部分，一个 `#![no_std]` 的程序才可以运行。

  

#### 编译目标与binutils工具集

由于需要编译到 RISC-V 平台，因此需要添加交叉编译链和设置 `cargo` 参数

```bash
rustup target add riscv64imac-unknown-none-elf
```

```toml
# os/.cargo/config 
[build]
target = "riscv64imac-unknown-none-elf"
```

并且Rust社区提供了一套LLVM套件的语法糖工具（可能需要配置环境变量）。

```bash
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

这些工具都是按照工具链版本限定的，因此如果工具链版本改变，这些都需要重新运行安装。

#### 内存布局

对于不同的程序，其所需要的起始的内存地址也不同，一般RISCV程序起始地址为0x11000，而内核一般在0x8020_0000。



```
start address: 0x0000000000011000
...
Program Header:
    PHDR off    0x0000000000000040 vaddr 0x0000000000010040 ...
    LOAD off    0x0000000000000000 vaddr 0x0000000000010000 ...
    LOAD off    0x0000000000001000 vaddr 0x0000000000011000 ...
   STACK off    0x0000000000000000 vaddr 0x0000000000000000 ...
```

下面摘自 `rCore-Tutorial`

- .text 段：代码段，存放汇编代码

- .rodata 段：只读数据段，顾名思义里面存放只读数据，通常是程序中的常量
- .data 段：存放被初始化的可读写数据，通常保存程序中的全局变量
- .bss 段：存放被初始化为 0 的可读写数据，与 .data 段的不同之处在于我们知道它要被初始化为 0，因此在可执行文件中只需记录这个段的大小以及所在位置即可，而不用记录里面的数据，也不会实际占用二进制文件的空间
- Stack：栈，用来存储程序**运行过程**中的局部变量，以及负责函数调用时的各种机制。它从高地址向低地址增长
- Heap：堆，用来支持程序**运行过程中**内存的**动态分配**，比如说你要读进来一个字符串，在你写程序的时候你也不知道它的长度究竟为多少，于是你只能在运行过程中，知道了字符串的长度之后，再在堆中给这个字符串分配内存

<img src="https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-0/pics/typical-layout.png" alt="内存布局示意图" style="zoom:40%;" />

具体程序内存布局可以通过 **linker script** 进行配置，语法参考 [Linker Script](https://sourceware.org/binutils/docs/ld/Scripts.html) 。

OpenSBI 将自身放置在0x8000_0000，完成初始化后跳转到0x8020_0000。这也是我们的程序将会在的地方。

#### OpenSBI

OpenSBI 和 BIOS 与 UEFI 一样，是属于 Firmware 级别的软件，会在启动时由 Bootloader 加载进入。这里的 OpenSBI 固件运行在 RISC-V 的最高权限模式 M-Mode 下，并且在将控制权转交给内核时会进入 RISC-V 为 OS 准备的 S-Mode 下，然后跳转到固定地址 0x8020_0000，具体可以参考[特权模式](https://github.com/JohnWestonNull/rCore_SoC_Dairy/blob/master/pdf_doc/RISCV_%E7%89%B9%E6%9D%83%E6%A8%A1%E5%BC%8F.pdf)。

接下来就是汇编入口，主要重要的事就是初始化栈

```asm
# 关于 RISC-V 下的汇编语言，可以参考 https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md

    .section .text.entry
    .globl _start
# 目前 _start 的功能：将预留的栈空间写入 $sp，然后跳转至 rust_main
_start:
    la sp, boot_stack_top
    call rust_main

    # 回忆：bss 段是 ELF 文件中只记录长度，而全部初始化为 0 的一段内存空间
    # 这里声明字段 .bss.stack 作为操作系统启动时的栈
    .section .bss.stack
    .global boot_stack
boot_stack:
    # 16K 启动栈大小
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top:
    # 栈结尾
```

#### OpenSBI 提供的系统调用

| Function Name              | Function ID | Extension ID | Replacement Extension |
| :------------------------- | ----------- | ------------ | --------------------- |
| sbi_set_timer              | 0           | 0x00         | N/A                   |
| sbi_console_putchar        | 0           | 0x01         | N/A                   |
| sbi_console_getchar        | 0           | 0x02         | N/A                   |
| sbi_clear_ipi              | 0           | 0x03         | N/A                   |
| sbi_send_ipi               | 0           | 0x04         | N/A                   |
| sbi_remote_fence_i         | 0           | 0x05         | N/A                   |
| sbi_remote_sfence_vma      | 0           | 0x06         | N/A                   |
| sbi_remote_sfence_vma_asid | 0           | 0x07         | N/A                   |
| sbi_shutdown               | 0           | 0x08         | N/A                   |
| **RESERVED**               |             | 0x09-0x0F    |                       |

使用 `ecall` 进行系统调用时， OpenSBI 会检查寄存器 `a7` 的值，如果 `a7` 的值在 0x00 到 0x08 之间则执行对应的系统调用，否则交由内核处理。
