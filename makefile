all:
	cargo bootimage
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-custom_os/debug/bootimage-custom_os.bin

test:
	cargo test
