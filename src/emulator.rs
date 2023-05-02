use std::fs::File;
use std::io::Read;
use std::process;

#[allow(dead_code)]
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

pub const REGISTERS_NAME: [&str; 8] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

pub struct Emulator {
    pub registers: [u32; Register::RegisterCount as usize],
    pub eflags: u32,
    pub memory: Vec<u8>,
    pub eip: usize,
}

pub fn create_emu(size: usize, eip: usize, esp: u32) -> Emulator {
    let mut emu = Emulator {
        registers: [0; Register::RegisterCount as usize],
        eflags: 0,
        memory: vec![0; size],
        eip,
    };
    emu.registers[Register::ESP as usize] = esp;
    emu
}

pub fn read_to_memory(file: &mut File, emu: &mut Emulator) {
    let mut index = 0;
    for byte in file.bytes() {
        emu.memory[index + 0x7c00] = match byte {
            Ok(b) => b,
            Err(e) => {
                eprintln!("バイトの読み取り中にエラーが発生しました: {}", e);
                process::exit(1)
            }
        };
        index += 1;
    }
}

pub fn dump_registers(emu: &mut Emulator) {
    for i in 0..Register::RegisterCount as usize {
        println!("{} = {:#010X}", REGISTERS_NAME[i], emu.registers[i]);
    }
    println!("EIP = {:#010X}", emu.eip);
}
