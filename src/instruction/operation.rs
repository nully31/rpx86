//
// Instructions
//
use super::*;
use crate::emulator::{GPR, GPR8};
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
        0b000 => add_rm32_imm8(emu, &modrm),
        0b101 => sub_rm32_imm8(emu, &modrm),
        0b111 => cmp_rm32_imm8(emu, &modrm),
        _ => panic!("Not implemented: code 83 , {:#x?}", modrm),
    }
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
    let value = emu.get_gpr_value(emu.get_gpr_id(reg.into()).unwrap());
    push32(emu, value);
    emu.inc_eip(1);
}

fn push32(emu: &mut Emulator, value: u32) {
    let address = emu.get_gpr_value(&GPR::ESP) - 4;
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

fn pop32(emu: &mut Emulator) -> u32 {
    let address = emu.get_gpr_value(&GPR::ESP);
    let ret = emu.get_memory32(address);
    emu.set_gpr(&GPR::ESP, address + 4);
    ret
}

pub fn call_rel32(emu: &mut Emulator) {
    let diff = emu.get_signed_code32(1);
    push32(emu, emu.get_eip() + 5);
    emu.inc_eip(diff + 5);
}

pub fn ret(emu: &mut Emulator) {
    let popped = pop32(emu);
    emu.set_eip(popped);
}

pub fn leave(emu: &mut Emulator) {
    let ebp = emu.get_gpr_value(&GPR::EBP);
    emu.set_gpr(&GPR::ESP, ebp);
    let popped = pop32(emu);
    emu.set_gpr(&GPR::EBP, popped);
    emu.inc_eip(1);
}

pub fn push_imm8(emu: &mut Emulator) {
    let value = emu.get_code8(1);
    push32(emu, value as u32);
    emu.inc_eip(2);
}

pub fn push_imm32(emu: &mut Emulator) {
    let value = emu.get_code32(1);
    push32(emu, value);
    emu.inc_eip(5);
}

pub fn add_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = modrm.get_rm32(emu);
    let imm8 = emu.get_signed_code8(0) as i32;
    emu.inc_eip(1);
    modrm.set_rm32(emu, rm32 + imm8 as u32);
}

pub fn cmp_r32_rm32(emu: &mut Emulator) {
    emu.inc_eip(1);
    let mut modrm = ModRM::new(emu);
    modrm.parse_modrm(emu);
    let r32 = modrm.get_r32(emu);
    let rm32 = modrm.get_rm32(emu);
    let result = r32 as i64 - rm32 as i64;
    emu.update_eflags_sub(r32, rm32, result as u64);
}

pub fn cmp_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = modrm.get_rm32(emu);
    let imm8 = emu.get_signed_code8(0) as i32;
    emu.inc_eip(1);
    let result = rm32 as u64 - imm8 as u64;
    emu.update_eflags_sub(rm32, imm8 as u32, result);
}

pub fn sub_rm32_imm8(emu: &mut Emulator, modrm: &ModRM) {
    let rm32 = modrm.get_rm32(emu);
    let imm8 = emu.get_signed_code8(0) as i32;
    emu.inc_eip(1);
    let result = rm32 as u64 - imm8 as u64;
    modrm.set_rm32(emu, result as u32);
    emu.update_eflags_sub(rm32, imm8 as u32, result);
}

pub fn jc(emu: &mut Emulator) {
    let diff = if emu.is_carry() {
        emu.get_signed_code8(1)
    } else {
        0
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn jnc(emu: &mut Emulator) {
    let diff = if emu.is_carry() {
        0
    } else {
        emu.get_signed_code8(1)
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn jz(emu: &mut Emulator) {
    let diff = if emu.is_zero() {
        emu.get_signed_code8(1)
    } else {
        0
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn jnz(emu: &mut Emulator) {
    let diff = if emu.is_zero() {
        0
    } else {
        emu.get_signed_code8(1)
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn js(emu: &mut Emulator) {
    let diff = if emu.is_signed() {
        emu.get_signed_code8(1)
    } else {
        0
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn jns(emu: &mut Emulator) {
    let diff = if emu.is_signed() {
        0
    } else {
        emu.get_signed_code8(1)
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn jo(emu: &mut Emulator) {
    let diff = if emu.is_overflow() {
        emu.get_signed_code8(1)
    } else {
        0
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn jno(emu: &mut Emulator) {
    let diff = if emu.is_overflow() {
        0
    } else {
        emu.get_signed_code8(1)
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn jl(emu: &mut Emulator) {
    let diff = if emu.is_signed() != emu.is_overflow() {
        emu.get_signed_code8(1)
    } else {
        0
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn jle(emu: &mut Emulator) {
    let diff = if emu.is_zero() || (emu.is_signed() != emu.is_overflow()) {
        emu.get_signed_code8(1)
    } else {
        0
    };
    emu.inc_eip(diff as i32 + 2);
}

pub fn in_al_dx(emu: &mut Emulator) {
    let address: u16 = (emu.get_gpr_value(&GPR::EDX) & 0xffff) as u16;
    let value: u8 = io::in8(address);
    emu.set_gpr8(&GPR8::AL, value);
    emu.inc_eip(1);
}

pub fn out_dx_al(emu: &mut Emulator) {
    let address = (emu.get_gpr_value(&GPR::EDX) & 0xffff) as u16;
    let value = emu.get_gpr8_value(&GPR8::AL);
    io::out8(address, value);
    emu.inc_eip(1);
}