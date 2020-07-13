# angry os

<p align="center">
  <img src="https://github.com/48d90782/angry_os/blob/master/images/angy_os.png" />
</p>

Structure:
1. `kernel.cpp` kernel_main entry point  
2. `generate-iso.sh` script to generate iso for the qemu. 
3. `cross` folder. This folder contains cross-compilers ([link](https://wiki.osdev.org/GCC_Cross-Compiler)). Build date: 13.07.2020 (based on the latest stable binutils (2.34) and gcc (10.1) )

Prerequisites:  
1. `xorriso` (preinstalled in Ubuntu) 
2. `qemu` --> `sudo apt install qemu-kvm libvirt-clients libvirt-daemon-system bridge-utils virt-manager`  

How to build:  
1. CLion - just open project in CLion and press build `angry_os.bin` target and then `angry_os.iso` target. When the build
   will be finished, `qemu` will start automatically.  
   
2. Command line:  
   2.1 `mkdir build && cd build`  
   2.2 `cmake ../`  
   2.3 `cmake --build . --target angry_os.bin && cmake --build . --target angry_os.iso`. If you will get such error `../../../generate-iso.sh: No such file or directory`,
       then in `CMakeLists.txt`, correct path to `generate-iso.sh` --> `COMMAND ../../../generate-iso.sh`  

Roadmap:
- [ ] Bare Bones I:  
  - [x] Cross compilers (`cross` folder)  
  - [ ] Scrolling in terminal
  - [ ] Rendering Colorful ASCII Art
  - [ ] Calling Global Constructors
  - [ ] Meaty Skeleton

GUI:  
1. I am planning to use GTK3 for the GUI
