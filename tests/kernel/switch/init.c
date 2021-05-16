#include <stdint.h>

struct PML4E {
	uint32_t P : 1;
	uint32_t RW : 1;
	uint32_t US : 1;
	uint32_t PWT : 1;
	uint32_t PCD : 1;
	uint32_t A : 1;
	uint32_t : 6;
	uint32_t pdpt_base : 20;
};

struct PDPTE {
	uint32_t P : 1;
	uint32_t RW : 1;
	uint32_t US : 1;
	uint32_t PWT : 1;
	uint32_t PCD : 1;
	uint32_t A : 1;
	uint32_t : 1;
	uint32_t PS : 1;
	uint32_t G : 1;
	uint32_t : 3;
	uint32_t pdt_base : 20;
};

struct PDE {
	uint32_t P : 1;
	uint32_t RW : 1;
	uint32_t US : 1;
	uint32_t PWT : 1;
	uint32_t PCD : 1;
	uint32_t A : 1;
	uint32_t : 1;
	uint32_t PS : 1;
	uint32_t G : 1;
	uint32_t : 3;
	uint32_t pt_base : 20;
};

struct PTE {
	uint32_t P : 1;
	uint32_t RW : 1;
	uint32_t US : 1;
	uint32_t PWT : 1;
	uint32_t PCD : 1;
	uint32_t A : 1;
	uint32_t D : 1;
	uint32_t PAT : 1;
	uint32_t G : 1;
	uint32_t : 3;
	uint32_t page_base : 20;
};

uint32_t init_page_long(void){
	struct PML4E *pml4 = (struct PML4E*)0x20000;
	struct PDPTE *pdpt = (struct PDPTE*)0x21000;

	struct PTE *pte = (struct PTE*)0x21000;

	pml4[0].pdpt_base = ((uint32_t)pdpt)>>12;
	pml4[0].P = 1;
	pml4[0].RW = 1;
	pml4[0].US = 1;

	pdpt[0].pdt_base = 0;
	pdpt[0].P = 1;
	pdpt[0].RW = 1;
	pdpt[0].US = 1;
	pdpt[0].PS = 1;

	return (uint32_t)pml4;
}
