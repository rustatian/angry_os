.set ALIGN, 1<<0
.set MEMINFO, 1<<1
.set FLAGS, ALIGN | MEMINFO
.set MAGIC, 0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)

#3.1.1 The layout of Multiboot header
#The layout of the Multiboot header must be as follows:
#
#Offset	Type	Field Name	Note
#0	u32	magic	required
#4	u32	flags	required
#8	u32	checksum	required

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.section .bss
.align 16
stack_bottom:
    .skip 16384 #16 KiB
stack_top:

.section .text
.global _start
.type _start, @function
_start:
    mov $stack_top, %esp
    call kernel_main
    cli
1:  hlt
    jmp 1b

.size _start, . - _start
