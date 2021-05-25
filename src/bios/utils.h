#ifndef _UTILS_H
#define _UTILS_H

#include <stdint.h>

void write_esb(uint8_t *addr, uint8_t v);
void write_esw(uint16_t *addr, uint16_t v);
void write_esd(uint32_t *addr, uint32_t v);
void memcpy_es(void *daddr, void *saddr, uint16_t len);

uint8_t in_byte(uint16_t port);
void out_byte(uint16_t port, uint8_t v);
uint16_t in_word(uint16_t port);
void out_word(uint16_t port, uint16_t v);

void cli(void);
void sti(void);

#endif
