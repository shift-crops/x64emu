#ifndef _REGS_H
#define _REGS_H

#include <stdint.h>

struct reg16_t {
    union {
        uint16_t ax;
        struct {
            uint8_t al;
            uint8_t ah;
        };
    };

    union {
        uint16_t cx;
        struct {
            uint8_t cl;
            uint8_t ch;
        };
    };

    union {
        uint16_t dx;
        struct {
            uint8_t dl;
            uint8_t dh;
        };
    };

    union {
        uint16_t bx;
        struct {
            uint8_t bl;
            uint8_t bh;
        };
    };

    uint16_t bp;
};
extern struct reg16_t reg;

#endif