pub struct CpuMemory {
    memory: [u8; 0xFFFF]
}

pub struct Bus {


}

trait Memory {
    fn read_byte(addr: u16) -> u8;
    fn write_byte(addr: u16, value:u8);
    fn read_word(addr: u16) -> u16;
    fn write_word(addr: u16, value:u16);
}

use bitflags::bitflags;

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
    fn read_byte(addr: u16) -> u8 {
        todo!()
    }

    fn write_byte(addr: u16, value: u8) {
        todo!()
    }

    fn read_word(addr: u16) -> u16 {
        todo!()
    }

    fn write_word(addr: u16, value: u16) {
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
        let operand = Cpu::read_byte(self.prog_counter);
        self.prog_counter+=1;
        return operand;
    }

    fn read_instruction_operand_16bit(&mut self) -> u16{
        let operand = Cpu::read_word(self.prog_counter);
        self.prog_counter+=2;
        return operand;
    }

    pub fn run(&mut self){
        loop {
            let opcode = Cpu::read_byte(self.prog_counter);
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
                0xA9 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.a = operand;
                    self.update_zero_and_negative_flags(self.a);
                    2
                },
                0xA2 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.x = operand;
                    self.update_zero_and_negative_flags(self.x);
                    2
                },
                0xA0 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.y = operand;
                    self.update_zero_and_negative_flags(self.y);
                    2
                },
                // Load Register from memory
                0xA5 => {
                    let address = self.read_instruction_operand_8bit();
                    self.a = Cpu::read_byte(address as u16);
                    self.update_zero_and_negative_flags(self.a);
                    3
                },
                0xB5 => {
                    let address = self.read_instruction_operand_8bit() + self.x;
                    self.a = Cpu::read_byte(address as u16);
                    self.update_zero_and_negative_flags(self.a);
                    4
                },
                0xAD => {
                    let address = self.read_instruction_operand_16bit();
                    self.a = Cpu::read_byte(address);
                    self.update_zero_and_negative_flags(self.a);
                    4
                },

                _ => todo!()
            };



        }
    }

}


fn main() {

    let nes= Cpu::new();

    nes.run();

    println!("Hello, world!");
}
