use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

// メモリは1MB
const MEMORY_SIZE: usize = 1024 * 1024;

pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    RegisterCount,
}

const REGISTERS_NAME: [&str; 8] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

struct Emulator {
    registers: [u32; Register::RegisterCount as usize],
    eflags: u32,
    memory: Vec<u8>,
    eip: usize,
}

fn create_emu(size: usize, eip: usize, esp: u32) -> Emulator {
    let mut emu = Emulator {
        registers: [0; Register::RegisterCount as usize],
        eflags: 0,
        memory: vec![0; size],
        eip,
    };
    emu.registers[Register::ESP as usize] = esp;
    emu
}

fn read_to_memory(file: &mut File, emu: &mut Emulator) {
    let mut index = 0;
    for byte in file.bytes() {
        emu.memory[index + 0x7c00] = match byte {
            Ok(b) => b,
            Err(e) => {
                eprintln!("バイトの読み取り中にエラーが発生しました: {}", e);
                process::exit(1);
            }
        };
        index += 1;
    }
}

fn get_code8(emu: &mut Emulator, index: usize) -> u8 {
    return emu.memory[emu.eip + index];
}

fn get_sign_code8(emu: &mut Emulator, index: usize) -> i8 {
    return emu.memory[emu.eip + index] as i8;
}

fn get_code32(emu: &mut Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;

    // リトルエンディアンでメモリの値を取得する
    for i in 0..4 {
        ret |= (get_code8(emu, index + i) as u32) << (i * 8);
    }
    return ret;
}

fn mov_r32_imm32(emu: &mut Emulator) {
    let reg = get_code8(emu, 0) - 0xB8;
    let value = get_code32(emu, 1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}

fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1);
    emu.eip = emu.eip.wrapping_add((diff + 2) as usize);
}

fn dump_registers(emu: &mut Emulator) {
    for i in 0..Register::RegisterCount as usize {
        println!("{} = {:#010X}", REGISTERS_NAME[i], emu.registers[i]);
    }
    println!("EIP = {:#010X}", emu.eip);
}

type InstructionFuncT = fn(&mut Emulator);
type Instructions = [InstructionFuncT; 256];
fn init_instructions(instructions: &mut Instructions) {
    for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
    }
    instructions[0xEB] = short_jump;
}
fn undefined_func(_emu: &mut Emulator) {}

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
        println!("EIP = {:#010X}, Code = {:#04X}", emu.eip, code);

        if instructions[code] as usize == undefined_func as usize {
            println!("\nNot Implemented: {:#04X}\n", code);
            break;
        }

        instructions[code](&mut emu);

        if emu.eip == 0x7c00 {
            println!("\nend of program.\n");
            break;
        }
    }

    dump_registers(&mut emu);
}
