cmake_minimum_required(VERSION 3.16 FATAL_ERROR)
project(os_dev_book)
enable_language(ASM_NASM)
set_source_files_properties(${ASM_SOURCES} PROPERTIES LANGUAGE ASM_NASM)
set(CMAKE_ASM_NASM_LINK_EXECUTABLE "ld <CMAKE_ASM_NASM_LINK_FLAGS> <LINK_FLAGS> <OBJECTS>  -o <TARGET> <LINK_LIBRARIES>")

set(CMAKE_ASM_NASM_OBJECT_FORMAT elf32)
SET(ASM_DIALECT "-NASM")
set(CMAKE_ASM_NASM_FLAGS "-f bin")
set(CMAKE_ASM_NASM_COMPILE_OBJECT "<CMAKE_ASM_NASM_COMPILER> <INCLUDES> \
    <FLAGS> -f ${CMAKE_ASM_NASM_OBJECT_FORMAT} -o <OBJECT> <SOURCE>")


add_custom_target(raw_boot_sector
        COMMAND nasm ${CMAKE_SOURCE_DIR}/boot_sec.asm -f bin -o ${CMAKE_SOURCE_DIR}/boot_sec.bin
        COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_SOURCE_DIR}/bin
        COMMAND ${CMAKE_COMMAND} -E rename ${CMAKE_SOURCE_DIR}/boot_sec.bin ${CMAKE_SOURCE_DIR}/bin/boot_sec.bin)
