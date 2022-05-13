use crate::bus::ppu::{PPUCTRL, PPUMASK, PPUSTATUS};
use crate::cartridge::Cartridge;
use crate::memory::Memory;

mod ppu;

pub struct Bus<'a> {
    cpu_memory: [u8; 0x800],
    cartridge: Option<&'a Cartridge>,

    ppuctrl: PPUCTRL,
    ppumask: PPUMASK,
    ppustatus: PPUSTATUS,
    oam_addr: u8,
    oam_ram: [u8; 0xff]
}

impl Memory for Bus<'_> {
    fn read_byte(&mut self, addr: u16) -> u8 {
        return match addr {
            0x0000..=0x1FFF => {
                //Ram is mirrored in this space
                let ram_addr = (addr % 0x800) as usize;
              self.cpu_memory[ram_addr]
            },
            0x2000..=0x3FFF => {
                //This area is a mirror for 0x2000 - 0x2007
                let ppu_io_addr = (addr - 0x2000) % 0x8;
                println!("PPU read {:x}", ppu_io_addr + 0x2000);
                return match ppu_io_addr {
                    0|1|3 => 0,
                    2 => self.ppustatus.bits(),
                    4 => {
                        let oam_addr = self.oam_addr;
                        self.oam_ram[oam_addr as usize]
                    }
                    _ => {
                        0
                    }
                };
            },
            //APU I/O
            0x4000..=0x401F => {
                0
            },
            //Expansion ROM (only certain mappers
            0x4020..=0x5FFF => {
                0
            },
            //Cart RAM
            0x6000..=0x7FFF =>{
                0
            }
            0x8000..=0xFFFF => {
                //TODO handle cart mapper
                let cartridge = self.cartridge.unwrap();
                let pkg_rom = &cartridge.pkg_rom;
                let pkg_rom_size = cartridge.pkg_rom_size;

                //If ROM only has 1 page, it's mirrored into 0xC000 - 0xFFFF
                let rom_addr = (addr - 0x8000) %  if pkg_rom_size == 0x4000 {0x4000} else {0x8000};
                *pkg_rom.get(rom_addr as usize).unwrap()
            }
        };
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                //Ram is mirrored in this space
                let ram_addr = (addr % 0x800) as usize;
                self.cpu_memory[ram_addr] = value;
            },
            0x2000..=0x3FFF => {
                //This area is a mirror for 0x2000 - 0x2007
                let ppu_io_addr = (addr - 0x2000) % 0x8;
                println!("PPU write {:x}", ppu_io_addr + 0x2000);
                return match ppu_io_addr {
                    0 => {
                        self.ppuctrl = PPUCTRL::from_bits_truncate(value);
                    },
                    1 => {
                        self.ppumask = PPUMASK::from_bits_truncate(value);
                    }
                    //OAM ADDR
                    3 => {
                        self.oam_addr = value;
                    }
                    //OAM DATA
                    4 => {
                        let addr = self.oam_addr;
                        self.oam_ram[addr as usize] = value;
                        self.oam_addr = addr.wrapping_add(1);
                    }
                    2 => {
                        //NOP
                    }
                    _ => {
                    }
                };
            },
            //APU I/O
            0x4000..=0x401F => {

            },
            //Expansion ROM (only certain mappers
            0x4020..=0x5FFF => {

            },
            //Cart RAM
            0x6000..=0x7FFF =>{

            }
            0x8000..=0xFFFF => {
                //TODO this should be different for each cart
            }
        }
    }
}

impl <'a> Bus<'a> {
    pub fn new() -> Bus<'a> {
        return Bus {
            cpu_memory: [0xA8; 0x800],
            cartridge: None,

            ppuctrl: PPUCTRL::from_bits_truncate(0),
            ppumask: PPUMASK::from_bits_truncate(0),
            ppustatus: PPUSTATUS::from_bits_truncate(0b11111111),
            oam_addr: 0,
            oam_ram: [0x0; 0xff]
        };
    }

    pub fn insert_cartridge(&mut self, cart: &'a Cartridge){
        self.cartridge = Some(cart);
    }


}