extern init_page_protected, init_page_long
extern start_protected, start_long
global switch_protected, switch_long

BITS 16
switch_protected:
	cli

	; PE enable
	mov eax, cr0
	or al, 1
	mov cr0, eax

	; gdtr
	lgdt [gdtr]

	mov ax, DS_KERNEL32
	mov ds, ax
	mov ss, ax
	jmp CS_KERNEL32:.next
.next:
BITS 32
	; init page tables
	;call init_paging_protected
	;mov cr3, eax

	; PG enable
	;mov eax, cr0
	;or eax, 0x80000000
	;mov cr0, eax

	sti
	jmp start_protected

switch_long:
	cli

	; PG disable
	mov eax, cr0
	mov edx, 0x80000000
	not edx
	or eax, edx
	mov cr0, eax

	; PAE enable
	mov eax, cr4
	or eax, 0x20
	mov cr4, eax

	; init page tables
	call init_page_long
	mov cr3, eax

	; LME enable
	mov ecx, 0xc0000080
	rdmsr
	or eax, 0x100
	wrmsr

	; PG enable
	mov eax, cr0
	or eax, 0x80000000
	mov cr0, eax

	; gdtr
	lgdt [gdtr]

	sti
	mov ax, DS_KERNEL64
	mov ds, ax
	mov ss, ax
	jmp CS_KERNEL64:start_long

align 4
gdtr:
	dw gdt.end - gdt -1
	dd gdt
gdt:
	dq 0
.cs32:
	dw 0xffff
	dw 0x0000
	db 0x00
	db 0x9a
	db 0x4f
	db 0x00
.ds32:
	dw 0xffff
	dw 0x0000
	db 0x00
	db 0x92
	db 0x4f
	db 0x00
.cs64:
	dw 0xffff
	dw 0x0000
	db 0x00
	db 0x9a
	db 0xaf
	db 0x00
.ds64:
	dw 0xffff
	dw 0x0000
	db 0x00
	db 0x92
	db 0xaf
	db 0x00
.end:

CS_KERNEL32 : equ gdt.cs32 - gdt
DS_KERNEL32 : equ gdt.ds32 - gdt
CS_KERNEL64 : equ gdt.cs64 - gdt
DS_KERNEL64 : equ gdt.ds64 - gdt
