# ===== Config =====
TARGET      := x86_64-unknown-none
KERNEL_NAME := kernel
ISO_NAME    := vishyos.iso

KERNEL_DIR  := kernel
RUST_BUILD  := $(KERNEL_DIR)/target/debug
RUST_LIB    := $(RUST_BUILD)/libkernel.a

KERNEL_ELF  := iso/boot/kernel.elf

AS          := as
LD          := ld.lld
CARGO       := cargo
QEMU        := qemu-system-x86_64

# ===== Sources =====
ASM_SRCS := boot.s multiboot.s
ASM_OBJS := $(ASM_SRCS:.s=.o)

# ===== Flags =====
ASFLAGS :=
LDFLAGS := -T linker.ld

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
	$(LD) $(LDFLAGS) \
		-o $(KERNEL_ELF) \
		multiboot.o \
		boot.o \
		$(RUST_LIB)

# --- ISO ---
iso: kernel
	grub-mkrescue -o $(ISO_NAME) iso/

# --- Run ---
run: iso
	$(QEMU) $(QEMU_FLAGS)

# --- Clean ---
clean:
	rm -f *.o
	rm -f $(ISO_NAME)
	rm -f $(KERNEL_ELF)
	cd $(KERNEL_DIR) && $(CARGO) clean

