# angry os 😠

**🏗 Structure**:


**📝 Prerequisites**:  
- https://sandpile.org/x86/initial.htm  



**🧱 How to build**:  


**🏎 Roadmap**:
- Considerations
    - RAM should be more than > 4Gb
    - NO LEGACY!
- UEFI kernel
    - PXE boot (via internet)
    - UEFI as loader
    - UEFI for memory map and stdout
    
- Non-shared memory kernel
    - Rust-style kernel
    - Memory can be shared between cores if it is read-only
    - Mutable memory is exclusive to one core
        - Cache coherency
        - No locks
    - Page tables don't need locks
    - No TLB shootdowns
    
- Soft reboots
    - Bootloader/Loader
