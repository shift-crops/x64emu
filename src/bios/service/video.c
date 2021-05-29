#include <stdint.h>
#include <stdbool.h>
#include "utils.h"
#include "regs.h"
#include "font8x8.h"

static void set_video_mode(uint8_t mode);
static void set_cursor_size(uint8_t start, uint8_t end);
static void set_cursor_position(uint8_t page, uint8_t x, uint8_t y);
static void get_cursor_position(uint8_t page);
static void set_video_page(uint8_t page);
static void scroll_up(uint8_t count, uint8_t attr, uint8_t cols, uint8_t rows);
static void scroll_down(uint8_t count, uint8_t attr, uint8_t cols, uint8_t rows);
static void read_char_attr(uint8_t page);
static void write_char_attr(uint8_t page, uint8_t chr, uint8_t attr, uint16_t count);
static void write_char(uint8_t page, uint8_t chr, uint16_t count);
static void set_color_palette(uint8_t mode, uint8_t color);
static void write_teletype(uint8_t page, uint8_t chr, uint8_t attr);
static void get_video_mode(void);
static void write_string(uint8_t mode, uint16_t buf, uint8_t attr, uint8_t x, uint8_t y);

struct VGAState {
    uint8_t mode;
    uint8_t cols;
    uint8_t rows;
    uint8_t page_number;
    uint16_t page_size;

    struct {
        uint8_t x;
        uint8_t y;
    } cursor[8];
};

static struct VGAState state = {};
static uint8_t seq_pmr;
static uint8_t seq_mmr;
static uint8_t gc_gmr;
static uint8_t gc_mr;

void _bsv_video(void) {
    switch(reg.ah) {
        case 0x00:
            set_video_mode(reg.al);
            break;
        case 0x01:
            set_cursor_size(reg.ch, reg.cl);
            break;
        case 0x02:
            set_cursor_position(reg.bh, reg.dl, reg.dh);
            break;
        case 0x03:
            get_cursor_position(reg.bh);
            break;
        case 0x05:
            set_video_page(reg.al);
            break;
        case 0x06:
            scroll_up(reg.al, reg.bh, reg.dl-reg.cl+1, reg.dh-reg.ch+1);
            break;
        case 0x07:
            scroll_down(reg.al, reg.bh, reg.dl-reg.cl+1, reg.dh-reg.ch+1);
            break;
        case 0x08:
            read_char_attr(reg.bh);
            break;
        case 0x09:
            write_char_attr(reg.bh, reg.al, reg.bl, reg.cx);
            break;
        case 0x0a:
            write_char(reg.bh, reg.al, reg.cx);
            break;
        case 0x0b:
            set_color_palette(reg.bh, reg.bl);
            break;
        case 0x0e:
            write_teletype(reg.bh, reg.al, reg.bl);
            break;
        case 0x0f:
            get_video_mode();
            break;
        case 0x13:
            write_string(reg.al, reg.bp, reg.bl, reg.dl, reg.dh);
            break;
    }
}

static void clear_planes(void);
static void load_font(void);
static void apply_cursor(uint8_t x, uint8_t y);
static void save_regs(void);
static void restore_regs(void);

