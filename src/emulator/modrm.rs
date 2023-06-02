use std::fmt::Display;

//
// ModR/M
//
use super::*;

#[derive(Debug)]
pub struct Disp {
    disp8: Option<i8>,
    disp32: Option<i32>,
}

#[derive(Debug)]
pub struct ModRM {
    r#mod: u8,
    reg_bit: u8,
    rm: u8,

    sib: u8,
    disp: Disp,
}

impl ModRM {
    pub fn new() -> ModRM {
        ModRM {
            r#mod: 0x0,
            reg_bit: 0x0,
            rm: 0x0,
            sib: 0x0,
            disp: Disp {
                disp8: None,
                disp32: None,
            },
        }
    }

    pub fn get_mod(&self) -> u8 {
        self.r#mod
    }

    pub fn get_opcode(&self) -> u8 {
        self.reg_bit
    }

    pub fn get_reg_index(&self) -> u8 {
        self.reg_bit
    }

    pub fn get_rm(&self) -> u8 {
        self.rm
    }

    pub fn get_sib(&self) -> u8 {
        self.sib
    }

    pub fn get_disp8(&self) -> Option<i8> {
        self.disp.disp8
    }

    pub fn get_disp32(&self) -> Option<i32> {
        self.disp.disp32
    }

    pub fn set_mod(&mut self, new_value: u8) {
        self.r#mod = new_value;
    }

    pub fn set_rm(&mut self, new_value: u8) {
        self.rm = new_value;
    }

    pub fn set_sib(&mut self, new_value: u8) {
        self.sib = new_value;
    }

    pub fn set_regbit(&mut self, new_value: u8) {
        self.reg_bit = new_value;
    }

    pub fn set_disp8(&mut self, disp: i8) {
        self.disp.disp8 = Some(disp);
    }

    pub fn set_disp32(&mut self, disp: i32) {
        self.disp.disp32 = Some(disp);
    }

    pub fn parse_modrm(&mut self, emu: &mut Emulator) {
        let code = emu.get_code8(0);
        self.set_mod((code & 0b11000000) >> 6);
        self.set_regbit((code & 0b00111000) >> 3);
        self.set_rm(code & 0b00000111);

        emu.inc_eip(1);

        if self.get_mod() != 0b11 && self.get_rm() == 0b11 {
            self.set_sib(emu.get_code8(0));
            emu.inc_eip(1);
        }

        if (self.get_mod() == 0b00 && self.get_rm() == 0b101) || self.get_mod() == 0b10 {
            self.set_disp32(emu.get_signed_code32(0));
            emu.inc_eip(4);
        } else if self.get_mod() == 0b01 {
            self.set_disp8(emu.get_signed_code8(0));
            emu.inc_eip(1);
        }
    }

    pub fn set_rm32(&mut self, emu: &mut Emulator, value: u32) {
        if self.get_mod() == 0b11 {
            let reg = *emu.get_gpr_id(self.get_rm().into()).unwrap_or_else(|| {
                panic!("Could not find the register specified by Mod/RM: {:#x?}", self);
            });
            emu.set_gpr(&reg, value);
        } else {
            emu.set_memory32(self.calc_memory_address(emu) as u32, value);
        }
    }

    pub fn calc_memory_address(&self, emu: &Emulator) -> i32 {
        if self.get_mod() == 0b00 {
            if self.get_rm() == 0b11 {
                // SIB
                panic!("Not implemented: {:#x?}", self);
            } else if self.get_rm() == 0b101 {
                self.get_disp32().unwrap_or_else(|| {
                    panic!("Displacement not found: {:?}", self);
                }) as i32
            } else {
                *emu.get_gpr_value(emu.get_gpr_id(self.get_rm().into())
                    .unwrap_or_else(|| {
                        panic!("Could not find the register specified by Mod/RM: {:#x?}", self)
                    })) as i32
            }
        } else if self.get_mod() == 0b01 {
            if self.get_rm() == 0b100 {
                // SIB
                panic!("Not implemented: {:#x?}", self);
            } else {
                *emu.get_gpr_value(emu.get_gpr_id(self.get_rm().into())
                    .unwrap_or_else(|| {
                        panic!("Could not find the register specified by Mod/RM: {:#x?}", self);
                    })) as i32
                    + self.get_disp8().unwrap_or_else(|| {
                        panic!("Displacement not found: {:?}", self);
                    }) as i32
            }
        } else if self.get_mod() == 0b10 {
            if self.get_rm() == 0b100 {
                // SIB
                panic!("Not implemented: {:#x?}", self);
            } else {
                *emu.get_gpr_value(emu.get_gpr_id(self.get_rm().into())
                    .unwrap_or_else(|| {
                        panic!("Could not find the register specified by Mod/RM: {:#x?}", self);
                    })) as i32
            }
        } else {
            panic!("Not implemented: {:#x?}", self);
        }
    }
}