#include <stdint.h>
#include "../utils.h"
#include "font8x8.h"

typedef struct {
	uint8_t red;
	uint8_t green;
	uint8_t blue;
} rgb_t;

const rgb_t palette[0x10] = {
//       R,    G,    B
	{0x00, 0x00, 0x00},
	{0x00, 0x00, 0x2a},
	{0x00, 0x2a, 0x00},
	{0x00, 0x2a, 0x2a},
	{0x2a, 0x00, 0x00},
	{0x2a, 0x00, 0x2a},
	{0x2a, 0x15, 0x00},
	{0x2a, 0x2a, 0x2a},
	{0x15, 0x15, 0x15},
	{0x15, 0x15, 0x3f},
	{0x15, 0x3f, 0x15},
	{0x15, 0x3f, 0x3f},
	{0x3f, 0x15, 0x15},
	{0x3f, 0x15, 0x3f},
	{0x3f, 0x3f, 0x15},
	{0x3f, 0x3f, 0x3f}
};

void config_crt(void);
void config_seq(void);
void config_gc(void);
void config_attr(void);
void config_dac(void);
void load_font(void);

void init_vga(void){
	cli();

	out_byte(0x3c2, 0x2);    // gr.msr = 2 (mem_ena)
	out_word(0x3ce, 0xff08); // gc.bmr = 0xff (bit mask)

	load_font();

	config_crt();
	config_seq();
	config_gc();
	config_attr();
	config_dac();

	sti();
}

void load_font(void){
	out_word(0x3c4, 0x0402); // seq.pmr = 0x4 (mask only plane2)
	out_word(0x3c4, 0x0604); // seq.mmr = 0x6 (ext_mem, oe_dis)

	out_word(0x3ce, 0x0005); // gc.gmr = 0 (read/write mode : 0)
	out_word(0x3ce, 0x0006); // gc.mr = 0 (text mode, map mode : 0)

	__asm__(
		"mov ax, es\n"
		"push ax\n"
		"mov ax, 0xa000\n"
		"mov es, ax"
	);

	for(int i=0; i<0x80; i++)
		memcpy_es((void*)(i*0x20), font8x8_basic[i], 8);

	__asm__(
		"pop ax\n"
		"mov es, ax"
	);
}

void config_crt(void){
	out_word(0x3b4, 0x2801); // hdeer = 0x28
	out_word(0x3b4, 0x1912); // vdeer = 0x19
	out_word(0x3b4, 0x0709); // mslr = 0x7 (scan_count : 8-1)
	out_word(0x3b4, 0x060a); // tcsr = 0x6 (cur_srt : 6)
	out_word(0x3b4, 0x170b); // tcer = 0x7 (cur_end : 7, cur_skew : 1)
}

void config_seq(void){
	out_word(0x3c4, 0x0302); // pmr = 0x3 (mask only plane0,1)
	out_word(0x3c4, 0x0003); // cfr = 0x0 (char font A/B : 0)
	out_word(0x3c4, 0x0204); // mmr = 0x2 (ext_mem, oe_dis : 0)
}

void config_gc(void){
	out_word(0x3ce, 0x1005); // gmr = 0x10 (oe_cga)
	out_word(0x3ce, 0x0e06); // mr = 0x0e (oe_decode, map mode : 3)
}

void config_attr(void){
	for(int i=0; i<0x10; i++){
		out_byte(0x3c0, i);
		out_byte(0x3c0, i);
	}
	out_byte(0x3c0, 0x10);
	out_byte(0x3c0, 0x08);  // mcr = 0x8 (ebsb_sel)
}

void config_dac(void){
	out_byte(0x3c8, 0);
	for(int i=0; i<0x10; i++){
		out_byte(0x3c9, palette[i].red);
		out_byte(0x3c9, palette[i].green);
		out_byte(0x3c9, palette[i].blue);
	}
}
