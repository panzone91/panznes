use crate::nes::ppu::palette::NES_PALETTE;
use crate::nes::system_bus::{PPUCTRL, PPUMASK, PPUSTATUS};
use crate::nes::Nes;

mod palette;

impl<'a> Nes<'a> {
    fn write_background(&mut self, current_scanline: u32) {
        let tile_index_row = current_scanline >> 0x3;

        for i in 0..31 {
            let current_index_nametable = ((tile_index_row * 32) + i) as u16;
            //TODO the starting point depends on scrolling registers
            let current_tile_index = self.read_ppu_byte((0x2000 + current_index_nametable) as u16);
            //Each tile is 16 bytes long, so the address of the tile is base + (index *16)
            //The base is 0x0000 or 0x1000 based on PPUCTRL
            let pattern_table: u16 = if self.ppuctrl.contains(PPUCTRL::BACKGROUND_PATTERN_TABLE) {
                0x1000
            } else {
                0x0000
            };
            let tile_address = pattern_table + (u16::from(current_tile_index) * 16);

            //A tile is 8x8 pixels and each pixel is represented by 2 bit, so each row is 2 bytes
            let current_tile_row = (current_scanline & 0x3) as u16;
            let tile_row_address = (tile_address + (current_tile_row * 2)) as u16;

            let row_word: u16 = (u16::from(self.read_ppu_byte(tile_row_address)) << 8)
                | u16::from(self.read_ppu_byte(tile_row_address + 1));

            //I need to compute the attribute table for select the palette
            //TODO attribute table
            let attribute_table = 0x2000 + 0x3C0 as u16;

            //The attribute table divides the 960 tiles of nametable into 8x8 blocks of
            //4x4 tiles (each block contains 16 tiles). We can index the byte this way:
            //
            // 111 111 -> the 3 MSBs are the 3 MSB of the tile number,
            // the 3 LSBs are bit 2, 3 and 4 of the address

            let attribute_table_index =
                (current_index_nametable >> 7) << 3 | ((current_index_nametable & 0x1F) >> 2);

            let attribute_table_byte = self.read_ppu_byte(attribute_table + attribute_table_index);

            //Each byte in the attribute table is the 2 MSB for the palette for a 4x4 block inside
            //the 8x8 block

            let internal_group_index =
                ((current_index_nametable & 0x40) >> 5) | ((current_index_nametable & 0x2) >> 1);

            let msb = ((attribute_table_byte & 0x3) << (internal_group_index * 2))
                >> (internal_group_index * 2);

            for k in 0..=7 {
                let b = (row_word >> (14 - k * 2)) & 0x3;
                let q = (msb << 2) | (b as u8);

                let palette = self.read_ppu_byte(0x3F00 + u16::from(q)) & 0x3F;

                let color = NES_PALETTE[palette as usize];

                let index_screen = (256 * current_scanline) + (i * 8) + k;
                self.screen[index_screen as usize] = color;
            }

            //I need to use the two bits in row to select the color inside the palette
        }

        let tile_index = current_scanline & 0x3;
    }

    pub fn execute_ppu(&mut self, cpu_cycles: u32) {
        let ppu_cycles = cpu_cycles as i32 * 3;
        let clock_current_scanline = self.clock_current_scanline - ppu_cycles;

        if clock_current_scanline <= 0 {
            //new scanline!
            let next_scanline = self.current_scanline + 1;

            match next_scanline {
                0..=239 => {
                    if self.ppumask.contains(PPUMASK::BACKGROUND_ENABLED) {
                        self.write_background(self.current_scanline);
                    }
                    self.current_scanline += 1
                }
                240 => {
                    //set HBlank, check if NMI is active and raise
                    let mut ppustatus = self.ppustatus;
                    ppustatus.insert(PPUSTATUS::V_BLANK);
                    self.ppustatus = ppustatus;

                    let ppuctrl = self.ppuctrl;
                    if ppuctrl.contains(PPUCTRL::NMI_ENABLED) {
                        self.raise_nmi();
                    }
                    self.current_scanline += 1;
                }
                //VBlank = do nothing
                241..=260 => {
                    self.current_scanline += 1;
                }
                //Finished scanlines, reset
                261 => {
                    self.current_scanline = 0;
                }
                _ => {
                    //TODO panic
                    panic!("Bad scanline")
                }
            }
            self.clock_current_scanline += 341;
        }
    }

    pub(crate) fn read_ppu_byte(&mut self, addr: u16) -> u8 {
        return match addr {
            //CHR_ROM
            //TODO This depends on cart mapping
            0..=0x1FFF => self.cartridge.unwrap().chr_rom[addr as usize],
            //Nametables
            //TODO this depends on cart mirroring!
            0x2000..=0x2FFF => {
                let ppu_addr = addr - 0x2000;
                self.ppu_memory[ppu_addr as usize]
            }
            //Mirror of 0x2000 .. 0x2EFF
            0x3000..=0x3EFF => {
                let ppu_addr = addr - 0x1000;
                self.read_ppu_byte(ppu_addr)
            }
            //Palettes area
            0x3F00..=0x3F1F => {
                let palette_addr = addr & 0x1F;
                self.palettes[palette_addr as usize]
            }
            //Mirror of 3F00 .. 0x3F1F
            0x3F20..=0x3FFF => {
                let ppu_addr = 0x3F00 + (addr & 0x1F);
                self.read_ppu_byte(ppu_addr)
            }
            _ => panic!("PPU bus is 14 bit long"),
        };
    }

    pub(crate) fn write_ppu_byte(&mut self, addr: u16, value: u8) {
        //PPU bus is 14 bit long, so every address in 0x4000..0xFFFF is mapped to 0x0000..0x3FFF
        let write_addr = addr & 0x3FFF;
        match write_addr {
            //CHR_ROM. Some carts use this area for bank switching
            //TODO This depends on cart mapping
            0..=0x1FFF => {
                //TODO for now only mapping 0
            }
            //Nametables
            //TODO this depends on cart mirroring!
            0x2000..=0x2FFF => {
                let ppu_addr = addr - 0x2000;
                self.ppu_memory[ppu_addr as usize] = value;
            }
            //Mirror of 0x2000 .. 0x2EFF
            0x3000..=0x3EFF => {
                let ppu_addr = write_addr - 0x1000;
                self.write_ppu_byte(ppu_addr, value);
            }
            //Palettes area
            0x3F00..=0x3F1F => {
                let palette_addr = write_addr & 0x1F;
                self.palettes[palette_addr as usize] = value;
            }
            //Mirror of 3F00 .. 0x3F1F
            0x3F20..=0x3FFF => {
                let ppu_addr = 0x3F00 + (write_addr & 0x1F);
                self.write_ppu_byte(ppu_addr, value);
            }
            //This should never happens...
            _ => panic!("PPU bus is 14 bit long"),
        };
    }
}
