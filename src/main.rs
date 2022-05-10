pub struct CpuMemory {
    memory: [u8; 0xFFFF]
}

pub struct Bus {


}

trait Memory {
    fn read_byte(&mut self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);
    fn read_word(&mut self, addr: u16) -> u16;
    fn write_word(&mut self,addr: u16, value: u16);
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


    pub fn run(&mut self){

        fn is_page_cross(addr1: u16, addr2: u16) -> bool {
            return addr1 & 0xFF != addr2 & 0xFF;
        }


        loop {
            let opcode = self.read_byte(self.prog_counter);
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
                    self.a = self.read_byte(address as u16);
                    self.update_zero_and_negative_flags(self.a);
                    3
                },
                0xB5 => {
                    let address = self.read_instruction_operand_8bit().wrapping_add(self.x);
                    self.a = self.read_byte(address as u16);
                    self.update_zero_and_negative_flags(self.a);
                    4
                },
                0xAD => {
                    let address = self.read_instruction_operand_16bit();
                    self.a = self.read_byte(address);
                    self.update_zero_and_negative_flags(self.a);
                    4
                },
                0xBD => {
                    let operand_address = self.read_instruction_operand_16bit();
                    let final_address = operand_address.wrapping_add(self.x as u16);
                    self.a = self.read_byte(final_address);
                    self.update_zero_and_negative_flags(self.a);
                    if is_page_cross(operand_address, final_address) {5} else {4}
                },
                0xB9 => {
                    let operand_address = self.read_instruction_operand_16bit();
                    let final_address = operand_address.wrapping_add(self.y as u16);
                    self.a = self.read_byte(final_address);
                    self.update_zero_and_negative_flags(self.a);
                    if is_page_cross(operand_address, final_address) {5} else {4}
                },
                0xA1 => {
                    let operand = self.read_instruction_operand_8bit();
                    let address = self.read_word(operand.wrapping_add(self.x) as u16);
                    self.a = self.read_byte(address);
                    self.update_zero_and_negative_flags(self.a);
                    6
                },
                0xB1 => {
                    let operand = self.read_instruction_operand_8bit();
                    let address = self.read_word(operand as u16);
                    let final_address = address.wrapping_add(self.y as u16);
                    self.a = self.read_byte(final_address as u16);
                    self.update_zero_and_negative_flags(self.a);
                    if is_page_cross(address, final_address) {6} else {5}
                },
                0xA6 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.x = self.read_byte(operand as u16);
                    self.update_zero_and_negative_flags(self.x);
                    3
                },
                0xB6 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.x = self.read_byte((operand.wrapping_add(self.y)) as u16);
                    self.update_zero_and_negative_flags(self.x);
                    6
                },
                0xAE => {
                    let address = self.read_instruction_operand_16bit();
                    self.x = self.read_byte(address);
                    self.update_zero_and_negative_flags(self.x);
                    4
                },
                0xBE => {
                    let address = self.read_instruction_operand_16bit();
                    let final_address = address.wrapping_add(self.y as u16);
                    self.x = self.read_byte(final_address);
                    self.update_zero_and_negative_flags(self.x);
                    if is_page_cross(address, final_address) {5} else {4}
                },
                0xA4 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.y = self.read_byte(operand as u16);
                    self.update_zero_and_negative_flags(self.y);
                    3
                },
                0xB4 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.y = self.read_byte(operand.wrapping_add(self.x) as u16);
                    self.update_zero_and_negative_flags(self.y);
                    4
                },
                0xAC => {
                    let address = self.read_instruction_operand_16bit();
                    self.y = self.read_byte(address);
                    self.update_zero_and_negative_flags(self.y);
                    4
                },
                0xBC => {
                    let address = self.read_instruction_operand_16bit();
                    let final_address = address.wrapping_add(self.x as u16);
                    self.y = self.read_byte(final_address);
                    self.update_zero_and_negative_flags(self.y);
                    if is_page_cross(address, final_address) {5} else {4}
                },
                //Store registry in memory
                0x85 => {
                    let address = self.read_instruction_operand_8bit();
                    self.write_byte(address as u16, self.a);
                    3
                },
                0x95 => {
                    let address = self.read_instruction_operand_8bit() + self.x;
                    self.write_byte(address as u16, self.a);
                    4
                },
                0x8D => {
                    let address = self.read_instruction_operand_16bit();
                    self.write_byte(address, self.a);
                    4
                },
                0x9D => {
                    let address = self.read_instruction_operand_16bit() + self.x as u16;
                    self.write_byte(address, self.a);
                    5
                },
                0x99 => {
                    let address = self.read_instruction_operand_16bit() + self.y as u16;
                    self.write_byte(address, self.a);
                    5
                },
                0x81 => {
                    let operand = self.read_instruction_operand_8bit();
                    let address = self.read_word(operand.wrapping_add(self.x) as u16);
                    self.write_byte(address, self.a);
                    6
                },
                0x91 => {
                    let operand = self.read_instruction_operand_8bit();
                    let address = self.read_word(operand as u16);
                    self.write_byte(address.wrapping_add(self.y as u16), self.a);
                    6
                },
                0x86 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.write_byte(operand as u16, self.x);
                    3
                },
                0x96 => {
                    let address = self.read_instruction_operand_8bit().wrapping_add(self.y);
                    self.write_byte(address as u16, self.x);
                    4
                },
                0x8E => {
                    let address = self.read_instruction_operand_16bit();
                    self.write_byte(address, self.x);
                    4
                },
                0x84 => {
                    let operand = self.read_instruction_operand_8bit();
                    self.write_byte(operand as u16, self.y);
                    3
                },
                0x94 => {
                    let address = self.read_instruction_operand_8bit().wrapping_add(self.x);
                    self.write_byte(address as u16, self.y);
                    4
                },
                0x8C => {
                    let address = self.read_instruction_operand_16bit();
                    self.write_byte(address, self.y);
                    4
                },
                //Push/pull
                0x48 => {
                    let stack_address: u16 = 0x100 + (self.stack_ptr as u16) ;
                    self.write_byte(stack_address, self.a);
                    self.stack_ptr.wrapping_sub(1);
                    3
                },
                0x08 => {
                    let stack_address: u16 = 0x100 + (self.stack_ptr as u16) ;
                    self.write_byte(stack_address, self.flag.bits);
                    self.stack_ptr.wrapping_sub(1);
                    3
                },
                0x68 => {
                    self.stack_ptr.wrapping_add(1);
                    let stack_address: u16 = 0x100 + (self.stack_ptr as u16);
                    self.a = self.read_byte(stack_address);
                    self.update_zero_and_negative_flags(self.a);
                    4
                },
                0x28 => {
                    self.stack_ptr.wrapping_add(1);
                    let stack_address: u16 = 0x100 + (self.stack_ptr as u16);
                    self.flag = FlagRegister::from_bits_truncate(self.read_byte(stack_address));
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
