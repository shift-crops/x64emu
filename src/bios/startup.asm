global start

bits 16
start:
	mov ax, cs
	mov ds, ax

	mov ax, 0x400
	mov ss, ax
	mov sp, 0x2000

	xor ax, ax
	mov es, ax

	jmp 0x0:0x7c00
