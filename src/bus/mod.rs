use crate::cartridge::Cartridge;
use crate::memory::Memory;

pub struct Bus<'a> {
    cpu_memory: [u8; 0x10000],
    cartridge: Option<&'a Cartridge>
}

impl Memory for Bus<'_> {
    fn read_byte(&mut self, addr: u16) -> u8 {
        return match addr {
            0x8000..=0xFFFF => {
                let rom_addr = addr - 0x8000;
                let pkg_rom = &self.cartridge.unwrap().pkg_rom;
                if(pkg_rom.len() == 0x4000 && rom_addr >= 0x4000) {
                    return *pkg_rom.get((rom_addr % 0x4000) as usize).unwrap();
                } else {
                    return *pkg_rom.get(rom_addr as usize).unwrap();
                }
            },
            _ => self.cpu_memory[addr as usize]
        };


        return self.cpu_memory[addr as usize];
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.cpu_memory[addr as usize] = value;
    }
}

impl <'a> Bus<'a> {
    pub fn new() -> Bus<'a> {
        return Bus {
            cpu_memory: [0xA8; 0x10000],
            cartridge: None
        };
    }

    pub fn insert_cartridge(&mut self, cart: &'a Cartridge){
        self.cartridge = Some(cart);
    }


}