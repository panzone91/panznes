pub struct Cartridge{
    pub pkg_rom: Vec<u8>,
    pub pkg_rom_size: usize
}

impl Cartridge {
    pub fn from_ines(rom: &Vec<u8>) -> Cartridge {
        //TODO check header
        let pkg_rom_size = rom[4] as usize;

        let pkg_rom = rom.get(16..16 +(16384 * pkg_rom_size)).unwrap().to_vec();
        println!("rom size {}",pkg_rom.len());
        Cartridge {
            pkg_rom,
            pkg_rom_size
        }
    }


}