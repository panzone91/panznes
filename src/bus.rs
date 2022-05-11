use crate::memory::Memory;

pub struct Bus {
    cpu_memory: [u8; 0x10000]
}

impl Memory for Bus {
    fn read_byte(&mut self, addr: u16) -> u8 {
        return self.cpu_memory[addr as usize];
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.cpu_memory[addr as usize] = value;
    }
}

impl Bus {
    pub fn new() -> Bus {
        return Bus {
            cpu_memory: [0xEA; 0x10000]
        };
    }
}