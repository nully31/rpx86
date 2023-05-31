#[derive(Hash, Eq, PartialEq, Debug)]
enum Register {
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
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

        let mut registers: HashMap<Register, u32> = HashMap::from([
            (Register::EAX, 0x0),
            (Register::EBX, 0x0),
            (Register::ECX, 0x0),
            (Register::EDX, 0x0),
            (Register::ESI, 0x0),
            (Register::EDI, 0x0),
            (Register::ESP, esp_value),
            (Register::EBP, 0x0),
        ]);
        let mut memory = vec![0; size];
        let mut eip = eip_value;
        let mut eflags = 0x0;

        Self { registers, eflags, memory, eip }
    }

    fn set_register(&mut self, reg: Register, new_value: u32) {
        self.registers.entry(reg).and_modify(|reg_value| {
            *reg_value = new_value;
        });
    }
}

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

    let file = fs::read(fp.get_fp()).unwrap();
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