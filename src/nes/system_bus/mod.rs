use crate::nes::{Interrupt, Nes};

use bitflags::bitflags;

/*
7  bit  0
---- ----
VPHB SINN
|||| ||||
|||| ||++- Base nametable address (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
|||| |+--- VRAM address increment per CPU read/write of PPUDATA
|||| |     (0: add 1, going across; 1: add 32, going down)
|||| +---- Sprite pattern table address for 8x8 sprites
||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
|||+------ Background pattern table address (0: $0000; 1: $1000)
||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
|+-------- PPU master/slave select
+--------- Generate an NMI at the start of the vertical blanking interval (0: off; 1: on)
 */

bitflags! {
    pub struct PPUCTRL: u8 {
        const NAMETABLE_ADDRESS         = 0b00000011;
        const VRAM_INCREMENT            = 0b00000100;
        const SPRITE_PATTERN_TABLE      = 0b00001000;
        const BACKGROUND_PATTERN_TABLE  = 0b00010000;
        const SPRITE_SIZE_16            = 0b00100000;
        const PPU_MASTER_SLAVE          = 0b01000000;
        const NMI_ENABLED               = 0b10000000;
    }
}

/*
7  bit  0
---- ----
BGRs bMmG
|||| ||||
|||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
|||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
|||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
|||| +---- 1: Show background
|||+------ 1: Show sprites
||+------- Emphasize red (green on PAL/Dendy)
|+-------- Emphasize green (red on PAL/Dendy)
+--------- Emphasize blue
*/
bitflags! {
    pub struct PPUMASK: u8 {
        const GREYSCALE                      = 0b00000001;
        const BACKGROUD_LEFT_ENABLED         = 0b00000010;
        const SPRITE_LEFT_ENABLED            = 0b00000100;
        const BACKGROUND_ENABLED             = 0b00001000;
        const SPRITES_ENABLED                = 0b00010000;
        const RED_EMPHASIZE                  = 0b00100000;
        const GREEN_EMPHASIZE                = 0b01000000;
        const BLUE_EMPHASIZE                 = 0b10000000;
    }
}

/*
7  bit  0
---- ----
VSO. ....
|||| ||||
|||+-++++- Garbage
||+------- Sprite overflow.
|+-------- Sprite 0 Hit.
+--------- Vertical blank has started (0: not in vblank; 1: in vblank).
*/

bitflags! {
    pub struct PPUSTATUS: u8 {
        const GARBAGE            = 0b00011111;
        const SPRITE_OVERFLOW    = 0b00100000;
        const SPRITE_0_HIT       = 0b01000000;
        const V_BLANK            = 0b10000000;
    }
}

impl<'a> Nes<'a> {
    pub(crate) fn read_cpu_byte(&mut self, addr: u16) -> u8 {
        return match addr {
            0x0000..=0x1FFF => {
                //Ram is mirrored in this space
                let ram_addr = (addr % 0x800) as usize;
                self.cpu_memory[ram_addr]
            }
            0x2000..=0x3FFF => {
                //This area is a mirror for 0x2000 - 0x2007
                let ppu_io_addr = (addr - 0x2000) % 0x8;
                println!("PPU read {:x}", ppu_io_addr + 0x2000);
                return match ppu_io_addr {
                    //todo should panic?
                    0 | 1 | 3 | 5 | 6 => 0,
                    2 => self.ppustatus.bits(),
                    4 => {
                        let oam_addr = self.oam_addr;
                        self.oam_ram[oam_addr as usize]
                    }
                    7 => {
                        let vram_addr = self.vram_addr;
                        let value = self.read_ppu_byte(vram_addr);
                        //Increase vram_addr based on VRAM_INCREMENT bit
                        let horizontal_increment = self.ppuctrl.contains(PPUCTRL::VRAM_INCREMENT);
                        self.vram_addr = vram_addr + if horizontal_increment { 1 } else { 32 };
                        value
                    }
                    _ => 0,
                };
            }
            //APU I/O
            0x4000..=0x401F => 0,
            //Expansion ROM (only certain mappers
            0x4020..=0x5FFF => 0,
            //Cart RAM
            0x6000..=0x7FFF => 0,
            0x8000..=0xFFFF => {
                //TODO handle cart mapper
                let cartridge = self.cartridge.unwrap();
                let pkg_rom = &cartridge.pkg_rom;
                let pkg_rom_size = cartridge.pkg_rom_size;

                //If ROM only has 1 page, it's mirrored into 0xC000 - 0xFFFF
                let rom_addr = (addr - 0x8000)
                    % if pkg_rom_size == 0x4000 {
                        0x4000
                    } else {
                        0x8000
                    };
                *pkg_rom.get(rom_addr as usize).unwrap()
            }
        };
    }

    pub(crate) fn write_cpu_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                //Ram is mirrored in this space
                let ram_addr = (addr % 0x800) as usize;
                self.cpu_memory[ram_addr] = value;
            }
            0x2000..=0x3FFF => {
                //This area is a mirror for 0x2000 - 0x2007
                let ppu_io_addr = (addr - 0x2000) % 0x8;
                println!("PPU write {:x}", ppu_io_addr + 0x2000);
                return match ppu_io_addr {
                    0 => {
                        self.ppuctrl = PPUCTRL::from_bits_truncate(value);
                    }
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
                    5 => {
                        let second_write = self.ppu_second_write;
                        if second_write {
                            self.vertical_scroll_origin = value
                        } else {
                            self.horizontal_scroll_origin = value
                        }
                        self.ppu_second_write = !second_write
                    }
                    6 => {
                        let second_write = self.ppu_second_write;
                        let addr = self.vram_addr;
                        let new_addr = if !second_write {
                            (value as u16 & 0x3F) << 8
                        } else {
                            addr | u16::from(value)
                        };

                        self.vram_addr = new_addr;
                        self.ppu_second_write = !second_write
                    }
                    7 => {
                        let vram_addr = self.vram_addr;
                        self.write_ppu_byte(vram_addr, value);
                        //Increase vram_addr based on VRAM_INCREMENT bit
                        let horizontal_increment = self.ppuctrl.contains(PPUCTRL::VRAM_INCREMENT);
                        self.vram_addr = vram_addr + if horizontal_increment { 1 } else { 32 };
                    }
                    2 => {
                        //NOP
                    }
                    _ => {}
                };
            }
            //APU I/O
            0x4000..=0x4013 | 0x4015..=0x401F => {}
            0x4014 => {
                self.request_dma = true;
                self.dma_src = u16::from(value) << 8;
            }
            //Expansion ROM (only certain mappers
            0x4020..=0x5FFF => {}
            //Cart RAM
            0x6000..=0x7FFF => {}
            0x8000..=0xFFFF => {
                //TODO this should be different for each cart
            }
        }
    }

    pub(crate) fn read_ppu_byte(&mut self, addr: u16) -> u8 {
        0
    }

    pub(crate) fn write_ppu_byte(&mut self, addr: u16, value: u8) {}

    pub(crate) fn raise_nmi(&mut self) {
        //TODO handle better for clock accuracy
        self.raise_interrupt(Interrupt::NMI);
    }
}