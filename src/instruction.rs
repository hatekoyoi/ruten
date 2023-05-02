use std::process;

use crate::emulator::*;
use crate::function::*;
use crate::modrm::*;

fn add_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = parse_modrm(emu);
    let r32: u32 = get_r32(emu, &mut modrm);
    let rm32: u32 = get_rm32(emu, &mut modrm);
    set_rm32(emu, &mut modrm, r32 + rm32);
}

fn push_r32(emu: &mut Emulator) {
    let reg = (get_code8(emu, 0) - 0x50) as usize;
    let value = get_register32(emu, reg);
    push32(emu, value);
    emu.eip += 1;
}

fn pop_r32(emu: &mut Emulator) {
    let reg = (get_code8(emu, 0) - 0x58) as usize;
    let value = pop32(emu);
    set_register32(emu, reg, value);
    emu.eip += 1;
}

fn push_imm32(emu: &mut Emulator) {
    let value = get_code32(emu, 1);
    push32(emu, value);
    emu.eip += 5;
}

fn push_imm8(emu: &mut Emulator) {
    let value = get_code8(emu, 1) as u32;
    push32(emu, value);
    emu.eip += 2;
}

fn code_83(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = parse_modrm(emu);
    match modrm.opecode {
        0 => add_rm32_imm8(emu, &mut modrm),
        5 => sub_rm32_imm8(emu, &mut modrm),
        _ => {
            println!("not implemented: 83 /{}", modrm.opecode);
            process::exit(1);
        }
    }
}

fn add_rm32_imm8(emu: &mut Emulator, modrm: &mut ModRM) {
    let rm32 = get_rm32(emu, modrm);
    let imm8 = get_sign_code8(emu, 0);
    emu.eip += 1;
    set_rm32(emu, modrm, rm32 + imm8 as u32);
}

fn sub_rm32_imm8(emu: &mut Emulator, modrm: &mut ModRM) {
    let rm32: u32 = get_rm32(emu, modrm);
    let imm8: u32 = get_sign_code8(emu, 0) as u32;
    emu.eip += 1;
    set_rm32(emu, modrm, rm32 - imm8);
}

fn mov_rm32_r32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = parse_modrm(emu);
    let r32: u32 = get_r32(emu, &mut modrm);
    set_rm32(emu, &mut modrm, r32);
}

fn mov_r32_rm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = parse_modrm(emu);
    let rm32 = get_rm32(emu, &mut modrm);
    set_r32(emu, &mut modrm, rm32);
}

fn mov_r32_imm32(emu: &mut Emulator) {
    let reg = get_code8(emu, 0) - 0xB8;
    let value = get_code32(emu, 1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}

fn ret(emu: &mut Emulator) {
    emu.eip = pop32(emu) as usize;
}

fn leave(emu: &mut Emulator) {
    let ebp = get_register32(emu, Register::EBP as usize);
    let value = pop32(emu);
    set_register32(emu, Register::ESP as usize, ebp);
    set_register32(emu, Register::EBP as usize, value);
    emu.eip += 1;
}

fn call_rel32(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1);
    push32(emu, (emu.eip + 5) as u32);
    emu.eip = emu.eip.wrapping_add((diff + 5) as usize);
}

fn mov_rm32_imm32(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = parse_modrm(emu);
    let value = get_code32(emu, 0);
    emu.eip += 4;
    set_rm32(emu, &mut modrm, value);
}

fn near_jump(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1);
    emu.eip = emu.eip.wrapping_add((diff + 5) as usize);
}

fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1);
    emu.eip = emu.eip.wrapping_add((diff + 2) as usize);
}

fn code_ff(emu: &mut Emulator) {
    emu.eip += 1;
    let mut modrm = parse_modrm(emu);
    match modrm.opecode {
        0 => inc_rm32(emu, &mut modrm),
        _ => {
            println!("not implemented: FF /{}", modrm.opecode);
            process::exit(1);
        }
    }
}

fn inc_rm32(emu: &mut Emulator, modrm: &mut ModRM) {
    let value: u32 = get_rm32(emu, modrm);
    set_rm32(emu, modrm, value + 1);
}

pub type InstructionFuncT = fn(&mut Emulator);
pub type Instructions = [InstructionFuncT; 256];
pub fn undefined_func(_emu: &mut Emulator) {}
pub fn init_instructions(instructions: &mut Instructions) {
    instructions[0x01] = add_rm32_r32;
    for i in 0..8 {
        instructions[0x50 + i] = push_r32;
    }
    for i in 0..8 {
        instructions[0x58 + i] = pop_r32;
    }
    instructions[0x68] = push_imm32;
    instructions[0x6A] = push_imm8;
    instructions[0x83] = code_83;
    instructions[0x89] = mov_rm32_r32;
    instructions[0x8B] = mov_r32_rm32;
    for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
    }
    instructions[0xC3] = ret;
    instructions[0xC7] = mov_rm32_imm32;
    instructions[0xC9] = leave;
    instructions[0xE8] = call_rel32;
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
    instructions[0xFF] = code_ff;
}
