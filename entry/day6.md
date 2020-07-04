# 7月2日，Day 6

大概搞清楚了内核堆和用户程序用的堆的区别。

## Lab 2 & 3

#### 内核堆

在 Lab 2 中，一个大小为 8MiB 的内核堆被创建在了内核的 .bss 区段中，值得注意的是这个堆并**不**为用户程序提供内存分配。而只是为了便于在内核中实现一些需要动态内存的小容器使用。其也不负责管理整个操作系统的内存。

PS: 我之前的误解就是认为这个堆负责管理整个操作系统的内存，因此对教程的很多内容产生了误解，在仔细研究过 Lab 2 和 Lab 3 的代码后我发现我错了。

#### RISC-V 的内存模型

要理解 RISC-V 的内存模型，可能首先得理解**页号**与**地址**之间的对应关系与含义。

- 物理地址：物理内存上的一个点，真正的地址，在一个进行过内核重映射的 OS 中，一般通过**虚拟地址**来计算出**虚拟页号**，再在对应的页表项上写入由**物理地址**计算得到的**物理页号**，最后硬件通过物理页表来访问。
- 物理页号：页表项中的值，一般一个页为 4 KiB 大小。
- 虚拟地址：一般用户程序访问的地址，其可以通过计算得到**虚拟页号**，**虚拟页号**对应页表中的一个页表项，页表项就对应的是**物理页号**，这些转换流程通常由硬件自动实现。
- 虚拟页号：对应页表中的一个页表项，页表项内存储**物理页号**及其对应的权限。

##### Example

0xffff_ffff_8000_0000 -> 0x8000_0000:

1. 首先计算虚拟页号 510 = dec(0xffff_ffff_8000_0000 >> 30)，所以是第 3 级页表的第 510 项。

2. 在启动时的内核映射时，由于使用**吉页** (Giga-Page) 的设计，每一个页表项都映射一个 1 GiB 的吉页，因此只需要在第 3 级页表的第 510 项写入 0x8000_0000 的物理页号即可。
3. 其物理页号为，0x8000_0000 >> 12 = 0x8_0000。

#### 附录：物理地址结构与虚拟地址结构

![img](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-3/pics/sv39_address.png)






