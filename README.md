# angry os

<p align="center">
  <img src="https://github.com/48d90782/angry_os/blob/master/images/angy_os.png" />
</p>

Structure:
1. `kernel.cpp` kernel_main entry point  
2. `generate-iso.sh` script to generate iso for the qemu.  

Prerequisites (ArchLinux):  
1. `i686-elf-gcc` cross-compiler  
2. `xorriso`  
3. `qemu`  

How to build:  
1. CLion - just open project in CLion and press build `angry_os.bin` target and then `angry_os.iso` target. When the build
   will be finished, `qemu` will start automatically.  
   
2. Command line:  
   2.1 `mkdir build && cd build`  
   2.2 `cmake ../`  
   2.3 `cmake --build . --target angry_os.bin && cmake --build . --target angry_os.iso`. If you will get such error `../../../generate-iso.sh: No such file or directory`,
       then in `CMakeLists.txt`, correct path to generate-iso.sh --> `COMMAND ../../../generate-iso.sh`  

Roadmap:  
Bare Bones I:  
- [ ] Scrolling in terminal
- [ ] Rendering Colorful ASCII Art
- [ ] Calling Global Constructors
- [ ] Meaty Skeleton

GUI:  
1. I am planning to use GTK3 for the GUI
