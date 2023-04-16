use crate::emulator::*;

pub fn get_code8(emu: &mut Emulator, index: usize) -> u8 {
    return emu.memory[emu.eip + index];
}

pub fn get_sign_code8(emu: &mut Emulator, index: usize) -> i8 {
    return emu.memory[emu.eip + index] as i8;
}

pub fn get_code32(emu: &mut Emulator, index: usize) -> u32 {
    let mut ret: u32 = 0;

    // リトルエンディアンでメモリの値を取得する
    for i in 0..4 {
        ret |= (get_code8(emu, index + i) as u32) << (i * 8);
    }
    return ret;
}

pub fn get_sign_code32(emu: &mut Emulator, index: usize) -> i32 {
    return get_code32(emu, index) as i32;
}
