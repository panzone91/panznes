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
    oam_ram: [u8; 0xff],

    ppu_second_write: bool,

    horizontal_scroll_origin: u8,
    vertical_scroll_origin: u8,

    vram_addr: u16,
    ppu_memory: [u8; 0x10000],

    request_dma: bool,
    dma_src: u16
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
            oam_ram: [0x0; 0xff],

            ppu_second_write: false,

            horizontal_scroll_origin: 0,
            vertical_scroll_origin: 0,

            vram_addr: 0,

            ppu_memory: [0x0; 0x10000],

            request_dma: false,
            dma_src: 0
        };
    }

    pub fn insert_cartridge(&mut self, cart: &'a Cartridge){
        self.cartridge = Some(cart);
    }

    pub fn read_cpu_byte(&mut self, addr: u16) -> u8 {
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
                    //todo should panic?
                    0|1|3|5|6 => 0,
                    2 => self.ppustatus.bits(),
                    4 => {
                        let oam_addr = self.oam_addr;
                        self.oam_ram[oam_addr as usize]
                    },
                    7 => {
                        let vram_addr = self.vram_addr;
                        let value = self.read_ppu_byte(vram_addr);
                        //Increase vram_addr based on VRAM_INCREMENT bit
                        let horizontal_increment = self.ppuctrl.contains(PPUCTRL::VRAM_INCREMENT);
                        self.vram_addr = vram_addr + if horizontal_increment { 1 } else { 32 };
                        value
                    },
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

    pub fn write_cpu_byte(&mut self, addr: u16, value: u8) {
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
                    },
                    5 => {
                        let second_write = self.ppu_second_write;
                        if second_write {
                            self.vertical_scroll_origin = value
                        } else {
                            self.horizontal_scroll_origin = value
                        }
                        self.ppu_second_write = !second_write
                    },
                    6 => {
                        let second_write = self.ppu_second_write;
                        let addr = self.vram_addr;
                        let new_addr =
                            if !second_write {
                                (value as u16 & 0x3F) << 8
                            } else {
                                addr | u16::from(value)
                            };

                        self.vram_addr = new_addr;
                        self.ppu_second_write = !second_write
                    },
                    7 => {
                        let vram_addr = self.vram_addr;
                        self.write_ppu_byte(vram_addr, value);
                        //Increase vram_addr based on VRAM_INCREMENT bit
                        let horizontal_increment = self.ppuctrl.contains(PPUCTRL::VRAM_INCREMENT);
                        self.vram_addr = vram_addr + if horizontal_increment { 1 } else { 32 };
                    },
                    2 => {
                        //NOP
                    }
                    _ => {
                    }
                };
            },
            //APU I/O
            0x4000..=0x4013|0x4015..=0x401F => {

            },
            0x4014 => {
                self.request_dma = true;
                self.dma_src = u16::from(value) << 8;
            }
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

    pub fn handle_dma_request(&mut self) {
        let src_addr = usize::from(self.dma_src);
        //TODO check if there is a better way, maybe a slice?
        for i in 0..256 {
            self.oam_ram[i] = self.cpu_memory[src_addr + i];
        }
        self.request_dma = false;
    }

    pub fn read_ppu_byte(&mut self, addr: u16) -> u8 {
        self.ppu_memory[addr as usize]
    }

    pub fn write_ppu_byte(&mut self, addr: u16, value: u8){
        self.ppu_memory[addr as usize] = value;
    }

}