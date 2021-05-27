#include <stdint.h>
#include "utils.h"
#include "service.h"

typedef struct {
	uint16_t offset;
	uint16_t segment;
} IVT;

static void set_ivt(int n, uint32_t offset, uint16_t cs);

void init_ivt(void){
	set_ivt(0x10, (uint32_t)bsv_video, 0xf000);
}

static void set_ivt(int n, uint32_t offset, uint16_t cs){
	IVT* ivt = (IVT*)0;

	store_esw(&(ivt[n].offset), offset);
	store_esw(&(ivt[n].segment), cs);
}