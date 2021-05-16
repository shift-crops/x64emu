TARGET   := kernel.bin

SRCS     := startup.s switch.s init.c
CSRCS   := $(filter %.c,$(SRCS))
SSRCS   := $(filter %.s,$(SRCS))

OBJS    := $(SSRCS:.s=.o) $(CSRCS:.c=.o)

CC      := gcc
AS      := nasm
CFLAGS  += -masm=intel -nostdlib -fno-asynchronous-unwind-tables -fcf-protection=none -fno-stack-protector -fno-pie -m32
LDFLAGS := --entry=start --oformat=binary -Ttext 0x7e00 -melf_i386

all : $(TARGET)

%.o : %.c
	$(CC) $(CFLAGS) -c $<

%.o : %.asm
	$(AS) -f elf $<

$(TARGET) : $(OBJS)
	$(LD) $(LDFLAGS) -o $@ $(OBJS)

clean :
	$(RM) $(OBJS) $(TARGET)
