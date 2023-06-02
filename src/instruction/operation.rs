//
// Instructions
//
use super::*;
use crate::emulator::modrm::ModRM;


pub fn mov_r32_imm32(emu: &mut Emulator) {
    let reg = emu.get_code8(0) - 0xB8;
    let value = emu.get_code32(1);
    let reg = *emu.get_gpr_id(reg.into()).unwrap_or_else(|| {
        panic!("Invalid register id");
    });  // TODO: implement better error handling
    emu.set_gpr(&reg, value);
    emu.inc_eip(5);
}

pub fn short_jump(emu: &mut Emulator) {
    let diff = emu.get_signed_code8(1);
    emu.set_eip((emu.get_eip() as i32 + diff as i32 + 2) as u32);
}

pub fn near_jump(emu: &mut Emulator) {
    let diff = emu.get_signed_code32(1);
    emu.set_eip((emu.get_eip() as i32 + diff + 5) as u32);
}

pub fn mov_rm32_imm32(emu: &mut Emulator) {
    emu.inc_eip(1);
    let mut modrm = ModRM::new();
    modrm.parse_modrm(emu);
    let value = emu.get_code32(0);
    emu.inc_eip(4);
    modrm.set_rm32(emu, value);
}