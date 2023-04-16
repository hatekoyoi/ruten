use crate::emulator::*;
use crate::function::*;

fn mov_r32_imm32(emu: &mut Emulator) {
    let reg = get_code8(emu, 0) - 0xB8;
    let value = get_code32(emu, 1);
    emu.registers[reg as usize] = value;
    emu.eip += 5;
}

fn near_jump(emu: &mut Emulator) {
    let diff = get_sign_code32(emu, 1);
    emu.eip = emu.eip.wrapping_add((diff + 5) as usize);
}

fn short_jump(emu: &mut Emulator) {
    let diff = get_sign_code8(emu, 1);
    emu.eip = emu.eip.wrapping_add((diff + 2) as usize);
}

pub type InstructionFuncT = fn(&mut Emulator);
pub type Instructions = [InstructionFuncT; 256];
pub fn undefined_func(_emu: &mut Emulator) {}
pub fn init_instructions(instructions: &mut Instructions) {
    for i in 0..8 {
        instructions[0xB8 + i] = mov_r32_imm32;
    }
    instructions[0xE9] = near_jump;
    instructions[0xEB] = short_jump;
}