static void set_video_mode(uint8_t mode){
    clear_planes();

    switch(mode) {
        case 0x00:
        case 0x01:
        case 0x02:
        case 0x03:
            load_font();
            out_word(0x3c4, 0x0302); // seq.pmr = 0x03 (mask only plane0,1)
            out_word(0x3c4, 0x0003); // seq.cfr = 0x00 (char font A/B : 0)
            out_word(0x3c4, 0x0204); // seq.mmr = 0x02 (ext_mem, oe_dis : 0)

            out_word(0x3ce, 0x1005); // gc.gmr = 0x10 (oe_cga)
            out_word(0x3ce, 0x0e06); // gc.mr = 0x0e (oe_decode, map mode : 3)
            apply_cursor(state.cursor[0].x, state.cursor[0].y);
            break;
        case 0x0d:
        case 0x0e:
        case 0x10:
        case 0x12:
            out_word(0x3c4, 0x0f02); // seq.pmr = 0x0f
            out_word(0x3c4, 0x0604); // seq.mmr = 0x06 (ext_mem, oe_dis)

            out_word(0x3ce, 0x0005); // gc.gmr = 0x00
            out_word(0x3ce, 0x0506); // gc.mr = 0x05 (graph, oe_decode : 0, map mode : 1)
            break;
        case 0x13:
            out_word(0x3c4, 0x0f02); // seq.pmr = 0x0f
            out_word(0x3c4, 0x0e04); // seq.mmr = 0x0e (ext_mem, oe_dis, chain4)

            out_word(0x3ce, 0x4005); // gc.gmr = 0x40 (c256)
            out_word(0x3ce, 0x0506); // gc.mr = 0x01 (graph, oe_decode : 0, map mode : 1)
            break;
        default:
            reg.al = 0xff;
            return;
    }

    switch(mode) {
        case 0x00:
        case 0x01:
            state.page_size = 0x800;
            break;
        case 0x02:
        case 0x03:
            state.page_size = 0x1000;
            break;
        case 0x0d:
            state.page_size = 0x2000;
            break;
        case 0x0e:
            state.page_size = 0x4000;
            break;
        case 0x10:
            state.page_size = 0x8000;
            break;
        case 0x12:
        case 0x13:
            state.page_size = 0;
            break;
    }

    switch(mode) {
        case 0x00:
        case 0x01:
        case 0x04:
        case 0x05:
        case 0x0d:
        case 0x13:
            out_word(0x3b4, 0x2801); // crt.hdeer = 0x28
            out_word(0x3b4, 0x1912); // crt.vdeer = 0x19
            state.cols = 0x28;
            state.rows = 0x19;
            break;
        case 0x02:
        case 0x03:
        case 0x06:
        case 0x0e:
            out_word(0x3b4, 0x5001); // crt.hdeer = 0x50
            out_word(0x3b4, 0x1912); // crt.vdeer = 0x19
            state.cols = 0x50;
            state.rows = 0x19;
            break;
        case 0x0f:
        case 0x10:
            out_word(0x3b4, 0x5001); // crt.hdeer = 0x50
            out_word(0x3b4, 0x2b12); // crt.vdeer = 0x2b
            state.cols = 0x50;
            state.rows = 0x2b;
            break;
        case 0x11:
        case 0x12:
            out_word(0x3b4, 0x5001); // crt.hdeer = 0x50
            out_word(0x3b4, 0x3c12); // crt.vdeer = 0x3c
            state.cols = 0x50;
            state.rows = 0x3c;
            break;
        default:
            reg.al = 0xff;
            return;
    }

    out_word(0x3b4, 0x000c); // crt.sahr = 0
    out_word(0x3b4, 0x000d); // crt.salr = 0
    state.mode = mode;
    state.page_number = 0;
}

static void set_cursor_size(uint8_t start, uint8_t end){
	out_byte(0x3b4, 0x0a);
	out_byte(0x3b5, start);      // crt.tcsr = start
	out_byte(0x3b4, 0x0b);
	out_byte(0x3b5, end & 0x1f); // crt.tcer = end
}

static void set_cursor_position(uint8_t page, uint8_t x, uint8_t y){
    if(page > 8) return;

	reg.ax = 0;
    state.cursor[page].x = x ;
    state.cursor[page].y = y ;
    apply_cursor(x, y);
}

static void get_cursor_position(uint8_t page){
    if(page > 8) return;

	reg.ax = 0;
	out_byte(0x3b4, 0x0a);
	reg.ch = in_byte(0x3b5) & 0x1f; // crt.tcsr
	out_byte(0x3b4, 0x0b);
	reg.cl = in_byte(0x3b5) & 0x1f; // crt.tcer
    reg.dl = state.cursor[page].x;
    reg.dh = state.cursor[page].y;
}

static void set_video_page(uint8_t page){
    if(page > 8) return;

    uint16_t addr = state.page_size*page;
    state.page_number = page;

    out_byte(0x3b4, 0x0c);
    out_byte(0x3b5, addr >> 8);   // crt.sahr
    out_byte(0x3b4, 0x0d);
    out_byte(0x3b5, addr & 0xff); // crt.salr
    apply_cursor(state.cursor[page].x, state.cursor[page].y);
}

