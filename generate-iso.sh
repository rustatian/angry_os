#!/usr/bin/env sh
mkdir -p isodir/boot/grub
cp angry_os.bin isodir/boot/angry_os.bin
echo 'set timeout=1
set default=10
menuentry "angry_os" {
	multiboot /boot/angry_os.bin
}' > isodir/boot/grub/grub.cfg
grub-mkrescue -o angry_os.iso isodir
rm -rf isodir