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

