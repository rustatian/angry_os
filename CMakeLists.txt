cmake_minimum_required(VERSION 3.14)
project(angry_os)

#check_language(Fortran)
#if(CMAKE_Fortran_COMPILER)
#    enable_language(Fortran)
#else()
#    message(STATUS "No Fortran support")
#endif()

enable_language(ASM_NASM)

set(CMAKE_CXX_STANDARD 17)

#add_executable(loader loader.s)
#target_link_libraries(loader link.ld)