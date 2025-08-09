# Obtain Kernel source
* Download kernel source tarball / checkout git repo
* Kernel source tarball from [kernel.org](https://www.kernel.org/) / checkout from [git.kernel.org](https://git.kernel.org/)
* Install any custom/downstream kernel source, e.g. [github.com/Rust-for-Linux/linux](https://github.com/Rust-for-Linux/linux)
* For native build in Linux distribution, install the package `linux-source`, e.g. using `apt` in `Ubuntu` and locate the source in `/usr/src`

# Prepare the host system for the build
* Ensure necessary rust packages are installed and supported for build
```
make rustavailable
```
If the output is `Rust is available!`, system is ready to build kernel with RUST support.
* Install necessary packages like rustc & other rust tools, clang, bindgen etc, if any missing packages are reported.
> Please refer `Documentation/rust/quick-start.rst` also
> [Quick Start](https://docs.kernel.org/6.10/rust/quick-start.html)

# Appy suitable config
* For native builds in Linux distributions, copy /boot/config-xxx
* Using suitable `defconfig` target of make
* Copy custom config provided by an expert
* Run the following command
```
make olddefconfig
```

# Customize the config
* Run `make menuconfig` and ensure RUST support is enabled
* Enable `General Setup ==> Rust Support`, and ensure `CONFIG_RUST=y` found in .config
* Enable `Kernel Hacking ==> Sample Kernel Code ==> Rust Samples`
* Also change Local Version in General Setup (Especially for native build, to avoid override the factory kernel)

# Build the kernel image and modules
```
make LLVM=1 -j<n>   # bzImage, modules
```

# Install the modules and kernel image
* For native builds
```
make LLVM=1 modules_install
make LLVM=1 install
```
* For cross builds, follow applicable steps to copy the modules and kernel image to target rootfs

# Additional tips
* If default rust compiler is not supporting, add `RUSTC=rustc=1.xx` to make, e.g. to compile using rustc-1.80
```
make LLVM=1 RUSTC=rustc-1.80 -j12
```
* If default bindgen is not supporting, add `BINDGEN=bindgen-0.xx`. to make command, e.g.
```
make LLVM=1 BINDGEN=bindgen-0.65 -j12
```

 

