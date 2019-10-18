cmake_minimum_required(VERSION 3.14)
project(angry_os)
add_executable(angry_os.bin linker.ld boot.s kernel.c)


set(QEMU_ARGS -cdrom ${CMAKE_SOURCE_DIR}/angry_os.iso)

add_custom_target(angry_os_build
        COMMAND i686-elf-as ${CMAKE_SOURCE_DIR}/boot.s -o ${CMAKE_SOURCE_DIR}/boot.o
        COMMAND i686-elf-gcc -c ${CMAKE_SOURCE_DIR}/kernel.c -o ${CMAKE_SOURCE_DIR}/kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra
        COMMAND i686-elf-gcc -T ${CMAKE_SOURCE_DIR}/linker.ld -o angry_os.bin -ffreestanding -O2 -nostdlib ${CMAKE_SOURCE_DIR}/boot.o ${CMAKE_SOURCE_DIR}/kernel.o -lgcc
        COMMAND cp angry_os.bin ${CMAKE_SOURCE_DIR}/isodir/boot/angry_os.bin
        COMMAND grub-mkrescue -o ${CMAKE_SOURCE_DIR}/angry_os.iso ${CMAKE_SOURCE_DIR}/isodir
)

add_custom_target(angry_os_run
        COMMAND qemu-system-i386 ${QEMU_ARGS}
        DEPENDS angry_os_build
        )

add_custom_target(
        clean_all
        COMMAND rm ${CMAKE_SOURCE_DIR}/isodir/boot/angry_os.bin
        COMMAND rm ${CMAKE_SOURCE_DIR}/boot.o
        COMMAND rm ${CMAKE_SOURCE_DIR}/kernel.o
        COMMAND rm ${CMAKE_SOURCE_DIR}/angry_os.iso
)

add_custom_target(
        run_all
        DEPENDS angry_os_build
        DEPENDS angry_os_run
)