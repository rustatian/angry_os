cmake_minimum_required(VERSION 3.16)
project(angry_os LANGUAGES ASM C CXX)

## Setup cross-compilation
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR i686)
set(CMAKE_ASM_NASM_OBJECT_FORMAT elf32)

# CRT
set(CRTBEGIN_OBJ ${CMAKE_SOURCE_DIR}/cross/lib/gcc/i686-elf/10.1.0/crtbegin.o)
set(CRTEND_OBJ ${CMAKE_SOURCE_DIR}/cross/lib/gcc/i686-elf/10.1.0/crtend.o)
#set(CRTI_OBJ ${CMAKE_SOURCE_DIR}/lib/crti.o)
#set(CRTN_OBJ ${CMAKE_SOURCE_DIR}/lib/crtn.o)

#OBJS:=foo.o bar.o
#CRTI_OBJ=crti.o
#OBJ_LINK_LIST:=$(CRTI_OBJ) $(CRTBEGIN_OBJ) $(OBJS) $(CRTEND_OBJ) $(CRTN_OBJ)
#INTERNAL_OBJS:=$(CRTI_OBJ) $(OBJS) $(CRTN_OBJ)
#
#myos.kernel: $(OBJ_LINK_LIST)
#$(CC) -o myos.kernel $(OBJ_LINK_LIST) -nostdlib -lgcc
#
#clean:
#rm -f myos.kernel $(INTERNAL_OBJS)

enable_language(ASM_NASM)

set(CMAKE_C_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/i686-elf-gcc)
set(CMAKE_CXX_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/i686-elf-g++)
#set(CMAKE_ASM_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/i686-elf-as)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_STANDARD 18)
#set(CMAKE_ASM_FLAGS "-msyntax=intel")
set(CMAKE_CXX_FLAGS "-ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti")
set(CMAKE_C_FLAGS "-ffreestanding -O2 -Wall -Wextra")
set(CMAKE_ASM_NASM_FLAGS_RELEASE "-O3")
set(CMAKE_ASM_NASM_FLAGS_RELWITHDEBINFO "-O2 -g")
set(CMAKE_ASM_NASM_FLAGS_MINSIZEREL "-Os")

set(KERNEL_BIN angry_os.bin)
set(KERNEL_ISO ${CMAKE_PROJECT_NAME}.iso)

add_library(CRTI_OBJ OBJECT lib/crti.asm)
add_library(CRTN_OBJ OBJECT lib/crtn.asm)

add_executable(${KERNEL_BIN} boot.asm CRTI_OBJ ${CRTBEGIN_OBJ} kernel.cpp)
set_target_properties( ${KERNEL_BIN} ${CRTEND_OBJ} CRTN_OBJ PROPERTIES LINK_FLAGS "-T ${CMAKE_SOURCE_DIR}/linker.ld -ffreestanding -O2 -nostdlib -lgcc")

add_custom_command(TARGET ${KERNEL_BIN} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/bin
        COMMAND ${CMAKE_COMMAND} -E copy ${KERNEL_BIN} ${CMAKE_SOURCE_DIR}/bin/)

## Generating the ISO file
add_custom_target(${KERNEL_ISO}
        COMMAND ../../../generate-iso.sh
        COMMENT "Generating the kernel bootable iso file"
        BYPRODUCTS ${KERNEL_ISO})
add_dependencies(angry_os.iso angry_os.bin)
add_custom_command(TARGET ${KERNEL_ISO} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/bin
        COMMAND ${CMAKE_COMMAND} -E copy ${KERNEL_ISO} ${CMAKE_SOURCE_DIR}/bin/
        COMMAND qemu-system-i386 -m 128 -M accel=kvm -cpu host -smp 4 -vga none -device virtio-vga,xres=1024,yres=768 -cdrom ${CMAKE_SOURCE_DIR}/bin/${KERNEL_ISO})