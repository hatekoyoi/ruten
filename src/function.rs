use crate::emulator::*;

pub fn get_code8(emu: &mut Emulator, index: usize) -> u8 {
    emu.memory[emu.eip + index]
}

pub fn get_sign_code8(emu: &mut Emulator, index: usize) -> i8 {
    emu.memory[emu.eip + index] as i8
}

pub fn get_code32(emu: &mut Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;

    // リトルエンディアンでメモリの値を取得する
    for i in 0..4 {
        ret |= (get_code8(emu, index + i) as u32) << (i * 8);
    }
    ret
}

pub fn get_sign_code32(emu: &mut Emulator, index: usize) -> i32 {
    get_code32(emu, index) as i32
}

pub fn get_register32(emu: &mut Emulator, index: usize) -> u32 {
    emu.registers[index]
}

pub fn set_register32(emu: &mut Emulator, index: usize, value: u32) {
    emu.registers[index] = value;
}

pub fn set_memory8(emu: &mut Emulator, address: u32, value: u32) {
    emu.memory[address as usize] = (value & 0xFF) as u8;
}

pub fn set_memory32(emu: &mut Emulator, address: u32, value: u32) {
    for i in 0..4 {
        set_memory8(emu, address + i, value >> (i * 8));
    }
}

pub fn get_memory8(emu: &mut Emulator, address: u32) -> u8 {
    emu.memory[address as usize]
}

pub fn get_memory32(emu: &mut Emulator, address: u32) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..4 {
        ret |= (get_memory8(emu, address + i) as u32) << (8 * i);
    }
    ret
}

pub fn push32(emu: &mut Emulator, value: u32) {
    let address = get_register32(emu, Register::ESP as usize) - 4;
    set_register32(emu, Register::ESP as usize, address);
    set_memory32(emu, address, value);
}

pub fn pop32(emu: &mut Emulator) -> u32 {
    let address = get_register32(emu, Register::ESP as usize);
    let ret = get_memory32(emu, address) as u32;
    set_register32(emu, Register::ESP as usize, address + 4);
    ret
}