static void scroll_up(uint8_t count, uint8_t attr, uint8_t cols, uint8_t rows){
    uint16_t base = 0xa000 + ((state.page_size*state.page_number) >> 5);
    uint16_t src_idx = (uint16_t)cols*count;
    uint16_t rem_idx = (uint16_t)cols*(rows-count);

    save_regs();
    out_word(0x3c4, 0x0604); // seq.mmr = 0x06 (ext_mem, oe_dis)
    out_word(0x3ce, 0x0005); // gc.gmr = 0x00 (read/write mode : 0)
    out_word(0x3ce, 0x0006); // gc.mr = 0x00 (oe_decode : 0, map mode : 0)

    asm volatile(
        "mov ax, ds\n"
        "push ax\n"
        "mov ax, es\n"
        "push ax\n"
        "mov ds, %0\n"
        "mov es, %0\n"
    ::"r"(base):"%ax");

    out_word(0x3c4, 0x0102); // seq.pmr = 0x01 (mask only plane0)
    if(count > 0 && count < rows) {
        out_word(0x3ce, 0x0004); // gc.rpsr = 0x00 (read plane0)
        memcpy_es((void*)0, (void*)(uint32_t)src_idx, rem_idx);
        memset_es((void*)(uint32_t)rem_idx, 0, src_idx);

        out_word(0x3c4, 0x0202); // seq.pmr = 0x02 (mask only plane1)
        out_word(0x3ce, 0x0104); // gc.rpsr = 0x01 (read plane1)
        memcpy_es((void*)0, (void*)(uint32_t)src_idx, rem_idx);
        memset_es((void*)(uint32_t)rem_idx, attr, src_idx);
    }
    else {
        uint16_t len = (uint16_t)cols*rows;
        memset_es((void*)0, 0, len);
        if (count >= rows) {
            out_word(0x3c4, 0x0202); // seq.pmr = 0x02 (mask only plane1)
            memset_es((void*)0, attr, len);
        }
    }

    asm volatile(
        "pop ax\n"
        "mov es, ax\n"
        "pop ax\n"
        "mov ds, ax\n"
    );

    restore_regs();
}

static void scroll_down(uint8_t count, uint8_t attr, uint8_t cols, uint8_t rows){
    uint16_t base = 0xa000 + ((state.page_size*state.page_number) >> 5);
    uint16_t addr = (uint16_t)cols*rows;
    uint16_t src_idx = (uint16_t)cols*(rows-count);
    uint16_t rem_idx = (uint16_t)cols*count;

    save_regs();
    out_word(0x3c4, 0x0604); // seq.mmr = 0x06 (ext_mem, oe_dis)
    out_word(0x3ce, 0x0005); // gc.gmr = 0x00 (read/write mode : 0)
    out_word(0x3ce, 0x0006); // gc.mr = 0x00 (oe_decode : 0, map mode : 0)

    asm volatile(
        "mov ax, ds\n"
        "push ax\n"
        "mov ax, es\n"
        "push ax\n"
        "mov ds, %0\n"
        "mov es, %0\n"
    ::"r"(base):"%ax");

    out_word(0x3c4, 0x0102); // seq.pmr = 0x01 (mask only plane0)
    if(count > 0 && count < rows) {
        out_word(0x3ce, 0x0004); // gc.rpsr = 0x00 (read plane0)
        memcpy_es_r((void*)(uint32_t)addr, (void*)(uint32_t)src_idx, src_idx);
        memset_es((void*)0, 0, rem_idx);

        out_word(0x3c4, 0x0202); // seq.pmr = 0x02 (mask only plane1)
        out_word(0x3ce, 0x0104); // gc.rpsr = 0x01 (read plane1)
        memcpy_es_r((void*)(uint32_t)addr, (void*)(uint32_t)src_idx, src_idx);
        memset_es((void*)0, attr, rem_idx);
    }
    else {
        memset_es((void*)0, 0, addr);
        if (count >= rows) {
            out_word(0x3c4, 0x0202); // seq.pmr = 0x02 (mask only plane1)
            memset_es((void*)0, attr, addr);
        }
    }

    asm volatile(
        "pop ax\n"
        "mov es, ax\n"
        "pop ax\n"
        "mov ds, ax\n"
    );

    restore_regs();
}

static void read_char_attr(uint8_t page){
    uint16_t base = 0xb800 + ((state.page_size*state.page_number) >> 4);
    uint16_t idx = ((uint16_t)state.cols*state.cursor[page].y + state.cursor[page].x)*2;

    save_regs();
    out_word(0x3c4, 0x0302); // seq.pmr = 0x03 (mask only plane0,1)
    out_word(0x3c4, 0x0204); // seq.mmr = 0x02 (ext_mem, oe_dis : 0)
    out_word(0x3ce, 0x0005); // gc.gmr = 0x00 (read/write mode : 0)
    out_word(0x3ce, 0x0e06); // gc.mr = 0x0e (oe_decode, map mode : 3)

    asm volatile(
        "mov dx, ds\n"
        "push dx\n"
        "mov ds, %1\n"
        "lodsw\n"
        "pop dx\n"
        "mov ds, dx\n"
    :"=a"(reg.ax)
    :"r"(base),"S"(idx)
    :"%dx");

    restore_regs();
}

