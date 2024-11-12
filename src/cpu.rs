use std::collections::HashMap;
use crate::cpu_types::{AddressingMode, Operation, CpuFlag, STACK_RESET};
use crate::instruction::{Instruction, INSTRUCTIONS_MAP};

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub status: CpuFlag,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

trait Memory {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos);
        let hi = self.mem_read(pos + 1);
        u16::from_le_bytes([lo, hi])
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let bytes = data.to_le_bytes();
        self.mem_write(pos, bytes[0]);
        self.mem_write(pos + 1, bytes[1]);
    }
}

impl Memory for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    //noinspection RsTypeCheck
    pub fn new() -> CPU {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0,
            status: CpuFlag::empty(),
            program_counter: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::ZeroPageX => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPageY => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_y) as u16
            }
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter);
                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }
            AddressingMode::NoneAddressing => {
                panic!("Mode NoneAddressing not supported");
            }
        }
    }

    // Ignoring decimal mode
    fn add_to_register_a(&mut self, data: u8) {
        let mut sum = self.register_a as u16 + data as u16;

        if self.status.contains(CpuFlag::CARRY) {
            sum += 1;
        }

        let carry = sum > 0xff;

        if carry {
            self.status.insert(CpuFlag::CARRY);
        } else {
            self.status.remove(CpuFlag::CARRY);
        }

        let result = sum as u8;

        //Determine if result causes overflow
        if (data ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.status.insert(CpuFlag::OVERFLOW);
        } else {
            self.status.remove(CpuFlag::OVERFLOW)
        }

        self.register_a = result;
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_register_a(value);
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let mut data;
        let addr;

        // Determine whether to operate on the accumulator or memory
        if let AddressingMode::NoneAddressing = mode {
            // ASL on the accumulator
            data = self.register_a;
            addr = 0; // Dummy value; not used in this case
        } else {
            // ASL on memory
            addr = self.get_operand_address(mode);
            data = self.mem_read(addr);
        }

        if data >> 7 == 1 {
            self.status.insert(CpuFlag::CARRY);
        } else {
            self.status.remove(CpuFlag::CARRY);
        }

        data <<= 1;

        if let AddressingMode::NoneAddressing = mode {
            self.register_a = data;
        } else {
            self.mem_write(addr, data);
        }

        self.update_zero_and_negative_flags(data);
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let jump: i8 = self.mem_read(self.program_counter) as i8;
            let jump_addr = self.program_counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);
            self.program_counter = jump_addr;
        }
    }
    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = CpuFlag::from_bits_truncate(0b00100100);
        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(CpuFlag::ZERO);
        } else {
            self.status.remove(CpuFlag::ZERO);
        }

        if result & 0b1000_0000 != 0 {
            self.status.insert(CpuFlag::NEGATIVE);
        } else {
            self.status.remove(CpuFlag::NEGATIVE);
        }
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static Instruction>
            = *INSTRUCTIONS_MAP;

        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes.get(&code)
                .expect(&format!("Opcode {:x} is not recognized", code));

            match opcode.operation {
                Operation::ADC => self.adc(&opcode.mode),
                Operation::AND => self.and(&opcode.mode),
                Operation::ASL => self.asl(&opcode.mode),
                Operation::BCC => self.branch(!self.status.contains(CpuFlag::CARRY)),
                Operation::BCS => self.branch(self.status.contains(CpuFlag::CARRY)),
                Operation::BEQ => self.branch(self.status.contains(CpuFlag::ZERO)),
                Operation::BIT => todo!(),
                Operation::BMI => self.branch(self.status.contains(CpuFlag::NEGATIVE)),
                Operation::BNE => self.branch(!self.status.contains(CpuFlag::ZERO)),
                Operation::BPL => self.branch(!self.status.contains(CpuFlag::NEGATIVE)),
                Operation::BRK => return,
                Operation::BVC => self.branch(!self.status.contains(CpuFlag::OVERFLOW)),
                Operation::BVS => self.branch(self.status.contains(CpuFlag::OVERFLOW)),
                Operation::CLC => todo!(),
                Operation::CLD => todo!(),
                Operation::CLI => todo!(),
                Operation::CLV => todo!(),
                Operation::CMP => todo!(),
                Operation::CPX => todo!(),
                Operation::CPY => todo!(),
                Operation::DEC => todo!(),
                Operation::DEX => todo!(),
                Operation::DEY => todo!(),
                Operation::EOR => todo!(),
                Operation::INC => todo!(),
                Operation::INX => self.inx(),
                Operation::INY => todo!(),
                Operation::JMP => todo!(),
                Operation::JSR => todo!(),
                Operation::LDA => self.lda(&opcode.mode),
                Operation::LDX => todo!(),
                Operation::LDY => todo!(),
                Operation::LSR => todo!(),
                Operation::NOP => todo!(),
                Operation::ORA => todo!(),
                Operation::PHA => todo!(),
                Operation::PHP => todo!(),
                Operation::PLA => todo!(),
                Operation::PLP => todo!(),
                Operation::ROL => todo!(),
                Operation::ROR => todo!(),
                Operation::RTI => todo!(),
                Operation::RTS => todo!(),
                Operation::SBC => todo!(),
                Operation::SEC => todo!(),
                Operation::SED => todo!(),
                Operation::SEI => todo!(),
                Operation::STA => todo!(),
                Operation::STX => todo!(),
                Operation::STY => todo!(),
                Operation::TAX => self.tax(),
                Operation::TAY => todo!(),
                Operation::TSX => todo!(),
                Operation::TXA => todo!(),
                Operation::TXS => todo!(),
                Operation::TYA => todo!(),
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.length - 1) as u16;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);

        assert_eq!(cpu.register_a, 5);
        assert_eq!(cpu.status.bits() & 0b0000_0010, 0b00);
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);

        assert_eq!(cpu.status.bits() & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);

        assert_eq!(cpu.status.bits() & 0b1000_0000, 0b1000_0000);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.load_and_run(vec![0xa9, 0x0A,0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa,0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}