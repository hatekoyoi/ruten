NASM=nasm

all:

# 実行する場合, プログラム終了条件をemu.eip == 0x7c00にする
short_jmp:
	$(NASM) -f bin -o test/short_jmp.bin test/short_jmp.asm
	cargo run test/short_jmp.bin

near_jmp:
	$(NASM) -f bin -o test/near_jmp.bin test/near_jmp.asm
	cargo run test/near_jmp.bin

clean:
	rm -f test/*.bin
	cargo clean
