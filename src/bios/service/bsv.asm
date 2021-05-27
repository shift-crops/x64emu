extern _bsv_video
global reg, bsv_video

BITS 16
bsv_video:
	call prepare_bsv
	call dword _bsv_video
	jmp return_kernel

prepare_bsv:
	mov [cs:reg.ax], ax
	mov [cs:reg.cx], cx
	mov [cs:reg.dx], dx
	mov [cs:reg.bx], bx
	mov [cs:reg.bp], bp
	mov [cs:reg.ds], ds
	mov ax, cs
	mov ds, ax
	ret

return_kernel:
	mov ax, [reg.ax]
	mov cx, [reg.cx]
	mov dx, [reg.dx]
	mov bx, [reg.bx]
	mov bp, [reg.bp]
	mov ds, [reg.ds]
	iret

reg: 
.ax: dw 0
.cx: dw 0
.dx: dw 0
.bx: dw 0
.bp: dw 0
.ds: dw 0