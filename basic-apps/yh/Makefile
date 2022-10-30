TARGET := riscv64gc-unknown-none-elf
MODE := release
TARGET_DIR := target/$(TARGET)/$(MODE)
KERNEL_USER_TARGET_DIR := ../../rCore-kernel/user/target/$(TARGET)/$(MODE)
KERNEL_USER_PROG_SRC_DIR := ../../rCore-kernel/user/src/bin
ELF := yh

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64
CP := cp 

binary: 
	@cargo build --release
	@touch $(KERNEL_USER_PROG_SRC_DIR)/$(ELF).add
	@$(CP) $(TARGET_DIR)/$(ELF) $(KERNEL_USER_TARGET_DIR)/$(ELF)
	@$(OBJCOPY) $(TARGET_DIR)/$(ELF) --strip-all -O binary $(patsubst $(KERNEL_USER_TARGET_DIR)/%, $(KERNEL_USER_TARGET_DIR)/%.bin, $(KERNEL_USER_TARGET_DIR)/$(ELF));

build: binary

clean:
	@cargo clean

.PHONY: binary build clean
