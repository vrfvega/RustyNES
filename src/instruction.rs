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

        Instruction::new(0x0a, Operation::ASL, AddressingMode::Accumulator, 1, 2),
        Instruction::new(0x06, Operation::ASL, AddressingMode::ZeroPage, 2, 5),
        Instruction::new(0x16, Operation::ASL, AddressingMode::ZeroPageX, 2, 6),
        Instruction::new(0x0e, Operation::ASL, AddressingMode::Absolute, 3, 6),
        Instruction::new(0x1e, Operation::ASL, AddressingMode::AbsoluteX, 3, 7),

        Instruction::new(0x90, Operation::BCC, AddressingMode::Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xb0, Operation::BCS, AddressingMode::Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BMI, AddressingMode::Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BNE, AddressingMode::Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BPL, AddressingMode::Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BEQ, AddressingMode::Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0x24, Operation::BIT, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0x2c, Operation::BIT, AddressingMode::Absolute, 3, 4),

        Instruction::new(0x00, Operation::BRK, AddressingMode::Implied, 1, 7),

        Instruction::new(0xF0, Operation::BVC, AddressingMode::Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0xF0, Operation::BVS, AddressingMode::Relative, 2, 2), // +1 if branch succeeds, +2 if to a new page

        Instruction::new(0x18, Operation::CLC, AddressingMode::Implied, 1, 2),

        Instruction::new(0xd8, Operation::CLD, AddressingMode::Implied, 1, 2),

        Instruction::new(0x58, Operation::CLI, AddressingMode::Implied, 1, 2),

        Instruction::new(0xb8, Operation::CLV, AddressingMode::Implied, 1, 2),

        Instruction::new(0xc9, Operation::CMP, AddressingMode::Immediate, 2, 2),
        Instruction::new(0xc5, Operation::CMP, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0xd5, Operation::CMP, AddressingMode::ZeroPageX, 2, 4),
        Instruction::new(0xcd, Operation::CMP, AddressingMode::Absolute, 3, 4),
        Instruction::new(0xdd, Operation::CMP, AddressingMode::AbsoluteX, 3, 4), // +1 if page crossed
        Instruction::new(0xd9, Operation::CMP, AddressingMode::AbsoluteY, 3, 4), // +1 if page crossed
        Instruction::new(0xc1, Operation::CMP, AddressingMode::IndirectX, 2, 6),
        Instruction::new(0xd1, Operation::CMP, AddressingMode::IndirectY, 2, 5), // +1 if page crossed

        Instruction::new(0xe0, Operation::CPX, AddressingMode::Immediate, 2, 2),
        Instruction::new(0xe4, Operation::CPX, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0xec, Operation::CPX, AddressingMode::Absolute, 3, 4),

        Instruction::new(0xc0, Operation::CPY, AddressingMode::Immediate, 2, 2),
        Instruction::new(0xc4, Operation::CPY, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0xcc, Operation::CPY, AddressingMode::Absolute, 3, 4),

        Instruction::new(0xc9, Operation::DEC, AddressingMode::ZeroPage, 2, 5),
        Instruction::new(0xd6, Operation::DEC, AddressingMode::ZeroPageX, 2, 6),
        Instruction::new(0xce, Operation::DEC, AddressingMode::Absolute, 3, 6),
        Instruction::new(0xde, Operation::DEC, AddressingMode::AbsoluteX, 3, 7),

        Instruction::new(0xca, Operation::DEX, AddressingMode::Implied, 1, 2),

        Instruction::new(0x88, Operation::DEY, AddressingMode::Implied, 1, 2),

        Instruction::new(0x49, Operation::EOR, AddressingMode::Immediate, 2, 2),
        Instruction::new(0x45, Operation::EOR, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0x55, Operation::EOR, AddressingMode::ZeroPageX, 2, 4),
        Instruction::new(0x4d, Operation::EOR, AddressingMode::Absolute, 3, 4),
        Instruction::new(0x5d, Operation::EOR, AddressingMode::AbsoluteX, 3, 4), // +1 if page crossed
        Instruction::new(0x59, Operation::EOR, AddressingMode::AbsoluteY, 3, 4), // +1 if page crossed
        Instruction::new(0x41, Operation::EOR, AddressingMode::IndirectX, 2, 6),
        Instruction::new(0x51, Operation::EOR, AddressingMode::IndirectY, 2, 5), // +1 if page crossed

        Instruction::new(0xe6, Operation::INC, AddressingMode::ZeroPage, 2, 5),
        Instruction::new(0xf6, Operation::INC, AddressingMode::ZeroPageX, 2, 6),
        Instruction::new(0xee, Operation::INC, AddressingMode::Absolute, 3, 6),
        Instruction::new(0xfe, Operation::INC, AddressingMode::AbsoluteX, 3, 7),

        Instruction::new(0xe8, Operation::INX, AddressingMode::Implied, 1, 2),

        Instruction::new(0xc8, Operation::INY, AddressingMode::Implied, 1, 2),

        Instruction::new(0x4c, Operation::JMP, AddressingMode::Absolute, 3, 3),
        Instruction::new(0x6c, Operation::JMP, AddressingMode::Indirect, 3, 5),

        Instruction::new(0x20, Operation::JSR, AddressingMode::Absolute, 3, 6),

        Instruction::new(0xa9, Operation::LDA, AddressingMode::Immediate, 2, 2),
        Instruction::new(0xa5, Operation::LDA, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0xb5, Operation::LDA, AddressingMode::ZeroPageX, 2, 4),
        Instruction::new(0xad, Operation::LDA, AddressingMode::Absolute, 3, 4),
        Instruction::new(0xbd, Operation::LDA, AddressingMode::AbsoluteX, 3, 4), // +1 if page crossed
        Instruction::new(0xb9, Operation::LDA, AddressingMode::AbsoluteY, 3, 4), // +1 if page crossed
        Instruction::new(0xa1, Operation::LDA, AddressingMode::IndirectX, 2, 6),
        Instruction::new(0xb1, Operation::LDA, AddressingMode::IndirectY, 2, 5), // +1 if page crossed

        Instruction::new(0xa2, Operation::LDX, AddressingMode::Immediate, 2, 2),
        Instruction::new(0xa6, Operation::LDX, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0xb6, Operation::LDX, AddressingMode::ZeroPageY, 2, 4),
        Instruction::new(0xae, Operation::LDX, AddressingMode::Absolute, 3, 4),
        Instruction::new(0xbe, Operation::LDX, AddressingMode::AbsoluteY, 3, 4), // +1 if page crossed

        Instruction::new(0xa0, Operation::LDX, AddressingMode::Immediate, 2, 2),
        Instruction::new(0xa4, Operation::LDX, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0xb4, Operation::LDX, AddressingMode::ZeroPageX, 2, 4),
        Instruction::new(0xac, Operation::LDX, AddressingMode::Absolute, 3, 4),
        Instruction::new(0xbc, Operation::LDX, AddressingMode::AbsoluteX, 3, 4), // +1 if page crossed

        Instruction::new(0x4a, Operation::LSR, AddressingMode::Accumulator, 1, 2),
        Instruction::new(0x46, Operation::LSR, AddressingMode::ZeroPage, 2, 5),
        Instruction::new(0x56, Operation::LSR, AddressingMode::ZeroPageX, 2, 6),
        Instruction::new(0x4e, Operation::LSR, AddressingMode::Absolute, 3, 6),
        Instruction::new(0x5e, Operation::LSR, AddressingMode::AbsoluteX, 3, 7),

        Instruction::new(0xea, Operation::NOP, AddressingMode::Implied, 1, 2),

        Instruction::new(0x09, Operation::ORA, AddressingMode::Immediate, 2, 2),
        Instruction::new(0x05, Operation::ORA, AddressingMode::ZeroPage, 2, 3),
        Instruction::new(0x15, Operation::ORA, AddressingMode::ZeroPageX, 2, 4),
        Instruction::new(0x0d, Operation::ORA, AddressingMode::Absolute, 3, 4),
        Instruction::new(0x1d, Operation::ORA, AddressingMode::AbsoluteX, 3, 4), // +1 if page crossed
        Instruction::new(0x19, Operation::ORA, AddressingMode::AbsoluteY, 3, 4), // +1 if page crossed
        Instruction::new(0x01, Operation::ORA, AddressingMode::IndirectX, 2, 6),
        Instruction::new(0x11, Operation::ORA, AddressingMode::IndirectY, 2, 5), // +1 if page crossed

        Instruction::new(0xaa, Operation::TAX, AddressingMode::NoneAddressing, 1, 2),
    ];

    pub static ref INSTRUCTIONS_MAP: HashMap<u8, &'static Instruction> = {
        let mut map = HashMap::new();
        for instr in &*INSTRUCTIONS {
            map.insert(instr.code, instr);
        }
        map
    };
}

