TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm


# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif


# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

KERNEL_ENTRY_PA := 0x80200000



# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64


# Disassembly
DISASM ?= -D


build: clean $(KERNEL_BIN)

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@echo Platform: $(BOARD)
	@cp src/linker-$(BOARD).ld src/linker.ld
	@cargo build $(MODE_ARG)
	@rm src/linker.ld

clean:
	@cargo clean


disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@nvim $(DISASM_TMP)
	@rm $(DISASM_TMP)

run: run-inner

run-inner: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

gdbserver: build
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S

gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'


.PHONY: build env kernel clean disasm disasm-vim run-inner switch-check gdbserver gdbclient
