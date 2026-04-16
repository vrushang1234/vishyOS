# vishyOS
#### A custom hobby kernel for x86 systems built in Rust.
![Kernel Screenshot](https://github.com/vrushang1234/vishyOS/blob/main/assets/kernel-ss1.png)

Current implementation supports GDT and IDT for interrupt handling, Keyboard and Timer interrupts, and printing via Limine framebuffer.

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
