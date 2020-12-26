# angry os 😠

**🏗 Structure**:


**📝 Prerequisites**:  



**🧱 How to build**:  


**🏎 Roadmap**:
- UEFI kernel
    - UEFI as loader
    - UEFI for memory map and stdout
    
- Non-shared memory kernel
    - Rust-style kernel
    - Memory can be shared between cores if it is read-only
    - Mutable memory is exclusive to one core
        - Cache coherency
        - No locks
