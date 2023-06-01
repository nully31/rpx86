//
// Instructions
//
use super::*;

pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg = emu.get_code8(0) - 0xB8;
    let value = emu.get_code32(1);
    let reg = emu.get_register_id(reg).unwrap_or_else(|| {
        panic!("Invalid register id");
    });  // TODO: implement better error handling
    emu.set_register(reg, value);
    emu.set_eip(emu.get_eip() + 5);
}

pub fn short_jump(emu: &mut Emulator) {
    let diff = emu.get_signed_code8(1) as i8;
    emu.set_eip((emu.get_eip() as i32 + diff as i32 + 2) as u32);
}
