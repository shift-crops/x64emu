TARGET   := bios.bin
OBJS     := startup.o utils.o
SUB_OBJS := init/init.a service/service.a

CC 	     := gcc
AS 	     := nasm
CFLAGS   += -I./include -masm=intel -nostdlib -fno-asynchronous-unwind-tables -fcf-protection=none -fno-stack-protector -fno-pie -m16
LDFLAGS  += --entry=start --oformat=binary -Ttext 0x0 -melf_i386

all : $(TARGET) crt0.bin

%.o : %.c
	$(CC) $(CFLAGS) -c $<

%.o : %.asm
	$(AS) -f elf $<

crt0.bin : crt0.asm
	$(AS) $< -o $@

$(TARGET) : $(OBJS) $(SUB_OBJS)
	$(LD) $(LDFLAGS) -o $@ $(OBJS) $(SUB_OBJS)

$(SUB_OBJS): FORCE
	$(MAKE) -C $(dir $@) $(notdir $@)

clean:
	$(RM) $(OBJS) $(TARGET) crt0.bin
	dirname $(SUB_OBJS) | xargs -l $(MAKE) clean -C

FORCE: