pub struct CpuMemory {
    memory: [u8; 0xFFFF]
}

pub struct Bus {


}

enum AddressMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY
}

enum Interrupt {
    NMI,
    IRQ,
    RESET,
    BREAK
}

pub struct Instruction {
    opcode: u8,
    address_mode: AddressMode,
    cycles: u8
}

trait Memory {
    fn read_byte(&mut self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn read_word(&mut self, addr: u16) -> u16;
    fn write_word(&mut self,addr: u16, value: u16);
}

use std::collections::HashMap;
use std::ops::Sub;
use bitflags::bitflags;
use crate::Interrupt::BREAK;

bitflags! {
    struct FlagRegister: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const IRQ_DISABLE       = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const UNUSED            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}


pub struct Cpu {
    // Registers
    a: u8,  //Accumulator
    x: u8,  //Index register
    y: u8,  //Index register
    stack_ptr: u8,  //Stack Pointer
    flag: FlagRegister,  //Flag register
    prog_counter: u16, //Program Counter
}

impl Memory for Cpu{
    fn read_byte(&mut self, addr: u16) -> u8 {
        todo!()
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        todo!()
    }

    fn read_word(&mut self, addr: u16) -> u16 {
        todo!()
    }

    fn write_word(&mut self,addr: u16, value: u16) {
        todo!()
    }
}

impl Cpu {
    pub fn new() {
        Cpu {
            a:0,
            x: 0,
            y: 0,
            stack_ptr: 0,
            flag: FlagRegister::from_bits_truncate(0),
            prog_counter: 0,
        };
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.flag.insert(FlagRegister::ZERO);
        } else {
            self.flag.remove(FlagRegister::ZERO);
        }

        if result >> 7 == 1 {
            self.flag.insert(FlagRegister::NEGATIV);
        } else {
            self.flag.remove(FlagRegister::NEGATIV);
        }
    }

    fn read_instruction_operand_8bit(&mut self) -> u8{
        let operand = self.read_byte(self.prog_counter);
        self.prog_counter+=1;
        return operand;
    }

    fn read_instruction_operand_16bit(&mut self) -> u16{
        let operand = self.read_word(self.prog_counter);
        self.prog_counter+=2;
        return operand;
    }

    fn is_page_cross(addr1: u16, addr2: u16) -> bool {
        return addr1 & 0xFF00 != addr2 & 0xFF00;
    }

    fn get_operand_address(&mut self, address_mode: &AddressMode) -> (u16, bool) {
        return match address_mode {
            AddressMode::Immediate => {
                let current_pc = self.prog_counter;
                self.prog_counter+=1;
                (current_pc, false)
            },
            AddressMode::ZeroPage => {
                let addr = self.read_instruction_operand_8bit();
                (addr as u16, false)
            },
            AddressMode::ZeroPageX => {
                let addr = self.read_instruction_operand_8bit();
                (addr.wrapping_add(self.x) as u16, false)
            },
            AddressMode::ZeroPageY => {
                let addr = self.read_instruction_operand_8bit();
                (addr.wrapping_add(self.y) as u16, false)
            },
            AddressMode::Absolute => {
                let addr = self.read_instruction_operand_16bit();
                (addr, false)
            },
            AddressMode::AbsoluteX => {
                let addr = self.read_instruction_operand_16bit();
                let final_address = addr.wrapping_add(self.x as u16);
                (final_address, Cpu::is_page_cross(addr, final_address))
            },
            AddressMode::AbsoluteY => {
                let addr = self.read_instruction_operand_16bit();
                let final_address = addr.wrapping_add(self.y as u16);
                (final_address, Cpu::is_page_cross(addr, final_address))
            },
            AddressMode::IndirectX => {
                let addr = self.read_instruction_operand_8bit().wrapping_add(self.x);
                let final_address = self.read_word(addr as u16);
                (final_address, false)
            }
            AddressMode::IndirectY => {
                let operand = self.read_instruction_operand_8bit();
                let address = self.read_word(operand as u16);
                let final_address = address.wrapping_add(self.y as u16);
                (final_address, Cpu::is_page_cross(address, final_address))
            }
            _ => (0, false) //TODO: Should be a panic?
        };
    }

