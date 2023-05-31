//
// Emulator stuff
//
#[derive(Hash, Eq, PartialEq, Debug)]
enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}

use std::collections::HashMap;

#[derive(Debug)]
struct Emulator {
    registers: HashMap<Register, u32>,
    eflags: u32,
    memory: Vec<u8>,
    eip: u32,
}

impl Emulator {
    fn new(size: usize, eip_value: u32, esp_value: u32) -> Self {
        assert!(size > 0);

        let registers: HashMap<Register, u32> = HashMap::from([
            (Register::EAX, 0x0),
            (Register::EBX, 0x0),
            (Register::ECX, 0x0),
            (Register::EDX, 0x0),
            (Register::ESI, 0x0),
            (Register::EDI, 0x0),
            (Register::ESP, esp_value),
            (Register::EBP, 0x0),
        ]);
        let memory = vec![0; size];
        let eip = eip_value;
        let eflags = 0x0;

        Self { registers, eflags, memory, eip }
    }

    fn get_register_id(&self, reg: u32) -> Option<Register> {
        match reg {
            v if v == Register::EAX as u32 => {
                Some(Register::EAX)
            }
            v if v == Register::ECX as u32 => {
                Some(Register::ECX)
            }
            v if v == Register::EDX as u32 => {
                Some(Register::EDX)
            }
            v if v == Register::EBX as u32 => {
                Some(Register::EBX)
            }
            v if v == Register::ESP as u32 => {
                Some(Register::ESP)
            }
            v if v == Register::EBP as u32 => {
                Some(Register::EBP)
            }
            v if v == Register::ESI as u32 => {
                Some(Register::ESI)
            }
            v if v == Register::EDI as u32 => {
                Some(Register::EDI)
            }
            _ => {
                None
            }
        }
    }

    fn set_register(&mut self, reg: Register, new_value: u32) {
        self.registers.entry(reg).and_modify(|reg_value| {
            *reg_value = new_value;
        });
    }

    fn set_eip(&mut self, new_value: u32) {
        self.eip = new_value;
    }

    fn load_bin(&mut self, binary: Vec<u8>) {
        self.memory = binary;
    }

    fn get_code8(&self, index: usize) -> u32 {
        self.memory[self.eip as usize + index] as u32
    }

    fn get_signed_code8(&self, index: usize) -> i32 {
        self.memory[self.eip as usize + index] as i32
    }

    fn get_code32(&self, index: usize) -> u32 {
        let mut ret: u32 = 0x0;
        // convert little endian to the correct byte order
        for i in (0..4).rev() {
            ret |= self.get_code8(i + index) as u32 >> (i * 8);
        }
        ret
    }

    fn run(&mut self, mem_size: u32, instructions: InstructionVector) {
        while self.eip < mem_size {
            let code = self.get_code8(0);
            match instructions.0[code as usize] {
                Some(instruction) =>  instruction(self),
                _ => panic!("Not implemented: code {code}")     // TODO: error propagation
            }
            if self.eip == 0x00 {
                println!("End of program");
                break;
            }
        }
    }

    fn dump(&self) {
        println!("{:#x?}", self);
    }
}


//
// File read
//
struct Config {
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Self { file_path })
    }

    fn get_fp(&self) -> &str {
        &self.file_path
    }
}


//
// Instruction table setup
//
type InstructionPtr = fn(&mut Emulator);

struct InstructionVector(Vec<Option<InstructionPtr>>);

impl InstructionVector {
    fn new(size: usize) -> InstructionVector {
        assert!(size >= 0xff);
        let mut instructions: Vec<Option<InstructionPtr>> = vec![None; size];
        for i in 0..8 {
            instructions[0xB8 + i] = Some(mov_r32_imm32);
        }
        instructions[0xEB] = Some(short_jump);
        InstructionVector(instructions)
    }
}

//
// Instructions
//
fn mov_r32_imm32(emu: &mut Emulator) {
    let reg = emu.get_code8(0) - 0xB8;
    let value = emu.get_code32(1);
    let reg = emu.get_register_id(reg).unwrap();  // TODO: error propagation
    emu.set_register(reg, value);
    emu.eip += 5;
}

fn short_jump(emu: &mut Emulator) {
    let diff = emu.get_signed_code8(1) as i8;
    emu.set_eip((emu.eip as i32 + diff as i32 + 2) as u32);
}


//
// main
//
use std::env;
use std::fs;
use std::process;
fn main() {
    let mut emu = Emulator::new(0xffff, 0x0000, 0x7c00);

    let args: Vec<String> = env::args().collect();
    let fp = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    emu.load_bin(fs::read(fp.get_fp()).unwrap_or_else(|err| {
        eprintln!("Could not load binary: {err}");
        process::exit(1);
    }));

    let instructions = InstructionVector::new(0xff);
    emu.run(0xffff, instructions);
    emu.dump();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_emulator() {
        let mut emu = Emulator::new(0x3, 0x1111, 0xffff);
        println!("{:?}", emu);
        assert_eq!(emu.memory.len(), 3);
        assert_eq!(emu.eip, 0x1111);
        assert_eq!(emu.registers.get(&Register::ESP), Some(&0xffff));

        emu.set_register(Register::EAX, 0xff);
        println!("{:?}", emu);
        assert_eq!(emu.registers.get(&Register::EAX), Some(&0xff));
    }
}