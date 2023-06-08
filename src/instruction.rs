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
        for i in 0..8 {
            instructions[0x50 + i] = Some(push_r32);
        }
        for i in 0..8 {
            instructions[0x58 + i] = Some(pop_r32);
        }
        instructions[0x68] = Some(push_imm32);
        instructions[0x6A] = Some(push_imm8);
        instructions[0x70] = Some(jo);
        instructions[0x71] = Some(jno);
        instructions[0x72] = Some(jc);
        instructions[0x73] = Some(jnc);
        instructions[0x74] = Some(jz);
        instructions[0x75] = Some(jnz);
        instructions[0x78] = Some(js);
        instructions[0x79] = Some(jns);
        instructions[0x7C] = Some(jl);
        instructions[0x7E] = Some(jle);
        instructions[0x83] = Some(code_83);
        instructions[0x89] = Some(mov_rm32_r32);
        instructions[0x8B] = Some(mov_r32_rm32);
        for i in 0..8 {
            instructions[0xB8 + i] = Some(mov_r32_imm32);
        }
        instructions[0xC3] = Some(ret);
        instructions[0xC7] = Some(mov_rm32_imm32);
        instructions[0xC9] = Some(leave);
        instructions[0xE8] = Some(call_rel32);
        instructions[0xE9] = Some(near_jump);
        instructions[0xEB] = Some(short_jump);
        instructions[0xFF] = Some(code_ff);

        InstructionVector(instructions)
    }
}
