# 6月30日，Day 4

今日在校工作，Productivity++。

顺利完成了lab1，可能会比预期的更早完成第一部分任务。

## lab1 总结

#### 中断种类

- 异常：执行指令时产生的不可预料的错误。如：零除，无效地址。
- 陷阱：主动导致中断的指令。如**系统调用**。
- 硬件中断：时钟中断，串口中断。

#### 中断寄存器

RISC-V 有一些 CSR 寄存器 （Control and Status Registers） 用来保存控制信息，其中有一部分就是用来控制中断的。

主要分为两组，S-Mode 中断控制与 M-Mode 中断控制，以**前缀**的方式区分，例如 <u>s</u>cuase 与 <u>m</u>cause。

###### 发生中断时，由硬件自动填写的寄存器

- `sepc` 与 `mepc`

  即 Exception Program Counter，记录触发中断的指令的地址。

  由于 RISC-V 指令不定长，因此跳过发生中断的指令时需要考虑其长度，例如 `ebreak` 命令只需要跳过 2 个字节。

- `scause` 与 `mcause`

  记录中断发生的原因。

- `stval` 在特殊的中断中记录额外的信息。

###### 由程序配置，指导硬件的寄存器

- `stvec ` 与 `mtvec`

  中断向量表配置，存储了基址 BASE 与 模式 MODE，MODE 有两种模式

  1. MODE = 0，表示 Direct 模式，遇到中断直接跳到 BASE 执行。
  2. MODE = 1，表示 Vectorized 中断向量表模式，此时 BASE 代表中断向量表的基址，遇到中断会跳转至 `BASE + 4 * cause`。

- `sstatus` 与 `mstatus`

  全局状态寄存器，有许多状态位，例如全局中断使能。

- `sie` 与 `mie`

  即 Interrupt Enable，其中不同位用于控制具体中断的使能。

- `sip` 与 `mip`

  待处理中断寄存器

  即 Interrupt Pending，指示某具体中断是否被触发，只有 `sie` 与 `sip` 都为 1 时，中断才触发。

###### 进入与退出中断指令

- `ecall` 系统调用指令，可以参考 [day3](https://github.com/JohnWestonNull/rCore_SoC_Dairy/blob/master/entry/day3.md) 中关于系统调用的部分。
- `sret` 与 `mret`，从 S-Mode 返回 U-Mode 与 从 M-Mode 返回 S-Mode。
- `ebreak`，触发一个断点。

###### 操作 CSR

操作 CSR 有一套特殊的命令。下面摘自教程

- `csrrw dst, csr, src`（CSR Read Write）
  同时读写的原子操作，将指定 CSR 的值写入 `dst`，同时将 `src` 的值写入 CSR。
- `csrr dst, csr`（CSR Read）
  仅读取一个 CSR 寄存器。
- `csrw csr, src`（CSR Write）
  仅写入一个 CSR 寄存器。
- `csrc(i) csr, rs1`（CSR Clear）
  将 CSR 寄存器中指定的位清零，`csrc` 使用通用寄存器作为 mask，`csrci` 则使用立即数。
- `csrs(i) csr, rs1`（CSR Set）
  将 CSR 寄存器中指定的位置 1，`csrc` 使用通用寄存器作为 mask，`csrci` 则使用立即数。

#### 中断上下文 Context

```Rust
use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
pub struct Context {
    pub x: [usize; 32],     // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize
}
```

在中断触发时，我们的[**中断处理函数**](https://github.com/JohnWestonNull/rCore_SoC_Dairy/blob/master/lab/os/src/asm/interrupt.asm)会在栈上根据 `Context` 的**布局**依次存储好 `x0` - `x31` 等 32 个通用寄存器的值和两个 CSR 的值 [`sstatus` 与 `sepc`]。

再将栈顶指针放到 `a0` 寄存器中，并且将 `scause` 与 `stval` 利用 `a1` 与 `a2` 寄存器传到用 Rust 写的分派函数中。

> `scause` 与 `stval` 之所以不放到上下文中是因为这两者只是临时变量，不需要特意存储到栈上，只需要在分派时放到对应的参数位置即可。

剩下的就是一些 *trivial* 的东西了，不在此赘述。



---

啊，准备先搞 lab3 再搞 lab2 了，lab2 里面把动态内存放在 bss 区的方式感觉不是很漂亮，所以准备先实现页表再根据类似    blog_os 的方法实现动态内存分配。