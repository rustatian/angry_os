cmake_minimum_required(VERSION 3.14)
project(angry_os)

enable_language(ASM_NASM)
set(CMAKE_CXX_STANDARD 17)


add_executable(kernel boot.s kernel.c)
