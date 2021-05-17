extern switch_protected, switch_long
global start, start_protected, start_long

BITS 16
start:
	hlt
	jmp switch_protected

BITS 32
start_protected:
	hlt
	jmp switch_long

BITS 64
start_long:
	hlt