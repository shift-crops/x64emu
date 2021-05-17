TARGET  := ipl.bin
SAMPLE  := mode/kernel.bin task/kernel.bin

AS      := nasm

all : $(TARGET) $(SAMPLE)

$(TARGET) : ipl.asm
	$(AS) $< -o $@

$(SAMPLE) : FORCE
	$(MAKE) -C $(dir $@) $(notdir $@)

clean :
	$(RM) $(TARGET)
	dirname $(SAMPLE) | xargs -l $(MAKE) clean -C

FORCE:
