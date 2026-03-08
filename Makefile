# ===== Config =====
TARGET      := x86_64-unknown-none
KERNEL_NAME := kernel
ISO_NAME    := vishyos.iso

KERNEL_DIR  := kernel
RUST_BUILD  := $(KERNEL_DIR)/target/debug
RUST_LIB    := $(RUST_BUILD)/libkernel.a

ISO_DIR     := iso
BOOT_DIR    := $(ISO_DIR)/boot

KERNEL_ELF  := $(BOOT_DIR)/kernel.elf


AS          := as
LD          := ld.lld
CARGO       := cargo
QEMU        := qemu-system-x86_64

# ===== Sources =====
ASM_SRCS := $(KERNEL_DIR)/boot/boot.s
ASM_OBJS := $(ASM_SRCS:.s=.o)

# ===== Flags =====
ASFLAGS :=
LDFLAGS := -T $(KERNEL_DIR)/linker.ld

QEMU_FLAGS := \
	-cdrom $(ISO_NAME) \
	-serial stdio \
	-no-reboot \
	-no-shutdown \
	-m 512M

# ===== Targets =====

.PHONY: all run clean iso kernel rust asm

all: iso

# --- Rust ---
rust:
	cd $(KERNEL_DIR) && $(CARGO) build -Zbuild-std

# --- ASM ---
asm: $(ASM_OBJS)

%.o: %.s
	$(AS) --64 $< -o $@

# --- Kernel ELF ---
kernel: rust asm
	mkdir -p $(BOOT_DIR)
	$(LD) $(LDFLAGS) \
		-o $(KERNEL_ELF) \
		$(KERNEL_DIR)/boot/boot.o \
		$(RUST_LIB)

# --- ISO ---
iso: kernel
	xorriso -as mkisofs \
		-b limine-bios-cd.bin \
		-no-emul-boot \
		-boot-load-size 4 \
		-boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part \
		--efi-boot-image \
		-o $(ISO_NAME) \
		$(ISO_DIR)

	$(ISO_DIR)/limine/limine bios-install $(ISO_NAME)

# --- Run ---
run: iso
	$(QEMU) $(QEMU_FLAGS)

# --- Clean ---
clean:
	rm -f $(KERNEL_DIR)/*.o
	rm -f $(ISO_NAME)
	rm -f $(KERNEL_ELF)
	cd $(KERNEL_DIR) && $(CARGO) clean

