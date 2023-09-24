# sysHyper-t1

A minimal Type-I hypervisor based on AArch64 & implemented in rust, capable of booting Linux / ArceOS on qemu.

Ported from [RVM-Tutorial](https://github.com/equation314/RVM-Tutorial).


## Features

* Lightweight enough, only 3K+ LoC
* Supported guest OS: [ArceOS](https://github.com/rcore-os/arceos), Linux
* Guest/host memory isolation with nested paging
* Device emulation and passthrough
* multi-vCPU and multi-guest support (without schedule)

## Install Build Dependencies

Install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) to use `rust-objcopy` and `rust-objdump` tools:

```console
$ cargo install cargo-binutils
```

[cross compilation tool chain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads) (>= 12.2.Rel1) and [QEMU](https://www.qemu.org/download/) (>= 7.0.0) are also needed.


## Run Arceos

```
cd hypervisor
make run LOG=info SMP=4 GUEST=arceos NET=y FS=y
```

## Run Linux

```
git apply run_linux.patch
cd hypervisor
make run LOG=info SMP=2 GUEST=linux NET=y FS=y
```