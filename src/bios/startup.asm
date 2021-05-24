extern init_vga
global start

BITS 16
start:
	nop
	nop
	mov ax, cs
	mov ds, ax

	mov ax, 0xc00
	mov ss, ax
	mov sp, 0x3000

	xor ax, ax
	mov es, ax
	pusha

	; some initialize process
	call dword init_vga

	popa
	mov ds, ax
	mov ss, ax
	xor sp, sp

	jmp 0x0:0x7c00
