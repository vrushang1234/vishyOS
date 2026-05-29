# vishyOS
#### A custom hobby kernel for x86 systems built in Rust.
![Kernel Screenshot](https://github.com/vrushang1234/vishyOS/blob/main/assets/kernel-ss1.png)

Current implementation supports:
* GDT and IDT for interrupt handling
* Keyboard and Timer interrupts
* Printing via Limine framebuffer
* Interactive shell with built-in commands
* 4 KiB page frame abstraction
* Simple physical frame allocator

## Prerequisites
* QEMU
* Rust
* rustup
* xorisso
* lld
* binutils
  
## How to build?

#### Install dependencies
```bash
rustup toolchain install nightly
rustup target add x86_64-uknown-none
```
#### Build and run 
```bash
make kernel # Build the kernel
make iso  # Create the ISO
make run  # Run the ISO
```

#### Clean
```bash
make clean 
```
