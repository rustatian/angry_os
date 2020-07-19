build:
	cd build && cmake .. && cmake --build . --target angry_os.iso
run:
	qemu-system-x86_64 -m 128 -vga none -device virtio-vga,xres=1024,yres=768 -cdrom bin/angry_os.iso