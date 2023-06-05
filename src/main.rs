//
// main
//
use std::env;
use std::fs;
use std::process;

use crate::{config::Config, emulator::Emulator, instruction::InstructionVector};

pub mod config;
pub mod emulator;
pub mod instruction;

const MEM_SIZE: usize = 0xffff;
const INST_SIZE: usize = 0xff;
const ORG: u32 = 0x7c00;

fn main() {
    let mut emu = Emulator::new(MEM_SIZE, ORG, ORG);

    let args: Vec<String> = env::args().collect();
    let fp = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    emu.load_bin(fs::read(fp.get_fp()).unwrap_or_else(|err| {
        eprintln!("Could not load binary: {err}");
        process::exit(1);
    }), ORG);

    let instructions = InstructionVector::new(INST_SIZE);
    emu.run(instructions);
    emu.dump();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_test() {
        let mut emu = Emulator::new(MEM_SIZE, 0x0000, ORG);
    
        emu.load_bin(fs::read("helloworld.bin").unwrap_or_else(|err| {
            eprintln!("Could not load binary: {err}");
            process::exit(1);
        }), 0x0000);
    
        let instructions = InstructionVector::new(INST_SIZE);
        emu.run(instructions);
        // emu.dump();
        assert_eq!(*emu.get_gpr_value(&emulator::GPR::EAX), 41);
    }
}