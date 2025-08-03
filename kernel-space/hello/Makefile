# Top-level Makefile snippet to build Rust module

obj-m := hello_rust.o
hello_rust-objs += src/lib.o

# Path to your kernel source tree
KERNEL_SRC := /lib/modules/6.8.12-rustemb/build

# Default target
all:
	make -C $(KERNEL_SRC) M=$(PWD) modules 

clean:
	make -C $(KERNEL_SRC) M=$(PWD) clean 

