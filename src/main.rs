mod emulator;
mod function;
mod instruction;

use emulator::*;
use function::*;
use instruction::*;
use std::env;
use std::fs::File;
use std::process;

// メモリは1MB
const MEMORY_SIZE: usize = 1024 * 1024;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: ruten filename");
        process::exit(1);
    }

    // EIPが0x7c00, ESPが0x7c00の状態のエミュレータを作る
    let mut emu = create_emu(MEMORY_SIZE, 0x7c00, 0x7c00);

    let mut file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("ファイルが開けません: {}", e);
            process::exit(1);
        }
    };

    // 機械語ファイルを読み込む
    read_to_memory(&mut file, &mut emu);

    let mut instructions: Instructions = [undefined_func; 256];
    init_instructions(&mut instructions);

    while emu.eip < MEMORY_SIZE {
        let code = get_code8(&mut emu, 0) as usize;
        // プログラムカウンタと実行されるバイナリを表示
        println!("EIP = {:#010X}, Code = {:#04X}", emu.eip, code);

        if instructions[code] as usize == undefined_func as usize {
            println!("\nNot Implemented: {:#04X}\n", code);
            break;
        }

        // 命令実行
        instructions[code](&mut emu);

        // EIPが0になったらプログラムを終了
        if emu.eip == 0 {
            println!("\nend of program.\n");
            break;
        }
    }

    dump_registers(&mut emu);
}
