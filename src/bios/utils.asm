global store_esb, store_esw, store_esd
global load_dsb, load_dsw, load_dsd
global memcpy_es, memcpy_es_r, memset_es, strlen_es
global in_byte, out_byte, in_word, out_word, cli, sti

BITS 16
store_esb:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	mov al, byte [bp+0xc]
	stosb
	pop di
	o32 leave
	o32 ret

store_esw:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	mov ax, word [bp+0xc]
	stosw
	pop di
	o32 leave
	o32 ret

store_esd:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	mov eax, dword [bp+0xc]
	stosd
	pop di
	o32 leave
	o32 ret

load_dsb:
	push ebp
	mov bp, sp
	push si
	mov si, word [bp+0x8]
	lodsb
	pop si
	o32 leave
	o32 ret

load_dsw:
	push ebp
	mov bp, sp
	push si
	mov si, word [bp+0x8]
	int3
	lodsw
	int3
	pop si
	o32 leave
	o32 ret

load_dsd:
	push ebp
	mov bp, sp
	push si
	mov si, word [bp+0x8]
	lodsd
	pop si
	o32 leave
	o32 ret

memcpy_es:
	push ebp
	mov bp, sp
	pushf
	push si
	push di
	push cx
	mov di, word [bp+0x8]
	mov si, word [bp+0xc]
	mov cx, word [bp+0x10]
	cld
	rep movsb
	pop cx
	pop di
	pop si
	popf
	o32 leave
	o32 ret

memcpy_es_r:
	push ebp
	mov bp, sp
	pushf
	push si
	push di
	push cx
	mov di, word [bp+0x8]
	mov si, word [bp+0xc]
	mov cx, word [bp+0x10]
	std
	rep movsb
	pop cx
	pop di
	pop si
	popf
	o32 leave
	o32 ret

memset_es:
	push ebp
	mov bp, sp
	pushf
	push di
	push cx
	mov di, word [bp+0x8]
	mov al, byte [bp+0xc]
	mov cx, word [bp+0x10]
	cld
	rep stosb
	pop cx
	pop di
	popf
	o32 leave
	o32 ret

strlen_es:
	push ebp
	mov bp, sp
	pushf
	push di
	push cx
	mov di, word [bp+0x8]
	xor ax, ax
	xor cx, cx
	not cx
	cld
	repnz scasb
	xor ax, ax
	sub ax, cx
	sub ax, 2
	pop cx
	pop di
	popf
	o32 leave
	o32 ret

in_byte:
	push ebp
	mov bp, sp
	push dx
	mov dx, word [bp+0x8]
	xor ax, ax
	in al, dx
	pop dx
	o32 leave
	o32 ret

out_byte:
	push ebp
	mov bp, sp
	push ax
	push dx
	mov dx, word [ebp+0x8]
	mov ax, word [ebp+0xc]
	out dx, al
	pop dx
	pop ax
	o32 leave
	o32 ret

in_word:
	push ebp
	mov bp, sp
	push dx
	mov dx, word [bp+0x8]
	in ax, dx
	pop dx
	o32 leave
	o32 ret

out_word:
	push ebp
	mov bp, sp
	push ax
	push dx
	mov dx, word [ebp+0x8]
	mov ax, word [ebp+0xc]
	out dx, ax
	pop dx
	pop ax
	o32 leave
	o32 ret

cli:
	cli
	o32 ret

sti:
	sti
	o32 ret
