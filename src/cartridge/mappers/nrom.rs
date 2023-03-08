use crate::cartridge::{CartridgeMirroring, Mapper};

pub struct NROM {
    pub pkg_rom: Vec<u8>,
    pub pkg_rom_size: usize,
    pub chr_rom: Vec<u8>,
    pub chr_rom_size: usize,
    pub namespace_mirroring: CartridgeMirroring,
    pub mapper: u8,
}

impl Mapper for NROM {
    fn read_pkg_byte(&mut self, addr: u16) -> u8 {
        let pkg_rom_size = self.pkg_rom_size;

        //If ROM only has 1 page, it's mirrored into 0xC000 - 0xFFFF
        let rom_addr = addr
            % if pkg_rom_size == 0x4000 {
                0x4000
            } else {
                0x8000
            };
        return *self.pkg_rom.get(rom_addr as usize).unwrap();
    }

    fn write_pkg_byte(&mut self, _addr: u16, _value: u8) {}

    fn read_chr_byte(&mut self, addr: u16) -> u8 {
        return self.chr_rom[addr as usize];
    }

    fn write_chr_byte(&mut self, _addr: u16, _value: u8) {}

    fn get_namespace_mirroring(&mut self) -> CartridgeMirroring {
        return self.namespace_mirroring.clone();
    }
}
