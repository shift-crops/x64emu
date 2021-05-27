#ifndef _UTILS_H
#define _UTILS_H

#include <stdint.h>

void store_esb(uint8_t *addr, uint8_t v);
void store_esw(uint16_t *addr, uint16_t v);
void store_esd(uint32_t *addr, uint32_t v);
uint8_t load_esb(uint8_t *addr);
uint16_t load_esw(uint16_t *addr);
uint32_t load_esd(uint32_t *addr);
void memcpy_es(void *daddr, void *saddr, uint16_t len);
void memset_es(void *addr, uint8_t c, uint16_t len);
uint16_t strlen_es(uint8_t *str);

uint8_t in_byte(uint16_t port);
void out_byte(uint16_t port, uint8_t v);
uint16_t in_word(uint16_t port);
void out_word(uint16_t port, uint16_t v);

void cli(void);
void sti(void);

#endif
