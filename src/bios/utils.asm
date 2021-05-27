global store_esb, store_esw, store_esd
global load_esb, load_esw, load_esd
global memcpy_es, memset_es, strlen_es
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

load_esb:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	lodsb
	pop di
	o32 leave
	o32 ret

load_esw:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	lodsw
	pop di
	o32 leave
	o32 ret

load_esd:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	lodsd
	pop di
	o32 leave
	o32 ret

memcpy_es:
	push ebp
	mov bp, sp
	push si
	push di
	push cx
	mov di, word [bp+0x8]
	mov si, word [bp+0xc]
	mov cx, word [bp+0x10]
	repnz movsb
	pop cx
	pop di
	pop si
	o32 leave
	o32 ret

memset_es:
	push ebp
	mov bp, sp
	push di
	push cx
	mov di, word [bp+0x8]
	mov al, byte [bp+0xc]
	mov cx, word [bp+0x10]
	repnz stosb
	pop cx
	pop di
	o32 leave
	o32 ret

strlen_es:
	push ebp
	mov bp, sp
	push di
	push cx
	mov di, word [bp+0x8]
	xor ax, ax
	xor cx, cx
	not cx
	repnz scasb
	xor ax, ax
	sub ax, cx
	sub ax, 2
	pop cx
	pop di
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
