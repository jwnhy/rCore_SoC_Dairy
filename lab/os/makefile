TARGET      := riscv64imac-unknown-none-elf
MODE        := debug
KERNEL_FILE := target/$(TARGET)/$(MODE)/os
BIN_FILE    := target/$(TARGET)/$(MODE)/kernel.bin

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

.PHONY: doc kernel build clean qemu run env

build: $(BIN_FILE)

doc:
	@cargo doc --document-private-items

kernel:
	@cargo build

$(BIN_FILE): kernel
	$(OBJCOPY) $(KERNEL_FILE) --strip-all -O binary $(BIN_FILE)

asm:
	@$(OBJDUMP) -d $(KERNEL_FILE) | less

clean:
	@cargo clean

qemu: build
	@qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios default \
            -device loader,file=$(BIN_FILE),addr=0x80200000

run: build qemu