build:
	nasm -g -Fdwarf -f elf64 loader.s -o loader.o
	ld -T link.ld -elf_x86_64 loader.o -o kernel.elf
	rm loader.o