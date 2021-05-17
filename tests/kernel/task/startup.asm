extern init_idt, init_paging
global start

BITS 16
start:
	cli
	; gdtr
	lgdt [gdtr]

	; Protected mode
	mov eax, cr0
	or al, 1
	mov cr0, eax

	mov ax, DS_KERNEL
	mov ds, ax
	mov ss, ax
	jmp CS_KERNEL:next

BITS 32
next:
	; ldtr
	mov eax, LDT
	lldt ax

	; tr
	mov eax, TASK0
	ltr ax

	; paging
	call init_paging
	mov cr3, eax
	mov [tss1.cr3], eax

	mov eax, cr0
	or eax, 0x80000000
	mov cr0, eax

	; idtr
	call init_idt
	lidt [eax]

	sti
inf:
	hlt
	int3
	jmp inf

handle_task:
	mov dword [esp], 0xdeadbeef
	int3
	iret
	jmp handle_task

align 4
gdtr:
	dw gdt.end - gdt -1
	dd gdt
gdt:
	dq 0
.cs0:
	dw 0xffff
	dw 0x0000
	db 0x00
	db 0x9a
	db 0xcf
	db 0x00
.ds0:
	dw 0xffff
	dw 0x0000
	db 0x00
	db 0x92
	db 0xcf
	db 0x00
.ldt:
	dw ldt.end - ldt - 1
	dw ldt
	db 0x00
	db 0x82
	db 0x00
	db 0x00
.task0:
	dw tss0.end - tss0 - 1
	dw tss0
	db 0x00
	db 0x89
	db 0x00
	db 0x00
.task1:
	dw tss1.end - tss1 - 1
	dw tss1
	db 0x00
	db 0x89
	db 0x00
	db 0x00
.end:

CS_KERNEL : equ gdt.cs0 - gdt
DS_KERNEL : equ gdt.ds0 - gdt
LDT   : equ gdt.ldt - gdt
TASK0 : equ gdt.task0 - gdt
TASK1 : equ gdt.task1 - gdt

align 4
ldt:
	dq 0
.end:

align 4
tss0:
times 0x66 db 0
	dw 0x2000
.end:

tss1:
times 0x1c db 0
.cr3:
	dd 0x0000
	dd handle_task
	dd 0x0000
times 0x10 db 0
	dd 0x1000
times 0x0c db 0
	dd 0x0000
	dd CS_KERNEL
	dd DS_KERNEL
	dd DS_KERNEL
	dd 0x0000
	dd 0x0000
	dd LDT
	dw 0x0000
	dw 0x2000
.end:
