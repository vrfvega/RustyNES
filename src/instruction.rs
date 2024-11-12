use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::cpu_types::{AddressingMode, Operation};
pub struct Instruction {
    pub code: u8,
    pub operation: Operation,
    pub mode: AddressingMode,
    pub length: u8,
    pub cycles: u8,
}

impl Instruction {
    fn new(code: u8, operation: Operation, mode: AddressingMode, length: u8, cycles: u8) -> Self {
        Instruction {
            code,
            operation,
            mode,
            length, // in bytes
            cycles,
        }
    }
}

lazy_static! {
    pub static ref INSTRUCTIONS: Vec<Instruction> = vec![
        Instruction::new(0x69, Operation::ADC, AddressingMode::Immediate, 2, 2),
        Instruction::new(0x65, Operation::ADC, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0x75, Operation::ADC, AddressingMode::ZeroPageX, 2, 4),
        Instruction::new(0x6d, Operation::ADC, AddressingMode::Absolute, 3, 4),
        Instruction::new(0x7d, Operation::ADC, AddressingMode::AbsoluteX, 3, 4), // +1 if page crossed
        Instruction::new(0x79, Operation::ADC, AddressingMode::AbsoluteY, 3, 4), // +1 if page crossed
        Instruction::new(0x61, Operation::ADC, AddressingMode::IndirectX, 2, 6),
        Instruction::new(0x71, Operation::ADC, AddressingMode::IndirectY, 2, 5), // +1 if page crossed

        Instruction::new(0x29, Operation::AND, AddressingMode::Immediate, 2, 2),
        Instruction::new(0x25, Operation::AND, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0x35, Operation::AND, AddressingMode::ZeroPageX, 2, 4),
        Instruction::new(0x2d, Operation::AND, AddressingMode::Absolute, 3, 4),
        Instruction::new(0x3d, Operation::AND, AddressingMode::AbsoluteX, 3, 4),
        Instruction::new(0x39, Operation::AND, AddressingMode::AbsoluteY, 3, 4), // +1 if page crossed
        Instruction::new(0x21, Operation::AND, AddressingMode::IndirectX, 2, 6), // +1 if page crossed
        Instruction::new(0x31, Operation::AND, AddressingMode::IndirectY, 2, 5), // +1 if page crossed

        Instruction::new(0x0a, Operation::ASL, AddressingMode::NoneAddressing, 1, 2),
        Instruction::new(0x06, Operation::ASL, AddressingMode::ZeroPage, 2, 5),
        Instruction::new(0x16, Operation::ASL, AddressingMode::ZeroPageX, 2, 6),
        Instruction::new(0x0e, Operation::ASL, AddressingMode::Absolute, 3, 6),
        Instruction::new(0x1e, Operation::ASL, AddressingMode::AbsoluteX, 3, 7),

        Instruction::new(0x90, Operation::BCC, AddressingMode::NoneAddressing, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xb0, Operation::BCS, AddressingMode::NoneAddressing, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BMI, AddressingMode::NoneAddressing, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BNE, AddressingMode::NoneAddressing, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BPL, AddressingMode::NoneAddressing, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BEQ, AddressingMode::NoneAddressing, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0x00, Operation::BRK, AddressingMode::NoneAddressing, 1, 7),

        Instruction::new(0xF0, Operation::BVC, AddressingMode::NoneAddressing, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BVS, AddressingMode::NoneAddressing, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xaa, Operation::TAX, AddressingMode::NoneAddressing, 1, 2),
        Instruction::new(0xe8, Operation::INX, AddressingMode::NoneAddressing, 1, 2),

        Instruction::new(0xa9, Operation::LDA, AddressingMode::Immediate, 2, 2),
        Instruction::new(0xa5, Operation::LDA, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0xb5, Operation::LDA, AddressingMode::ZeroPageX, 2, 4),
        Instruction::new(0xad, Operation::LDA, AddressingMode::Absolute, 3, 4),
        Instruction::new(0xbd, Operation::LDA, AddressingMode::AbsoluteX, 3, 4),
        Instruction::new(0xb9, Operation::LDA, AddressingMode::AbsoluteY, 3, 4),
        Instruction::new(0xa1, Operation::LDA, AddressingMode::IndirectX, 2, 6),
        Instruction::new(0xb1, Operation::LDA, AddressingMode::IndirectY, 2, 5),
    ];

    pub static ref INSTRUCTIONS_MAP: HashMap<u8, &'static Instruction> = {
        let mut map = HashMap::new();
        for instr in &*INSTRUCTIONS {
            map.insert(instr.code, instr);
        }
        map
    };
}

