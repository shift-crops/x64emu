global write_esb, write_esw, write_esd, memcpy_es
global in_byte, out_byte, in_word, out_word, cli, sti

BITS 16
write_esb:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	mov ax, word [bp+0xc]
	mov byte [es:di], al
	pop di
	o32 leave
	o32 ret

write_esw:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	mov ax, word [bp+0xc]
	mov word [es:di], ax
	pop di
	o32 leave
	o32 ret

write_esd:
	push ebp
	mov bp, sp
	push di
	mov di, word [bp+0x8]
	mov eax, dword [bp+0xc]
	mov dword [es:di], eax
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
