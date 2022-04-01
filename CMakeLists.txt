cmake_minimum_required(VERSION 3.23)
project(angry_os LANGUAGES ASM C CXX)

## Setup cross-compilation
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR x86_64)
set(CMAKE_ASM_NASM_OBJECT_FORMAT elf64)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_STANDARD 18)
enable_language(ASM_NASM)

# CRT
set(CRTBEGIN_OBJ ${CMAKE_SOURCE_DIR}/cross/lib/gcc/x86_64-elf/11.2.0/crtbegin.o)
set(CRTEND_OBJ ${CMAKE_SOURCE_DIR}/cross/lib/gcc/x86_64-elf/11.2.0/crtend.o)


set(CMAKE_C_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/x86_64-elf-gcc)
set(CMAKE_CXX_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/x86_64-elf-g++)
set(CMAKE_ASM_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/x86_64-elf-as)

set(CMAKE_ASM_FLAGS "-msyntax=intel")
set(CMAKE_CXX_FLAGS "-ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti")
set(CMAKE_C_FLAGS "-ffreestanding -O2 -Wall -Wextra")
set(CMAKE_ASM_NASM_FLAGS_RELEASE "-O3")
set(CMAKE_ASM_NASM_FLAGS_RELWITHDEBINFO "-O2 -g")
set(CMAKE_ASM_NASM_FLAGS_MINSIZEREL "-Os")

set(KERNEL_BIN angry_os.bin)
set(KERNEL_ISO ${CMAKE_PROJECT_NAME}.iso)

add_executable(${KERNEL_BIN} boot.asm kernel.cpp)
set_target_properties(${KERNEL_BIN} PROPERTIES LINK_FLAGS "-T ${CMAKE_SOURCE_DIR}/linker.ld -ffreestanding -O2 -nostdlib -lgcc")

add_custom_command(TARGET ${KERNEL_BIN} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/bin
        COMMAND ${CMAKE_COMMAND} -E copy ${KERNEL_BIN} ${CMAKE_SOURCE_DIR}/bin/)

## Generating the ISO file
add_custom_target(${KERNEL_ISO}
        COMMAND ../../generate-iso.sh
        COMMENT "Generating the kernel bootable iso file"
        BYPRODUCTS ${KERNEL_ISO})
add_dependencies(angry_os.iso angry_os.bin)
add_custom_command(TARGET ${KERNEL_ISO} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/bin
        COMMAND ${CMAKE_COMMAND} -E copy ${KERNEL_ISO} ${CMAKE_SOURCE_DIR}/bin/
        COMMAND qemu-system-i386 -m 128 -M accel=kvm -cpu host -smp 4 -vga none -device virtio-vga,xres=1024,yres=768 -cdrom ${CMAKE_SOURCE_DIR}/bin/${KERNEL_ISO})