//
// Emulator stuff
//
use crate::instruction::InstructionVector;

pub mod modrm;

const CARRY_FLAG: u32 = 1;
const ZERO_FLAG: u32 = 1 << 6;
const SIGN_FLAG: u32 = 1 << 7;
const OVERFLOW_FLAG: u32 = 1 << 11;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum GPR {
    EAX = 0,
    ECX = 1,
    EDX = 2,
    EBX = 3,
    ESP = 4,
    EBP = 5,
    ESI = 6,
    EDI = 7,
}

#[derive(Eq, PartialEq, Debug)]
pub struct SPR {
    eflags: u32,
    eip: u32,
}

use std::collections::BTreeMap;
use std::fmt;

pub struct Emulator {
    reg_file: BTreeMap<GPR, u32>,
    sp_reg: SPR,
    memory: Vec<u8>,
}

impl Emulator {
    pub fn new(size: usize, eip_value: u32, esp_value: u32) -> Self {
        assert!(size > 0);

        let reg_file = BTreeMap::from([
            (GPR::EAX, 0x0),
            (GPR::EBX, 0x0),
            (GPR::ECX, 0x0),
            (GPR::EDX, 0x0),
            (GPR::ESI, 0x0),
            (GPR::EDI, 0x0),
            (GPR::ESP, esp_value),
            (GPR::EBP, 0x0),
        ]);
        let sp_reg = SPR {
            eflags: 0x0,
            eip: eip_value,
        };
        let memory = vec![0; size];

        Self { reg_file, sp_reg, memory }
    }

    pub fn get_gpr_id(&self, reg: u32) -> Option<&GPR> {
        match reg {
            v if v == GPR::EAX as u32 => {
                Some(&GPR::EAX)
            }
            v if v == GPR::ECX as u32 => {
                Some(&GPR::ECX)
            }
            v if v == GPR::EDX as u32 => {
                Some(&GPR::EDX)
            }
            v if v == GPR::EBX as u32 => {
                Some(&GPR::EBX)
            }
            v if v == GPR::ESP as u32 => {
                Some(&GPR::ESP)
            }
            v if v == GPR::EBP as u32 => {
                Some(&GPR::EBP)
            }
            v if v == GPR::ESI as u32 => {
                Some(&GPR::ESI)
            }
            v if v == GPR::EDI as u32 => {
                Some(&GPR::EDI)
            }
            _ => {
                None
            }
        }
    }

    pub fn get_gpr_value(&self, reg: &GPR) -> &u32 {
        self.reg_file.get(reg).unwrap_or_else(|| {
            panic!("Could not find the register specified by: {:#x?}", reg);
        })
    }

    pub fn set_gpr(&mut self, reg: &GPR, new_value: u32) {
        self.reg_file.entry(*reg).and_modify(|reg_value| {
            *reg_value = new_value;
        });
    }

    pub fn get_eip(&self) -> u32 {
        self.sp_reg.eip
    }

    pub fn set_eip(&mut self, new_value: u32) {
        self.sp_reg.eip = new_value;
    }

    pub fn inc_eip(&mut self, increment_by: i32) {
        let mut eip = self.sp_reg.eip as i32;
        eip += increment_by;
        self.set_eip(eip as u32);
    }

    pub fn get_eflags(&self) -> u32 {
        self.sp_reg.eflags
    }

    pub fn is_carry(&self) -> bool {
        self.get_eflags() & CARRY_FLAG != 0
    }

    pub fn is_zero(&self) -> bool {
        self.get_eflags() & ZERO_FLAG != 0
    }

    pub fn is_signed(&self) -> bool {
        self.get_eflags() & SIGN_FLAG != 0
    }

    pub fn is_overflow(&self) -> bool {
        self.get_eflags() & OVERFLOW_FLAG != 0
    }

    pub fn update_eflags_sub(&mut self, v1: u32, v2: u32, result: u64) {
        let sign1 = v1 >> 31;
        let sign2 = v2 >> 31;
        let signr = (result >> 31) & 1;

        self.set_carry(result >> 32);
        self.set_zero(result == 0);
        self.set_sign(signr);
        self.set_overflow(sign1 != sign2 && sign1 as u64 != signr);
    }

