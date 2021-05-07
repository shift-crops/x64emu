global start

bits 16
start:
	mov ax, cs
	mov ds, ax

	mov ax, 0xc00
	mov ss, ax
	mov sp, 0x3000

	xor ax, ax
	mov es, ax
	pusha

	; some initialize process

	popa
	mov ds, ax
	mov ss, ax
	xor sp, sp

	jmp 0x0:0x7c00
