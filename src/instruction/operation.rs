//
// Instructions
//
use super::*;
use crate::emulator::GPR;
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
    let mut modrm = ModRM::new(emu);
    modrm.parse_modrm(emu);
    let value = emu.get_code32(0);
    emu.inc_eip(4);
    modrm.set_rm32(emu, value);
}

pub fn mov_rm32_r32(emu: &mut Emulator) {
    emu.inc_eip(1);
    let mut modrm = ModRM::new(emu);
    modrm.parse_modrm(emu);
    modrm.set_rm32(emu, modrm.get_r32(emu));
}

pub fn mov_r32_rm32(emu: &mut Emulator) {
    emu.inc_eip(1);
    let mut modrm = ModRM::new(emu);
    modrm.parse_modrm(emu);
    modrm.set_r32(emu, modrm.get_rm32(emu));
}

pub fn add_rm32_r32(emu: &mut Emulator) {
    emu.inc_eip(1);
    let mut modrm = ModRM::new(emu);
    modrm.parse_modrm(emu);
    let r32 = modrm.get_r32(emu);
    let rm32 = modrm.get_rm32(emu);
    modrm.set_rm32(emu, r32 + rm32);
}

pub fn code_83(emu: &mut Emulator) {
    emu.inc_eip(1);
    let mut modrm = ModRM::new(emu);
    modrm.parse_modrm(emu);

    match modrm.get_opcode() {
        0b101 => sub_rm32_imm8(emu, &modrm),
        _ => panic!("Not implemented: code 83 , {:#x?}", modrm),
    }
}

pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = modrm.get_rm32(emu);
    let imm8 = emu.get_signed_code8(0);
    emu.inc_eip(1);
    modrm.set_rm32(emu, rm32 - imm8 as u32);
}

pub fn code_ff(emu: &mut Emulator) {
    emu.inc_eip(1);
    let mut modrm = ModRM::new(emu);
    modrm.parse_modrm(emu);

    match modrm.get_opcode() {
        0b000 => inc_rm32(emu, &modrm),
        _ => panic!("Not implemented: code FF , {:#x?}", modrm),
    }
}

pub fn inc_rm32(emu: &mut Emulator, modrm: &ModRM) {
    modrm.set_rm32(emu, modrm.get_rm32(emu) + 1);
}

pub fn push_r32(emu: &mut Emulator) {
    let reg = emu.get_code8(0) - 0x50;
    let value = *emu.get_gpr_value(emu.get_gpr_id(reg.into()).unwrap());
    push32(emu, value);
    emu.inc_eip(1);
}

pub fn push32(emu: &mut Emulator, value: u32) {
    let address = *emu.get_gpr_value(&GPR::ESP) - 4;
    emu.set_gpr(&GPR::ESP, address);
    emu.set_memory32(address, value);
}

pub fn pop_r32(emu: &mut Emulator) {
    let reg = emu.get_code8(0) - 0x58;
    let reg = *emu.get_gpr_id(reg.into()).unwrap();
    let popped = pop32(emu);
    emu.set_gpr(&reg, popped);
    emu.inc_eip(1);
}

pub fn pop32(emu: &mut Emulator) -> u32 {
    let address = *emu.get_gpr_value(&GPR::ESP);
    let ret = emu.get_memory32(address);
    emu.set_gpr(&GPR::ESP, address + 4);
    ret
}

pub fn call_rel32(emu: &mut Emulator) {
    let diff = emu.get_signed_code32(1);
    push32(emu, emu.get_eip() + 5);
    emu.inc_eip(diff as u32 + 5);
}

pub fn ret(emu: &mut Emulator) {
    let popped = pop32(emu);
    emu.set_eip(popped);
}

pub fn leave(emu: &mut Emulator) {
    let ebp = emu.get_gpr_value(&GPR::EBP);
    emu.set_gpr(&GPR::ESP, *ebp);
    let popped = pop32(emu);
    emu.set_gpr(&GPR::EBP, popped);
    emu.inc_eip(1);
}