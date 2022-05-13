use bitflags::bitflags;

mod opcodes;

use crate::{Bus};
use crate::cpu::opcodes::{AddressMode, Instruction, OPCODES};
use crate::memory::Memory;

enum Interrupt {
    NMI,
    IRQ,
    RESET,
    BREAK,
}

bitflags! {
    pub(self) struct FlagRegister: u8 {
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


pub struct Cpu<'a> {
    // Registers
    pub(self) a: u8,
    //Accumulator
    pub(self) x: u8,
    //Index register
    pub(self) y: u8,
    //Index register
    pub(self) stack_ptr: u8,
    //Stack Pointer
    pub(self) flag: FlagRegister,
    //Flag register
    pub(self) prog_counter: u16,
    //Program Counter,
    bus: &'a mut Bus<'a>,
}

impl Memory for Cpu<'_>{
    fn read_byte(&mut self, addr: u16) -> u8 {
        return self.bus.read_cpu_byte(addr);
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.bus.write_cpu_byte(addr, value);
    }
}

impl <'a> Cpu<'a> {
    pub fn new(bus: &'a mut Bus<'a>) -> Cpu<'a> {
        return Cpu {
            a: 0,
            x: 0,
            y: 0,
            stack_ptr: 0,
            flag: FlagRegister::from_bits_truncate(0),
            prog_counter: 0,
            bus,
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

    fn read_instruction_operand_8bit(&mut self) -> u8 {
        let operand = self.read_byte(self.prog_counter);
        self.prog_counter += 1;
        return operand;
    }

    fn read_instruction_operand_16bit(&mut self) -> u16 {
        let operand = self.read_word(self.prog_counter);
        self.prog_counter += 2;
        return operand;
    }

    fn is_page_cross(addr1: u16, addr2: u16) -> bool {
        return addr1 & 0xFF00 != addr2 & 0xFF00;
    }


    fn get_operand_address(&mut self, address_mode: &AddressMode) -> (u16, bool) {
        return match address_mode {
            AddressMode::Immediate => {
                let current_pc = self.prog_counter;
                self.prog_counter += 1;
                (current_pc, false)
            }
            AddressMode::ZeroPage => {
                let addr = self.read_instruction_operand_8bit();
                (addr as u16, false)
            }
            AddressMode::ZeroPageX => {
                let addr = self.read_instruction_operand_8bit();
                (addr.wrapping_add(self.x) as u16, false)
            }
            AddressMode::ZeroPageY => {
                let addr = self.read_instruction_operand_8bit();
                (addr.wrapping_add(self.y) as u16, false)
            }
            AddressMode::Absolute => {
                let addr = self.read_instruction_operand_16bit();
                (addr, false)
            }
            AddressMode::AbsoluteX => {
                let addr = self.read_instruction_operand_16bit();
                let final_address = addr.wrapping_add(self.x as u16);
                (final_address, Cpu::is_page_cross(addr, final_address))
            }
            AddressMode::AbsoluteY => {
                let addr = self.read_instruction_operand_16bit();
                let final_address = addr.wrapping_add(self.y as u16);
                (final_address, Cpu::is_page_cross(addr, final_address))
            }
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

    fn lda(&mut self, opcode: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
        return if is_page_cross { opcode.cycles + 1 } else { opcode.cycles }; //Based on opcode
    }

    fn ldx(&mut self, opcode: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.x = value;
        self.update_zero_and_negative_flags(self.x);
        return if is_page_cross { opcode.cycles + 1 } else { opcode.cycles }; //Based on opcode
    }

    fn ldy(&mut self, opcode: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.y = value;
        self.update_zero_and_negative_flags(self.y);
        return if is_page_cross { opcode.cycles + 1 } else { opcode.cycles }; //Based on opcode
    }

    fn st(&mut self, var: u8, opcode: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&opcode.address_mode);
        self.write_byte(operand, var);
        return if is_page_cross { opcode.cycles + 1 } else { opcode.cycles }; //Based on opcode
    }

    const STACK_PAGE: u16 = 0x100;

    fn push(&mut self, data: u8) -> u32 {
        let stack_address: u16 = Cpu::STACK_PAGE.wrapping_add(self.stack_ptr as u16);
        self.write_byte(stack_address, data);
        self.stack_ptr = self.stack_ptr.wrapping_sub(1);
        return 3;
    }

    fn pop(&mut self) -> (u8, u32) {
        self.stack_ptr = self.stack_ptr.wrapping_add(1);
        let stack_address: u16 = Cpu::STACK_PAGE.wrapping_add(self.stack_ptr as u16);
        let value = self.read_byte(stack_address);
        (value, 4)
    }

    fn arithmetic_register_a(&mut self, arithmetic_op: fn(a: u8, carry: u8, data: u8) -> u16, instruction: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let carry = if self.flag.contains(FlagRegister::CARRY) { 1 } else { 0 };

        let value = self.read_byte(operand);

        let result_16bit = arithmetic_op(self.a, carry, value);
        let result_8bit = result_16bit as u8;

        //Handle flags
        if result_16bit > 0xff { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        //If the two sign bits from a and value are the same but result is different we have overflow
        if (self.a ^ result_8bit) & (value ^ result_8bit) & 0x80 != 0 {
            self.flag.insert(FlagRegister::OVERFLOW);
        } else {
            self.flag.remove(FlagRegister::OVERFLOW);
        }

        self.update_zero_and_negative_flags(result_8bit);

        //Update result
        self.a = result_8bit;
        return if is_page_cross { instruction.cycles + 1 } else { instruction.cycles };
    }

    fn add(a: u8, carry: u8, data: u8) -> u16 {
        u16::from(a).wrapping_add(u16::from(carry)).wrapping_add(u16::from(data))
    }

    fn adc(&mut self, instruction: &Instruction) -> u32 {
        return self.arithmetic_register_a(Cpu::add, instruction);
    }

    fn sub(a: u8, carry: u8, data: u8) -> u16 {
        u16::from(a).wrapping_add(u16::from(carry)).wrapping_sub(1).wrapping_sub(u16::from(data))
    }

    fn sbc(&mut self, instruction: &Instruction) -> u32 {
        self.arithmetic_register_a(Cpu::sub, instruction)
    }


    fn logical_register_a(&mut self, logical_op: fn(a: u8, data: u8) -> u8, instruction: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let result = logical_op(self.a, value);

        //Handle flags
        self.update_zero_and_negative_flags(result);
        //Update result
        self.a = result;
        return if is_page_cross { instruction.cycles + 1 } else { instruction.cycles };
    }

    fn and(a: u8, data: u8) -> u8 {
        a & data
    }

    fn xor(a: u8, data: u8) -> u8 {
        a ^ data
    }

    fn or(a: u8, data: u8) -> u8 {
        a | data
    }

    fn compare(&mut self, instruction: &Instruction, compare_with: u8) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let result = compare_with.wrapping_sub(value);
        if compare_with >= value { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }

        self.update_zero_and_negative_flags(result);
        return if is_page_cross { instruction.cycles + 1 } else { instruction.cycles };
    }

    fn bit(&mut self, instruction: &Instruction) -> u32 {
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

    fn increase_in_memory(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand).wrapping_add(1);
        self.write_byte(operand, value);
        self.update_zero_and_negative_flags(value);
        return instruction.cycles;
    }

    fn decrease_in_memory(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand).wrapping_sub(1);
        self.write_byte(operand, value);
        self.update_zero_and_negative_flags(value);
        return instruction.cycles;
    }

    fn shl(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let rot_value = value << 1;
        if (value & 0x80) != 0 { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    fn shl_acc(&mut self) -> u32 {
        let rot_value = self.a << 1;
        if (self.a & 0x80) != 0 { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    fn shr(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let rot_value = value >> 1;
        if (value & 0x01) != 0 { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    fn shr_acc(&mut self) -> u32 {
        let rot_value = self.a >> 1;
        if (self.a & 0x01) != 0 { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    fn rol(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) { 1 } else { 0 };

        let mut rot_value = value << 1;
        if (value & 0x80) != 0 { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        if old_carry == 1 { rot_value = rot_value | 0x01 }

        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    fn rol_acc(&mut self) -> u32 {
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) { 1 } else { 0 };

        let mut rot_value = self.a << 1;
        if (self.a & 0x80) != 0 { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        if old_carry == 1 { rot_value = rot_value | 0x01 }

        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    fn ror(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) { 1 } else { 0 };

        let mut rot_value = value << 1;
        if (value & 0x80) != 0 { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        if old_carry == 1 { rot_value = rot_value | 0x80 }

        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    fn ror_acc(&mut self) -> u32 {
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) { 1 } else { 0 };

        let mut rot_value = self.a >> 1;
        if (self.a & 0x80) != 0 { self.flag.insert(FlagRegister::CARRY) } else { self.flag.remove(FlagRegister::CARRY) }
        if old_carry == 1 { rot_value = rot_value | 0x80 }

        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    fn conditional_jump(&mut self, instruction: &Instruction, condition: bool) -> u32 {
        let offset: i8 = self.read_instruction_operand_8bit() as i8;
        if condition == true {
            let jump_addr = self.prog_counter.wrapping_add(offset as u16);
            let is_page_cross = Cpu::is_page_cross(self.prog_counter, jump_addr);
            self.prog_counter = jump_addr;
            return if is_page_cross { instruction.cycles + 2 } else { instruction.cycles + 1 };
        } else {
            return instruction.cycles;
        }
    }

    fn raise_interrupt(&mut self, interrupt_type: Interrupt) -> u32 {
        if matches!(interrupt_type,Interrupt::IRQ) && self.flag.contains(FlagRegister::IRQ_DISABLE) {
            return 0;
        }

        if matches!(interrupt_type,Interrupt::IRQ) {
            self.reset();
            return 7;
        }

        let break_flag = match interrupt_type {
            Interrupt::IRQ | Interrupt::NMI => 0,
            Interrupt::RESET | Interrupt::BREAK => 1
        };
        if break_flag != 0 { self.flag.insert(FlagRegister::BREAK) } else { self.flag.remove(FlagRegister::BREAK) }

        let ret_pc = self.prog_counter;
        self.push((ret_pc >> 8) as u8);
        self.push((ret_pc & 0xFF) as u8);

        self.push(self.flag.bits);

        self.flag.insert(FlagRegister::IRQ_DISABLE);

        let interrupt_routine = match interrupt_type {
            Interrupt::IRQ | Interrupt::BREAK => 0xFFFE,
            Interrupt::NMI => 0xFFFA,
            Interrupt::RESET => 0xFFFC
        };
        let interrupt_fn = self.read_word(interrupt_routine);
        self.prog_counter = interrupt_fn;
        7
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stack_ptr = 0xFD;
        self.flag = FlagRegister::from_bits_truncate(0b100100);
        self.prog_counter = self.read_word(0xFFFC);
    }

    pub fn execute_instruction(&mut self) -> u32 {

        let opcode = self.read_byte(self.prog_counter);
        let instruction = &OPCODES[opcode as usize];
        println!("Executing {:x}: {:x} (a:{:x}, x:{:x}, y:{:x}, S:{:x}, P:{:x}  )", self.prog_counter, opcode, self.a, self.x, self.y, self.stack_ptr, self.flag);
        self.prog_counter = self.prog_counter.wrapping_add(1);

        let cycles = match opcode {
            //Register/Immediate to Register move
            0xA8 => {
                self.y = self.a;
                self.update_zero_and_negative_flags(self.y);
                2
            }
            0xAA => {
                self.x = self.a;
                self.update_zero_and_negative_flags(self.x);
                2
            }
            0xBA => {
                self.x = self.stack_ptr;
                self.update_zero_and_negative_flags(self.x);
                2
            }
            0x98 => {
                self.a = self.y;
                self.update_zero_and_negative_flags(self.a);
                2
            }
            0x8A => {
                self.a = self.x;
                self.update_zero_and_negative_flags(self.a);
                2
            }
            0x9A => {
                self.stack_ptr = self.x;
                2
            }

            //LDA
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(instruction),
            //LDX
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(instruction),
            //LDY
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(instruction),

            //STA
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => self.st(self.a, instruction),
            //STX
            0x86 | 0x96 | 0x8E => self.st(self.x, instruction),
            //STY
            0x84 | 0x94 | 0x8C => self.st(self.y, instruction),

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
            }

            //PLP
            0x28 => {
                let (value, cycles) = self.pop();
                self.flag = FlagRegister::from_bits_truncate(value);
                self.update_zero_and_negative_flags(self.flag.bits);
                cycles
            }

            //ADC
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(instruction),
            //SBC
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(instruction),
            //AND
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.logical_register_a(Cpu::and, instruction),
            //XOR
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.logical_register_a(Cpu::xor, instruction),
            //OR
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.logical_register_a(Cpu::or, instruction),
            //CMP
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.compare(instruction, self.a),
            //CPX
            0xE0 | 0xE4 | 0xEC => self.compare(instruction, self.x),
            //CPY
            0xC0 | 0xC4 | 0xCC => self.compare(instruction, self.y),
            //BIT
            0x24 | 0x2C => self.bit(instruction),
            //INC
            0xE6 | 0xF6 | 0xEE | 0xFE => self.increase_in_memory(instruction),
            //DEC
            0xC6 | 0xD6 | 0xCE | 0xDE => self.decrease_in_memory(instruction),
            //INX
            0xE8 => {
                self.x = self.x.wrapping_add(1);
                self.update_zero_and_negative_flags(self.x);
                2
            }
            //INY
            0xC8 => {
                self.y = self.y.wrapping_add(1);
                self.update_zero_and_negative_flags(self.y);
                2
            }
            //DEX
            0xCA => {
                self.x = self.x.wrapping_sub(1);
                self.update_zero_and_negative_flags(self.x);
                2
            }
            //DEY
            0x88 => {
                self.y = self.y.wrapping_sub(1);
                self.update_zero_and_negative_flags(self.y);
                2
            }
            //ASL
            0x0A => self.shl_acc(),
            0x06 | 0x16 | 0x0E | 0x1E => self.shl(instruction),
            //LSR
            0x4A => self.shr_acc(),
            0x46 | 0x56 | 0x4E | 0x5E => self.shr(instruction),
            //ROL
            0x2A => self.rol_acc(),
            0x26 | 0x36 | 0x2E | 0x3E => self.rol(instruction),
            //ROR
            0x6A => self.ror_acc(),
            0x66 | 0x76 | 0x6E | 0x7E => self.ror(instruction),
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
            }
            //RTI
            0x40 => {
                let (flag, _) = self.pop();
                let current_brk = if self.flag.contains(FlagRegister::BREAK) { 1 } else { 0 };


                self.flag = FlagRegister::from_bits_truncate(flag);
                if current_brk != 0 {
                    self.flag.insert(FlagRegister::BREAK);
                }
                self.flag.insert(FlagRegister::UNUSED);

                let (low_byte, _) = self.pop();
                let (hi_byte, _) = self.pop();
                let prog_counter = ((hi_byte as u16) << 8) | low_byte as u16;
                self.prog_counter = prog_counter;
                6
            }
            //RTS
            0x60 => {
                let (low_byte, _) = self.pop();
                let (hi_byte, _) = self.pop();
                let prog_counter = ((hi_byte as u16) << 8) | low_byte as u16;
                self.prog_counter = prog_counter;
                6
            }
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
            0x18 => {
                self.flag.remove(FlagRegister::CARRY);
                instruction.cycles
            }
            //CLI
            0x58 => {
                self.flag.remove(FlagRegister::IRQ_DISABLE);
                instruction.cycles
            }
            //CLD
            0xD8 => {
                self.flag.remove(FlagRegister::DECIMAL_MODE);
                instruction.cycles
            }
            //CLV
            0xB8 => {
                self.flag.remove(FlagRegister::OVERFLOW);
                instruction.cycles
            }
            //SEC
            0x38 => {
                self.flag.insert(FlagRegister::CARRY);
                instruction.cycles
            }
            //SEI
            0x78 => {
                self.flag.insert(FlagRegister::IRQ_DISABLE);
                instruction.cycles
            }
            //SED
            0xF8 => {
                self.flag.insert(FlagRegister::DECIMAL_MODE);
                instruction.cycles
            }
            //BRK
            0x00 => self.raise_interrupt(Interrupt::BREAK),
            //NOP
            0xEA => {
                instruction.cycles
            },
            _ => todo!()
        };
        return cycles;
    }
}