    fn lda(&mut self, opcode: &Instruction) -> u8 {
        let (operand,is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
        return if is_page_cross { opcode.cycles + 1 } else { opcode.cycles } //Based on opcode
    }

    fn ldx(&mut self, opcode: &Instruction) -> u8 {
        let (operand,is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.x = value;
        self.update_zero_and_negative_flags(self.x);
        return if is_page_cross { opcode.cycles + 1 } else { opcode.cycles } //Based on opcode
    }

    fn ldy(&mut self, opcode: &Instruction) -> u8 {
        let (operand,is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.y = value;
        self.update_zero_and_negative_flags(self.y);
        return if is_page_cross { opcode.cycles + 1 } else { opcode.cycles } //Based on opcode
    }

    fn st(&mut self, var: u8, opcode: &Instruction) -> u8 {
        let (operand,is_page_cross) = self.get_operand_address(&opcode.address_mode);
        self.write_byte(operand, var);
        return if is_page_cross { opcode.cycles + 1 } else { opcode.cycles } //Based on opcode
    }



    const OPCODES: [Instruction; 151] = [
        Instruction { opcode: 0xA8, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0xAA, cycles: 2, address_mode: AddressMode::Implied},
        Instruction { opcode: 0xBA, cycles: 2, address_mode: AddressMode::Implied},
        Instruction { opcode: 0x98, cycles: 2, address_mode: AddressMode::Implied},
        Instruction { opcode: 0x8A, cycles: 2, address_mode: AddressMode::Implied},
        Instruction { opcode: 0x9A, cycles: 2, address_mode: AddressMode::Implied},

        Instruction { opcode: 0xA9, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xA5, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xB5, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0xAD, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0xBD, cycles: 4, address_mode: AddressMode::AbsoluteX },
        Instruction { opcode: 0xB9, cycles: 4, address_mode: AddressMode::AbsoluteY },
        Instruction { opcode: 0xA1, cycles: 6, address_mode: AddressMode::IndirectX },
        Instruction { opcode: 0xB1, cycles: 5, address_mode: AddressMode::IndirectY },

        Instruction { opcode: 0xA2, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xA6, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xB6, cycles: 4, address_mode: AddressMode::ZeroPageY },
        Instruction { opcode: 0xAE, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0xBE, cycles: 4, address_mode: AddressMode::AbsoluteY },

        Instruction { opcode: 0xA0, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xA4, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xB4, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0xAC, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0xBC, cycles: 4, address_mode: AddressMode::AbsoluteX },

        Instruction { opcode: 0x85, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x95, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x8D, cycles: 4, address_mode: AddressMode::Absolute},
        Instruction { opcode: 0x9D, cycles: 5, address_mode: AddressMode::AbsoluteX },
        Instruction { opcode: 0x99, cycles: 6, address_mode: AddressMode::AbsoluteY },
        Instruction { opcode: 0x81, cycles: 6, address_mode: AddressMode::IndirectX },
        Instruction { opcode: 0x91, cycles: 6, address_mode: AddressMode::IndirectY },

        Instruction { opcode: 0x86, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x96, cycles: 4, address_mode: AddressMode::ZeroPageY },
        Instruction { opcode: 0x8E, cycles: 4, address_mode: AddressMode::Absolute },

        Instruction { opcode: 0x84, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x94, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x8C, cycles: 4, address_mode: AddressMode::Absolute },

        Instruction { opcode: 0x48, cycles: 3, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x08, cycles: 3, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x68, cycles: 4, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x28, cycles: 4, address_mode: AddressMode::Implied },

        Instruction { opcode: 0x69, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0x65, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x75, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x6D, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x7D, cycles: 4, address_mode: AddressMode::AbsoluteX },
        Instruction { opcode: 0x79, cycles: 4, address_mode: AddressMode::AbsoluteY },
        Instruction { opcode: 0x61, cycles: 6, address_mode: AddressMode::IndirectX },
        Instruction { opcode: 0x71, cycles: 5, address_mode: AddressMode::IndirectY },

        Instruction { opcode: 0xE9, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xE5, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xF5, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0xED, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0xFD, cycles: 4, address_mode: AddressMode::AbsoluteX },
        Instruction { opcode: 0xF9, cycles: 4, address_mode: AddressMode::AbsoluteY },
        Instruction { opcode: 0xE1, cycles: 6, address_mode: AddressMode::IndirectX },
        Instruction { opcode: 0xF1, cycles: 5, address_mode: AddressMode::IndirectY },

        Instruction { opcode: 0x29, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0x25, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x35, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x2D, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x3D, cycles: 4, address_mode: AddressMode::AbsoluteX },
        Instruction { opcode: 0x39, cycles: 4, address_mode: AddressMode::AbsoluteY },
        Instruction { opcode: 0x21, cycles: 6, address_mode: AddressMode::IndirectX },
        Instruction { opcode: 0x31, cycles: 5, address_mode: AddressMode::IndirectY },

        Instruction { opcode: 0x49, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0x45, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x55, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x4D, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x5D, cycles: 4, address_mode: AddressMode::AbsoluteX },
        Instruction { opcode: 0x59, cycles: 4, address_mode: AddressMode::AbsoluteY },
        Instruction { opcode: 0x41, cycles: 6, address_mode: AddressMode::IndirectX },
        Instruction { opcode: 0x51, cycles: 5, address_mode: AddressMode::IndirectY },

        Instruction { opcode: 0x09, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0x05, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x15, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x0D, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x1D, cycles: 4, address_mode: AddressMode::AbsoluteX },
        Instruction { opcode: 0x19, cycles: 4, address_mode: AddressMode::AbsoluteY },
        Instruction { opcode: 0x01, cycles: 6, address_mode: AddressMode::IndirectX },
        Instruction { opcode: 0x11, cycles: 5, address_mode: AddressMode::IndirectY },

        Instruction { opcode: 0xC9, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xC5, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xD5, cycles: 4, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0xCD, cycles: 4, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0xDD, cycles: 4, address_mode: AddressMode::AbsoluteX },
        Instruction { opcode: 0xD9, cycles: 4, address_mode: AddressMode::AbsoluteY },
        Instruction { opcode: 0xC1, cycles: 6, address_mode: AddressMode::IndirectX },
        Instruction { opcode: 0xD1, cycles: 5, address_mode: AddressMode::IndirectY },

        Instruction { opcode: 0xE0, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xE4, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xEc, cycles: 4, address_mode: AddressMode::Absolute },

        Instruction { opcode: 0xC0, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xC4, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xCC, cycles: 4, address_mode: AddressMode::Absolute },

        Instruction { opcode: 0x24, cycles: 3, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x2C, cycles: 4, address_mode: AddressMode::Absolute },

        Instruction { opcode: 0xE6, cycles: 5, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xF6, cycles: 6, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0xEE, cycles: 6, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0xFE, cycles: 7, address_mode: AddressMode::AbsoluteX },

        Instruction { opcode: 0xC6, cycles: 5, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0xD6, cycles: 6, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0xCE, cycles: 6, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0xDE, cycles: 7, address_mode: AddressMode::AbsoluteX },

        Instruction { opcode: 0xE8, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0xC8, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0xCA, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x88, cycles: 2, address_mode: AddressMode::Implied },

        Instruction { opcode: 0x0A, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x06, cycles: 5, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x16, cycles: 6, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x0E, cycles: 6, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x1E, cycles: 7, address_mode: AddressMode::AbsoluteX },

        Instruction { opcode: 0x4A, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x46, cycles: 5, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x56, cycles: 6, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x4E, cycles: 6, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x5E, cycles: 7, address_mode: AddressMode::AbsoluteX },

        Instruction { opcode: 0x2A, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x26, cycles: 5, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x36, cycles: 6, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x2E, cycles: 6, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x3E, cycles: 7, address_mode: AddressMode::AbsoluteX },

        Instruction { opcode: 0x6A, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x66, cycles: 5, address_mode: AddressMode::ZeroPage },
        Instruction { opcode: 0x76, cycles: 6, address_mode: AddressMode::ZeroPageX },
        Instruction { opcode: 0x6E, cycles: 6, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x7E, cycles: 7, address_mode: AddressMode::AbsoluteX },

        Instruction { opcode: 0x4C, cycles: 3, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x6C, cycles: 5, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x20, cycles: 6, address_mode: AddressMode::Absolute },
        Instruction { opcode: 0x40, cycles: 6, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x60, cycles: 6, address_mode: AddressMode::Implied },

        Instruction { opcode: 0x10, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0x30, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0x50, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0x70, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0x90, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xB0, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xD0, cycles: 2, address_mode: AddressMode::Immediate },
        Instruction { opcode: 0xF0, cycles: 2, address_mode: AddressMode::Immediate },

        Instruction { opcode: 0x18, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x58, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0xD8, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0xB8, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x38, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0x78, cycles: 2, address_mode: AddressMode::Implied },
        Instruction { opcode: 0xF8, cycles: 2, address_mode: AddressMode::Implied },

        Instruction { opcode: 0x00, cycles: 7, address_mode: AddressMode::Implied },

        Instruction { opcode: 0xEA, cycles: 2, address_mode: AddressMode::Implied },
    ];



    const STACK_PAGE :u16 = 0x100;

    fn push(&mut self, data: u8) -> u8 {
        let stack_address: u16 = Cpu::STACK_PAGE.wrapping_add(self.stack_ptr as u16) ;
        self.write_byte(stack_address, data);
        self.stack_ptr = self.stack_ptr.wrapping_sub(1);
        return 3;
    }

    fn pop(&mut self) -> (u8, u8) {
        self.stack_ptr = self.stack_ptr.wrapping_add(1);
        let stack_address: u16 = Cpu::STACK_PAGE.wrapping_add(self.stack_ptr as u16);
        let value = self.read_byte(stack_address);
        (value, 4)
    }

    fn arithmetic_register_a(&mut self, arithmetic_op: fn(a: u8, carry:u8, data:u8) -> u16, instruction: &Instruction) -> u8 {
        let (operand,is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let carry = if self.flag.contains(FlagRegister::CARRY) {1} else {0};

        let value = self.read_byte(operand);

        let result_16bit = arithmetic_op(self.a, carry, value);
        let result_8bit = result_16bit as u8;

        //Handle flags
        if result_16bit > 0xff {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        //If the two sign bits from a and value are the same but result is different we have overflow
        if (self.a ^ result_8bit) & (value ^ result_8bit) & 0x80 != 0 {
            self.flag.insert(FlagRegister::OVERFLOW);
        } else {
            self.flag.remove(FlagRegister::OVERFLOW);
        }

        self.update_zero_and_negative_flags(result_8bit);

        //Update result
        self.a = result_8bit;
        return if is_page_cross { instruction.cycles + 1 } else { instruction.cycles }
    }

    fn add (a: u8, carry:u8, data:u8) -> u16 {
        u16::from(a).wrapping_add(u16::from(carry)).wrapping_add(u16::from(data))
    }

    fn adc(&mut self, instruction: &Instruction) -> u8 {
        return self.arithmetic_register_a(Cpu::add, instruction);
    }

    fn sub (a: u8, carry:u8, data:u8) -> u16 {
        u16::from(a).wrapping_add(u16::from(carry)).wrapping_sub(1).wrapping_sub(u16::from(data))
    }

    fn sbc(&mut self, instruction: &Instruction) -> u8 {
        self.arithmetic_register_a(Cpu::sub, instruction)
    }


    fn logical_register_a(&mut self, logical_op: fn(a: u8, data:u8) -> u8, instruction: &Instruction) -> u8 {
        let (operand,is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let result = logical_op(self.a, value);

        //Handle flags
        self.update_zero_and_negative_flags(result);
        //Update result
        self.a = result;
        return if is_page_cross { instruction.cycles + 1 } else { instruction.cycles }
    }

    fn and(a: u8, data:u8) -> u8 {
        a & data
    }

    fn xor(a: u8, data:u8) -> u8 {
        a ^ data
    }

    fn or(a: u8, data:u8) -> u8 {
        a | data
    }

    fn compare(&mut self, instruction: &Instruction, compare_with: u8) -> u8 {
        let (operand,is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let result = compare_with.wrapping_sub(value);
        if compare_with >= value {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}

        self.update_zero_and_negative_flags(result);
        return if is_page_cross { instruction.cycles + 1 } else { instruction.cycles }
    }

    fn bit(&mut self, instruction: &Instruction) -> u8 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);
        if (self.a & value) == 0 {
            self.flag.insert(FlagRegister::ZERO);
        } else {
            self.flag.remove(FlagRegister::ZERO);
        }

        self.flag.set(FlagRegister::NEGATIV, value & 0b10000000 > 0);
        self.flag.set(FlagRegister::OVERFLOW, value & 0b01000000 > 0);
        return instruction.cycles;
    }

    fn increase_in_memory(&mut self, instruction: &Instruction) -> u8 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand).wrapping_add(1);
        self.write_byte(operand, value);
        self.update_zero_and_negative_flags(value);
        return instruction.cycles
    }

    fn decrease_in_memory(&mut self, instruction: &Instruction) -> u8 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand).wrapping_sub(1);
        self.write_byte(operand, value);
        self.update_zero_and_negative_flags(value);
        return instruction.cycles
    }

    fn shl(&mut self, instruction: &Instruction) -> u8 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let rot_value = value << 1;
        if (value & 0x80) != 0 {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    fn shl_acc(&mut self) -> u8 {
        let rot_value = self.a << 1;
        if (self.a & 0x80) != 0 {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    fn shr(&mut self, instruction: &Instruction) -> u8 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let rot_value = value >> 1;
        if (value & 0x01) != 0 {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    fn shr_acc(&mut self) -> u8 {
        let rot_value = self.a >> 1;
        if (self.a & 0x01) != 0 {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    fn rol(&mut self, instruction: &Instruction) -> u8 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) {1} else {0};

        let mut rot_value = value << 1;
        if (value & 0x80) != 0 {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        if old_carry == 1 {rot_value = rot_value | 0x01}

        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    fn rol_acc(&mut self) -> u8 {
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) {1} else {0};

        let mut rot_value = self.a << 1;
        if (self.a & 0x80) != 0 {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        if old_carry == 1 {rot_value = rot_value | 0x01}

        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    fn ror(&mut self, instruction: &Instruction) -> u8 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) {1} else {0};

        let mut rot_value = value << 1;
        if (value & 0x80) != 0 {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        if old_carry == 1 {rot_value = rot_value | 0x80}

        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    fn ror_acc(&mut self) -> u8 {
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) {1} else {0};

        let mut rot_value = self.a >> 1;
        if (self.a & 0x80) != 0 {self.flag.insert(FlagRegister::CARRY)} else {self.flag.remove(FlagRegister::CARRY)}
        if old_carry == 1 {rot_value = rot_value | 0x80}

        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    fn conditional_jump(&mut self, instruction: &Instruction, condition: bool) ->u8 {
        let offset : i8 = self.read_instruction_operand_8bit() as i8;
        if condition == true {
            let jump_addr = self.prog_counter.wrapping_add(offset as u16);
            let is_page_cross = Cpu::is_page_cross(self.prog_counter, jump_addr);
            self.prog_counter = jump_addr;
            return if is_page_cross { instruction.cycles + 2 } else { instruction.cycles + 1 };
        }
        else {
            return instruction.cycles;
        }

    }

    fn raise_interrupt(&mut self, interrupt_type: Interrupt) -> u8 {
        if matches!(interrupt_type,Interrupt::IRQ) && self.flag.contains(FlagRegister::IRQ_DISABLE) {
            return 0;
        }

        if matches!(interrupt_type,Interrupt::IRQ) {
            return self.reset();
        }

        let break_flag = match interrupt_type {
            Interrupt::IRQ|Interrupt::NMI => 0,
            Interrupt::RESET|BREAK => 1
        };
        if break_flag != 0 {self.flag.insert(FlagRegister::BREAK)} else {self.flag.remove(FlagRegister::BREAK)}

        let ret_pc = self.prog_counter;
        self.push((ret_pc >> 8) as u8);
        self.push((ret_pc & 0xFF) as u8);

        self.push(self.flag.bits);

        self.flag.insert(FlagRegister::IRQ_DISABLE);

        let interrupt_routine = match interrupt_type {
            Interrupt::IRQ|Interrupt::BREAK => 0xFFFE,
            Interrupt::NMI => 0xFFFA,
            Interrupt::RESET => 0xFFFC
        };
        let interrupt_fn = self.read_word(interrupt_routine);
        self.prog_counter = interrupt_fn;
        7
    }

    fn reset(&mut self) -> u8{
        self.flag.insert(FlagRegister::BREAK);
        //TODO fix flag register
        self.flag.insert(FlagRegister::IRQ_DISABLE);
        let interrupt_fn = self.read_word(0xFFFC);
        self.prog_counter = interrupt_fn;
        7
    }

    pub fn run(&mut self){


        let mut opcodes = HashMap::new();

        for opcode in Cpu::OPCODES {
            opcodes.insert(opcode.opcode, opcode);
        }

        loop {
            let opcode = self.read_byte(self.prog_counter);
            let instruction = opcodes.get(&opcode).unwrap();
            self.prog_counter+=1;

            let cycles = match opcode {
                //Register/Immediate to Register move
                0xA8 => {
                    self.y = self.a;
                    self.update_zero_and_negative_flags(self.y);
                    2
                },
                0xAA => {
                    self.x = self.a;
                    self.update_zero_and_negative_flags(self.x);
                    2
                },
                0xBA => {
                    self.x = self.stack_ptr;
                    self.update_zero_and_negative_flags(self.x);
                    2
                },
                0x98 => {
                    self.a = self.y;
                    self.update_zero_and_negative_flags(self.a);
                    2
                },
                0x8A => {
                    self.a = self.x;
                    self.update_zero_and_negative_flags(self.a);
                    2
                },
                0x9A => {
                    self.stack_ptr = self.x;
                    2
                },

                //LDA
                0xA9|0xA5|0xB5|0xAD|0xBD|0xB9|0xA1|0xB1 => self.lda(instruction),
                //LDX
                0xA2|0xA6|0xB6|0xAE|0xBE => self.ldx( instruction),
                //LDY
                0xA0|0xA4|0xB4|0xAC|0xBC => self.ldy(instruction),

                //STA
                0x85|0x95|0x8D|0x9D|0x99|0x81|0x91 => self.st(self.a, instruction),
                //STX
                0x86|0x96|0x8E=> self.st(self.x, instruction),
                //STY
                0x84|0x94|0x8C => self.st(self.y, instruction),

                //PHA
                0x48 => self.push(self.a),

                //PHP
                0x08 => self.push(self.flag.bits),

                //PLA
                0x68 => {
                    let (value, cycles) = self.pop();
                    self.a = value;
                    self.update_zero_and_negative_flags(self.a);
                    cycles
                },

                //PLP
                0x28 => {
                    let (value, cycles) = self.pop();
                    self.flag = FlagRegister::from_bits_truncate(value);
                    self.update_zero_and_negative_flags(self.flag.bits);
                    cycles
                },

                //ADC
                0x69|0x65|0x75|0x6D|0x7D|0x79|0x61|0x71 => self.adc(instruction),
                //SBC
                0xE9|0xE5|0xF5|0xED|0xFD|0xF9|0xE1|0xF1 => self.sbc(instruction),
                //AND
                0x29|0x25|0x35|0x2D|0x3D|0x39|0x21|0x31 => self.logical_register_a(Cpu::and, instruction),
                //XOR
                0x49|0x45|0x55|0x4D|0x5D|0x59|0x41|0x51 => self.logical_register_a(Cpu::xor, instruction),
                //OR
                0x09|0x05|0x15|0x0D|0x1D|0x19|0x01|0x11 => self.logical_register_a(Cpu::or, instruction),
                //CMP
                0xC9|0xC5|0xD5|0xCD|0xDD|0xD9|0xC1|0xD1 => self.compare(instruction, self.a),
                //CPX
                0xE0|0xE4|0xEC => self.compare(instruction, self.x),
                //CPY
                0xC0|0xC4|0xCC => self.compare(instruction, self.y),
                //BIT
                0x24|0x2C => self.bit(instruction),
                //INC
                0xE6|0xF6|0xEE|0xFE => self.increase_in_memory(instruction),
                //DEC
                0xC6|0xD6|0xCE|0xDE => self.decrease_in_memory(instruction),
                //INX
                0xE8 => {self.x+=1; 2}
                //INY
                0xC8 => {self.y+=1; 2}
                //DEX
                0xCA => {self.x-=1; 2}
                //DEY
                0x88 => {self.y-=1; 2}
                //ASL
                0x0A => self.shl_acc(),
                0x06|0x16|0x0E|0x1E => self.shl(instruction),
                //LSR
                0x4A => self.shr_acc(),
                0x46|0x56|0x4E|0x5E => self.shr(instruction),
                //ROL
                0x2A => self.rol_acc(),
                0x26|0x36|0x2E|0x3E => self.rol(instruction),
                //ROR
                0x6A => self.ror_acc(),
                0x66|0x76|0x6E|0x7E => self.ror(instruction),
                //JMP
                0x4C => {
                    let jump_addr = self.read_instruction_operand_16bit();
                    self.prog_counter = jump_addr;
                    3
                }
                0x6C => {
                    let jump_addr_ptr = self.read_instruction_operand_16bit();
                    let jump_addr = self.read_word(jump_addr_ptr);
                    self.prog_counter = jump_addr;
                    5
                }
                //JSR
                0x20 => {
                    let jump_addr = self.read_instruction_operand_16bit();
                    let ret_pc = self.prog_counter;
                    self.push((ret_pc >> 8) as u8);
                    self.push((ret_pc & 0xFF) as u8);
                    self.prog_counter = jump_addr;
                    6
                },
                //RTI
                0x40 => {
                    let (flag,_) = self.pop();
                    let current_brk = if self.flag.contains(FlagRegister::BREAK) {1} else {0};


                    self.flag = FlagRegister::from_bits_truncate(flag);
                    if current_brk != 0 {
                        self.flag.insert(FlagRegister::BREAK);
                    }
                    self.flag.insert(FlagRegister::UNUSED);

                    let (low_byte,_) = self.pop();
                    let (hi_byte, _) = self.pop();
                    let prog_counter = ((hi_byte as u16) << 8) | low_byte as u16;
                    self.prog_counter = prog_counter;
                    6
                },
                //RTS
                0x60 => {
                    let (low_byte,_) = self.pop();
                    let (hi_byte, _) = self.pop();
                    let prog_counter = ((hi_byte as u16) << 8) | low_byte as u16;
                    self.prog_counter = prog_counter;
                    6
                },
                //BPL
                0x10 => self.conditional_jump(instruction, !self.flag.contains(FlagRegister::NEGATIV)),
                //BMI
                0x30 => self.conditional_jump(instruction, self.flag.contains(FlagRegister::NEGATIV)),
                //BVC
                0x50 => self.conditional_jump(instruction, !self.flag.contains(FlagRegister::OVERFLOW)),
                //BVS
                0x70 => self.conditional_jump(instruction, self.flag.contains(FlagRegister::OVERFLOW)),
                //BCC
                0x90 => self.conditional_jump(instruction, !self.flag.contains(FlagRegister::CARRY)),
                //BCS
                0xB0 => self.conditional_jump(instruction, self.flag.contains(FlagRegister::CARRY)),
                //BNE
                0xD0 => self.conditional_jump(instruction, !self.flag.contains(FlagRegister::ZERO)),
                //BEQ
                0xF0 => self.conditional_jump(instruction, self.flag.contains(FlagRegister::ZERO)),
                //CLC
                0x18 => {self.flag.remove(FlagRegister::CARRY); instruction.cycles},
                //CLI
                0x58 => {self.flag.remove(FlagRegister::IRQ_DISABLE); instruction.cycles},
                //CLD
                0xD8 => {self.flag.remove(FlagRegister::DECIMAL_MODE); instruction.cycles},
                //CLV
                0xB8 => {self.flag.remove(FlagRegister::OVERFLOW); instruction.cycles},
                //SEC
                0x38 => {self.flag.insert(FlagRegister::CARRY); instruction.cycles},
                //SEI
                0x78 => {self.flag.insert(FlagRegister::IRQ_DISABLE); instruction.cycles},
                //SED
                0xF8 => {self.flag.insert(FlagRegister::DECIMAL_MODE); instruction.cycles},
                //BRK
                0x00 => self.raise_interrupt(Interrupt::BREAK),
                //NOP
                0xEA => instruction.cycles,
                _ => todo!()
            };



        }
    }

}


fn main() {

    println!("Hello, world!");
}
