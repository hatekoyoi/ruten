NASM=nasm
GCC=gcc
LD=ld

all:

# 実行する場合, プログラム終了条件をemu.eip == 0x7c00にする
# short-jmp:
# 	$(NASM) -f bin -o test/short-jmp.bin test/short-jmp.asm
# 	cargo run test/short-jmp.bin

near-jmp:
	$(NASM) -f bin -o test/near-jmp.bin test/near-jmp.asm
	cargo run test/near-jmp.bin

modrm-test:
	$(NASM) -f bin -o test/modrm-test.bin test/modrm-test.asm
	cargo run test/modrm-test.bin

call-test:
	$(NASM) -f bin -o test/call-test.bin test/call-test.asm
	cargo run test/call-test.bin

leave-test:
	$(NASM) -f elf test/crt0.asm -o test/crt0.o
	$(GCC) -O0 -march=i386 -m32 -nostdlib -fno-asynchronous-unwind-tables -g -fno-stack-protector -fno-pie -c -o test/leave-test.o test/leave-test.c
	$(LD) -m elf_i386 --entry=start --oformat=binary -Ttext 0x7c00 -o test/leave-test.bin test/crt0.o test/leave-test.o
	cargo run test/leave-test.bin

leave-test2:
	$(NASM) -f elf test/crt0.asm -o test/crt0.o
	$(GCC) -O0 -march=i386 -m32 -nostdlib -fno-asynchronous-unwind-tables -g -fno-stack-protector -fno-pie -c -o test/leave-test2.o test/leave-test2.c
	$(LD) -m elf_i386 --entry=start --oformat=binary -Ttext 0x7c00 -o test/leave-test2.bin test/crt0.o test/leave-test2.o
	cargo run test/leave-test2.bin

clean:
	rm -f test/*.bin
	cargo clean
