PREFIX  = riscv64-unknown-elf-
CC      = $(PREFIX)gcc
OBJCOPY = $(PREFIX)objcopy

# start from 0x0 address
CFLAGS  = -Wl,-Ttext=0x0 -nostdlib

#make bin file
%.bin: %.s
	$(CC) $(CFLAGS) -march=rv64i -o $* $<
	$(OBJCOPY) -O binary $* $@

.PHONY: clean
clean:
	rm -f *.bin