static void write_char_attr(uint8_t page, uint8_t chr, uint8_t attr, uint16_t count){
    uint16_t base = 0xb800 + ((state.page_size*state.page_number) >> 4);
    uint16_t idx = ((uint16_t)state.cols*state.cursor[page].y + state.cursor[page].x)*2;
    uint16_t data = ((uint16_t)attr << 8) + chr;

    save_regs();
    out_word(0x3c4, 0x0302); // seq.pmr = 0x03 (mask only plane0,1)
    out_word(0x3c4, 0x0204); // seq.mmr = 0x02 (ext_mem, oe_dis : 0)
    out_word(0x3ce, 0x0005); // gc.gmr = 0x00 (read/write mode : 0)
    out_word(0x3ce, 0x0e06); // gc.mr = 0x0e (oe_decode, map mode : 3)

    asm volatile(
        "mov dx, es\n"
        "push dx\n"
        "mov es, %0\n"
        "rep stosw\n"
        "pop dx\n"
        "mov es, dx\n"
    ::"r"(base),"a"(data),"c"(count),"D"(idx)
    :"%dx");

    restore_regs();
}

static void write_char(uint8_t page, uint8_t chr, uint16_t count){
    uint16_t base = 0xa000 + ((state.page_size*state.page_number) >> 5);
    uint16_t idx = ((uint16_t)state.cols*state.cursor[page].y + state.cursor[page].x);

    save_regs();
    out_word(0x3c4, 0x0102); // seq.pmr = 0x03 (mask only plane0)
    out_word(0x3c4, 0x0604); // seq.mmr = 0x06 (ext_mem, oe_dis)
    out_word(0x3ce, 0x0005); // gc.gmr = 0x00 (read/write mode : 0)
    out_word(0x3ce, 0x0006); // gc.mr = 0x00 (oe_decode : 0, map mode : 0)

    asm volatile(
        "mov dx, es\n"
        "push dx\n"
        "mov es, %0\n"
        "rep stosb\n"
        "pop dx\n"
        "mov es, dx\n"
    ::"r"(base),"a"(chr),"c"(count),"D"(idx)
    :"%dx");

    restore_regs();
}

static void set_color_palette(uint8_t mode, uint8_t color){
}

static void write_teletype(uint8_t page, uint8_t chr, uint8_t attr){
    uint8_t x = state.cursor[page].x;
    uint8_t y = state.cursor[page].y;
    uint16_t base = 0xb800 + ((state.page_size*state.page_number) >> 4);
    uint16_t idx = ((uint16_t)state.cols*y + x)*2;
    uint16_t data = ((uint16_t)(attr&0xf) << 8) + chr;

    save_regs();
    out_word(0x3c4, 0x0302); // seq.pmr = 0x03 (mask only plane0,1)
    out_word(0x3c4, 0x0204); // seq.mmr = 0x02 (ext_mem, oe_dis : 0)
    out_word(0x3ce, 0x0005); // gc.gmr = 0x00 (read/write mode : 0)
    out_word(0x3ce, 0x0e06); // gc.mr = 0x0e (oe_decode, map mode : 3)

    asm volatile(
        "mov dx, es\n"
        "push dx\n"
        "mov es, %0\n"
        "stosw\n"
        "pop dx\n"
        "mov es, dx\n"
    ::"r"(base),"a"(data),"D"(idx)
    :"%dx");

    restore_regs();

    if(++x >= state.cols) {
        x = 0;
        if(y+1 >= state.rows)
            scroll_up(1, 0x7, state.cols, state.rows);
        else
            y++;
    }
    state.cursor[page].x = x;
    state.cursor[page].y = y;
    apply_cursor(x, y);
}

static void get_video_mode(void){
    reg.ah = state.cols;
    reg.al = state.mode;
    reg.bh = state.page_number;
}

static void write_string(uint8_t mode, uint16_t buf, uint8_t attr, uint8_t x, uint8_t y){
    bool move_cursor = mode & 1;
    bool write_attr = (mode >> 1) & 1;

    uint16_t len = strlen_es((uint8_t*)(uint32_t)buf);
    uint16_t base = 0xa000 + ((state.page_size*state.page_number) >> 5);
    uint16_t dst_idx = state.cols*y + x;

    save_regs();
    out_word(0x3c4, 0x0102); // seq.pmr = 0x01 (mask only plane0)
    out_word(0x3c4, 0x0604); // seq.mmr = 0x06 (ext_mem, oe_dis)
    out_word(0x3ce, 0x0005); // gc.gmr = 0x00 (read/write mode : 0)
    out_word(0x3ce, 0x0006); // gc.mr = 0x00 (oe_decode : 0, map mode : 0)

    asm volatile(
        "mov ax, ds\n"
        "push ax\n"
        "mov ax, es\n"
        "push ax\n"
        "mov ds, ax\n"
        "mov es, %0\n"
    ::"r"(base):"%ax");

    memcpy_es((void*)(uint32_t)dst_idx, (void*)(uint32_t)buf, len);
    if(write_attr){
        out_word(0x3c4, 0x0202); // seq.pmr = 0x02 (mask only plane1)
        memset_es((void*)(uint32_t)dst_idx, attr, len+1);
    }

    asm volatile(
        "pop ax\n"
        "mov es, ax\n"
        "pop ax\n"
        "mov ds, ax\n"
    );
    restore_regs();

    if(move_cursor){
        dst_idx += len;
        set_cursor_position(state.page_number, dst_idx % state.cols, dst_idx / state.cols);
    }
}

static void clear_planes(void){
    out_word(0x3c4, 0x0f02); // seq.pmr = 0x0f
	out_word(0x3c4, 0x0604); // seq.mmr = 0x6 (ext_mem, oe_dis)

	out_word(0x3ce, 0x0005); // gc.gmr = 0 (read/write mode : 0)
	out_word(0x3ce, 0x0006); // gc.mr = 0 (map mode : 0)

	asm volatile(
		"mov ax, es\n"
		"push ax\n"
		"pushf\n"

		"mov ax, 0xa000\n"
		"mov es, ax\n"
	    "xor di, di\n"
	    "xor eax, eax\n"
		"mov ecx, 0x4000\n"
	    "cld\n"
	    "rep stosd\n"

		"popf\n"
		"pop ax\n"
		"mov es, ax"
	);
}

static void load_font(void){
	out_word(0x3c4, 0x0402); // seq.pmr = 0x4 (mask only plane2)
	out_word(0x3c4, 0x0604); // seq.mmr = 0x6 (ext_mem, oe_dis)

	out_word(0x3ce, 0x0005); // gc.gmr = 0 (read/write mode : 0)
	out_word(0x3ce, 0x0006); // gc.mr = 0 (text mode, map mode : 0)

	asm volatile(
		"mov ax, es\n"
		"push ax\n"
		"mov ax, 0xa000\n"
		"mov es, ax"
	);

	for(int i=0; i<0x80; i++)
		memcpy_es((void*)(i*0x20), font8x8_basic[i], 8);

	asm volatile(
		"pop ax\n"
		"mov es, ax"
	);
}

static void apply_cursor(uint8_t x, uint8_t y){
    uint16_t idx = state.cols*y + x;

    out_byte(0x3b4, 0x0e);
    out_byte(0x3b5, idx >> 8);   // crt.tclhr
    out_byte(0x3b4, 0x0f);
    out_byte(0x3b5, idx & 0xff); // crt.tcllr
}

static void save_regs(void){
    out_byte(0x3c4, 0x02);
    seq_pmr = in_byte(0x3c5);
    out_byte(0x3c4, 0x04);
    seq_mmr = in_byte(0x3c5);
    out_byte(0x3ce, 0x05);
    gc_gmr = in_byte(0x3cf);
    out_byte(0x3ce, 0x06);
    gc_mr = in_byte(0x3cf);
}

static void restore_regs(void){
    out_byte(0x3c4, 0x02);
    out_byte(0x3c5, seq_pmr);
    out_byte(0x3c4, 0x04);
    out_byte(0x3c5, seq_mmr);
    out_byte(0x3ce, 0x05);
    out_byte(0x3cf, gc_gmr);
    out_byte(0x3ce, 0x06);
    out_byte(0x3cf, gc_mr);
}