; to linking
global start

; code
section .text
bits 32

start:
    ; print OK
    mov dword [0xb8000], 0x2f4b2f4f
    hlt
