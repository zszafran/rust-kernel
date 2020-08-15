default: run

bootimage:
	cargo bootimage

run: bootimage
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-freestanding/debug/bootimage-rust-kernel.bin

run-curses: bootimage
	qemu-system-x86_64 -curses -drive format=raw,file=target/x86_64-freestanding/debug/bootimage-rust-kernel.bin