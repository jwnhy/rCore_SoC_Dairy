TARGET      	:= riscv64imac-unknown-none-elf
MODE        	:= debug

KERNEL_FILE 	:= target/$(TARGET)/$(MODE)/os
BIN_FILE    	:= target/$(TARGET)/$(MODE)/kernel.bin

OBJDUMP     := rust-objdump --arch-name=riscv64
OBJCOPY     := rust-objcopy --binary-architecture=riscv64

.PHONY: doc kernel build clean qemu run env

build: $(BIN_FILE)

test: $(TEST_BIN_FILE)

doc:
	@cargo doc --document-private-items

kernel:
	@cargo build

test_kernel:
	@cargo test --no-run

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

debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios default -device loader,file=$(BIN_FILE),addr=0x80200000" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_FILE)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d