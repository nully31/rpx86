//
// Instruction table setup
//
use crate::emulator::Emulator;
use crate::instruction::operation::*;

pub mod operation;

type InstructionPtr = fn(&mut Emulator);

pub struct InstructionVector(pub Vec<Option<InstructionPtr>>);

impl InstructionVector {
    pub fn new(size: usize) -> InstructionVector {
        assert!(size >= 0xff);
        let mut instructions: Vec<Option<InstructionPtr>> = vec![None; size];

        instructions[0x01] = Some(add_rm32_r32);
        instructions[0x83] = Some(code_83);
        instructions[0x89] = Some(mov_rm32_r32);
        instructions[0x8B] = Some(mov_r32_rm32);
        for i in 0..8 {
            instructions[0xB8 + i] = Some(mov_r32_imm32);
        }
        instructions[0xC7] = Some(mov_rm32_imm32);
        instructions[0xE9] = Some(near_jump);
        instructions[0xEB] = Some(short_jump);
        instructions[0xFF] = Some(code_ff);

        InstructionVector(instructions)
    }
}
