TARGET  := init.a
OBJS    := vga.o

CC      := gcc
AS      := nasm
CFLAGS  += -masm=intel -nostdlib -fno-asynchronous-unwind-tables -fcf-protection=none -fno-stack-protector -fno-pie -m16

all : $(TARGET)

%.o : %.c
	$(CC) $(CFLAGS) -c $<

%.o : %.asm
	$(AS) -f elf $<

$(TARGET) : $(OBJS)
	$(AR) rcs $@ $^

clean :
	$(RM) $(OBJS) $(TARGET)