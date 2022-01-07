cmake_minimum_required(VERSION 3.16)
project(angry_os LANGUAGES ASM C CXX)

## Setup cross-compilation
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR arm)
set(CMAKE_ASM_NASM_OBJECT_FORMAT elf64)

# CRT
#set(CRTBEGIN_OBJ ${CMAKE_SOURCE_DIR}/cross/lib/gcc/i686-elf/10.1.0/crtbegin.o)
#set(CRTEND_OBJ ${CMAKE_SOURCE_DIR}/cross/lib/gcc/i686-elf/10.1.0/crtend.o)
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

enable_language(ASM)

set(CMAKE_C_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/aarch64-elf-gcc)
set(CMAKE_CXX_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/aarch64-elf-g++)
set(CMAKE_ASM_COMPILER ${CMAKE_SOURCE_DIR}/cross/bin/aarch64-elf-as)
set(CMAKE_OBJCOPY ${CMAKE_SOURCE_DIR}/cross/bin/aarch64-elf-objcopy)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_STANDARD 11)
#set(CMAKE_ASM_FLAGS "-msyntax=intel")
set(CMAKE_CXX_FLAGS "-ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti")
set(CMAKE_C_FLAGS "-ffreestanding -O2 -Wall -Wextra")
set(CMAKE_ASM_FLAGS_RELEASE "-O3")
set(CMAKE_ASM_FLAGS_RELWITHDEBINFO "-O2 -g")
set(CMAKE_ASM_FLAGS_MINSIZEREL "-Os")

set(KERNEL_ELF angry_os.elf)
set(KERNEL_IMG kernel8.img)

#add_library(CRTI_OBJ OBJECT lib/crti.asm)
#add_library(CRTN_OBJ OBJECT lib/crtn.asm)

add_executable(${KERNEL_ELF} boot.S kernel.c)
set_target_properties(${KERNEL_ELF} PROPERTIES LINK_FLAGS "-T ${CMAKE_SOURCE_DIR}/linker.ld -o angry_os.elf -ffreestanding -O2 -nostdlib -lgcc")

add_custom_command(TARGET ${KERNEL_ELF} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/bin
        COMMAND ${CMAKE_COMMAND} -E copy ${KERNEL_ELF} ${CMAKE_SOURCE_DIR}/bin/)

## Generating the ISO file
add_custom_target(${KERNEL_IMG}
        COMMAND ${CMAKE_SOURCE_DIR}/cross/bin/aarch64-elf-objcopy ${KERNEL_ELF} -O binary ${KERNEL_IMG}
        COMMENT "Generating the kernel bootable iso file"
        BYPRODUCTS ${KERNEL_IMG})

add_dependencies(kernel8.img angry_os.elf)
add_custom_command(TARGET ${KERNEL_IMG} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/bin
        COMMAND ${CMAKE_COMMAND} -E copy ${KERNEL_IMG} ${CMAKE_SOURCE_DIR}/bin/
        COMMAND qemu-system-aarch64 -M raspi3 -serial stdio -kernel ${CMAKE_SOURCE_DIR}/bin/${KERNEL_IMG})