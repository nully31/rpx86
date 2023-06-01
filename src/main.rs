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