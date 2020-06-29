#!/usr/bin/zsh
cargo build
rust-objdump target/riscv64imac-unknown-none-elf/debug/os -h -d --arch-name=riscv64 > obj_info
rust-objcopy target/riscv64imac-unknown-none-elf/debug/os --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/kernel.bin
