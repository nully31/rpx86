//
// Emulator stuff
//
use crate::instruction::InstructionVector;

pub mod modrm;

#[derive(Hash, Eq, PartialEq, Debug)]
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

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct SPR {
    eflags: u32,
    eip: u32,
}

use std::collections::HashMap;
use std::fmt;

pub struct Emulator {
    reg_file: HashMap<GPR, u32>,
    sp_reg: SPR,
    memory: Vec<u8>,
}

impl Emulator {
    pub fn new(size: usize, eip_value: u32, esp_value: u32) -> Self {
        assert!(size > 0);

        let reg_file = HashMap::from([
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

    pub fn get_gpr_id(&self, reg: u32) -> Option<GPR> {
        match reg {
            v if v == GPR::EAX as u32 => {
                Some(GPR::EAX)
            }
            v if v == GPR::ECX as u32 => {
                Some(GPR::ECX)
            }
            v if v == GPR::EDX as u32 => {
                Some(GPR::EDX)
            }
            v if v == GPR::EBX as u32 => {
                Some(GPR::EBX)
            }
            v if v == GPR::ESP as u32 => {
                Some(GPR::ESP)
            }
            v if v == GPR::EBP as u32 => {
                Some(GPR::EBP)
            }
            v if v == GPR::ESI as u32 => {
                Some(GPR::ESI)
            }
            v if v == GPR::EDI as u32 => {
                Some(GPR::EDI)
            }
            _ => {
                None
            }
        }
    }

    pub fn get_gpr_value(&self, reg: &GPR) -> Option<&u32> {
        self.reg_file.get(reg)
    }

    pub fn set_gpr(&mut self, reg: GPR, new_value: u32) {
        self.reg_file.entry(reg).and_modify(|reg_value| {
            *reg_value = new_value;
        });
    }

    pub fn get_eip(&self) -> u32 {
        self.sp_reg.eip
    }

    pub fn set_eip(&mut self, new_value: u32) {
        self.sp_reg.eip = new_value;
    }

    pub fn inc_eip(&mut self, incrementor: u32) {
        self.sp_reg.eip += incrementor;
    }

    pub fn get_eflags(&self) -> u32 {
        self.sp_reg.eflags
    }

    pub fn set_eflags(&mut self, new_value: u32) {
        self.sp_reg.eflags = new_value;
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

        emu.set_gpr(GPR::EAX, 0xff);
        println!("{:?}", emu);
        assert_eq!(emu.reg_file.get(&GPR::EAX), Some(&0xff));
    }
}