    fn set_carry(&mut self, is_carry: u64) {
        if is_carry != 0 {
            self.sp_reg.eflags |= CARRY_FLAG;
        } else {
            self.sp_reg.eflags &= !CARRY_FLAG;
        }
    }

    fn set_zero(&mut self, is_zero: bool) {
        if is_zero {
            self.sp_reg.eflags |= ZERO_FLAG;
        } else {
            self.sp_reg.eflags &= !ZERO_FLAG;
        }
    }

    fn set_sign(&mut self, is_signed: u64) {
        if is_signed != 0 {
            self.sp_reg.eflags |= SIGN_FLAG;
        } else {
            self.sp_reg.eflags &= !SIGN_FLAG;
        }
    }

    fn set_overflow(&mut self, is_overflow: bool) {
        if is_overflow {
            self.sp_reg.eflags |= OVERFLOW_FLAG;
        } else {
            self.sp_reg.eflags &= !OVERFLOW_FLAG;
        }
    }

    pub fn load_bin(&mut self, binary: Vec<u8>, address: u32) {
        let end_index = address as usize + binary.len();
        self.memory.splice(address as usize..end_index, binary);
    }

    pub fn get_code8(&self, index: usize) -> u8 {
        self.memory[self.sp_reg.eip as usize + index] as u8
    }

    pub fn get_signed_code8(&self, index: usize) -> i8 {
        self.memory[self.sp_reg.eip as usize + index] as i8
    }

    pub fn get_code32(&self, index: usize) -> u32 {
        let mut ret: u32 = 0x0;
        // convert little endian to the correct byte order
        for i in 0..4 {
            ret |= (self.get_code8(i + index) as u32) << (i * 8);
        }
        ret
    }

    pub fn get_signed_code32(&self, index: usize) -> i32 {
        let mut ret: i32 = 0x0;
        for i in 0..4 {
            ret |= (self.get_code8(i + index) as i32) << (i * 8);
        }
        ret
    }

    pub fn get_memory8(&self, address: u32) -> u8 {
        self.memory[address as usize]
    }

    pub fn get_memory32(&self, address: u32) -> u32 {
        let mut ret: u32 = 0x0;
        for i in 0..4 {
            ret |= (self.get_memory8(address + i) as u32) << (i * 8);
        }
        ret
    }

    pub fn set_memory8(&mut self, address: u32, value: u32) {
        self.memory[address as usize] = (value & 0xff) as u8;
    }

    pub fn set_memory32(&mut self, address: u32, value: u32) {
        for i in 0..4 {
            self.set_memory8(address + i, value >> (i * 8));
        }
    }

    pub fn run(&mut self, instructions: InstructionVector) {
        while self.sp_reg.eip < self.memory.len() as u32 {
            let code = self.get_code8(0);
            println!("eip: 0x{:x}, code: 0x{:x}", self.sp_reg.eip, code);
            match instructions.0[code as usize] {
                Some(instruction) =>  instruction(self),
                _ => panic!("Not implemented: code 0x{:x}", code)     // TODO: error propagation
            }
            if self.sp_reg.eip == 0x00 {
                println!("End of program");
                break;
            }
        }
    }

    pub fn dump(&self) {
        println!("{:#x?}", self);
    }
}

impl fmt::Debug for Emulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Emulator")
            .field("reg_file", &self.reg_file)
            .field("sp_reg", &self.sp_reg)
            //.field("memory", &self.memory)
            .finish()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emulator_test() {
        let mut emu = Emulator::new(0x3, 0x1111, 0xffff);
        println!("{:?}", emu);
        assert_eq!(emu.memory.len(), 3);
        assert_eq!(emu.get_eip(), 0x1111);
        assert_eq!(emu.reg_file.get(&GPR::ESP), Some(&0xffff));

        emu.set_gpr(&GPR::EAX, 0xff);
        println!("{:?}", emu);
        assert_eq!(emu.reg_file.get(&GPR::EAX), Some(&0xff));
    }
}