use std::collections::HashMap;

#[derive(Hash, Eq,PartialEq, Debug)]
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

        let mut registers: HashMap<Register, u32> = HashMap::new();
        let mut memory: Vec<u8> = vec![0; size];
        let mut eip: u32 = eip_value;
        let mut eflags: u32 = 0x0;

        registers.insert(Register::ESP, esp_value);

        Self { registers: registers, eflags: eflags, memory: memory, eip: eip }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_emulator() {
        let emulator = Emulator::new(0x3, 0x1111, 0xffff);
        println!("{:?}", emulator);
    }
}