mod mappers;

use crate::cartridge::mappers::nrom::NROM;

pub trait Mapper {
    fn read_pkg_byte(&mut self, addr: u16) -> u8;
    fn write_pkg_byte(&mut self, addr: u16, value: u8);

    fn read_chr_byte(&mut self, addr: u16) -> u8;
    fn write_chr_byte(&mut self, addr: u16, value: u8);

    fn get_namespace_mirroring(&mut self) -> CartridgeMirroring;
}

#[derive(Copy, Clone)]
pub enum CartridgeMirroring {
    HORIZONTAL,
    VERTICAL,
}

pub fn from_ines(rom: &Vec<u8>) -> impl Mapper {
    //TODO check header
    let pkg_rom_size = rom[4] as usize * 16384;
    let chr_rom_size = rom[5] as usize * 8192;
    let flag6 = rom[6];
    let flag7 = rom[7];

    let mapper = (flag7 & 0xF0) | ((flag6 & 0xF0) >> 4);

    println!("file size {}", rom.len());
    println!(
        "pkg_rom_size {}, chr_rom_size {}, mapper {}",
        pkg_rom_size, chr_rom_size, mapper
    );

    let pkg_rom_start_index = 16;
    let chr_rom_start_index = pkg_rom_start_index + pkg_rom_size;

    let pkg_rom = rom[pkg_rom_start_index..pkg_rom_start_index + pkg_rom_size].to_vec();
    //The cartridge could use chr_ram...
    let chr_rom = if chr_rom_size == 0 {
        Vec::with_capacity(0xFFFF)
    } else {
        rom[chr_rom_start_index..chr_rom_start_index + chr_rom_size].to_vec()
    };

    let namespace_mirroring = if flag6 & 0x1 == 0 {
        CartridgeMirroring::HORIZONTAL
    } else {
        CartridgeMirroring::VERTICAL
    };

    println!("rom size {}", pkg_rom.len());
    NROM {
        pkg_rom,
        pkg_rom_size,
        chr_rom,
        chr_rom_size,
        namespace_mirroring,
        mapper,
    }
}
