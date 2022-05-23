use crate::nes::ppu::palette::NES_PALETTE;
use crate::nes::system_bus::{PPUCTRL, PPUMASK, PPUSTATUS};
use crate::nes::Nes;

mod memory;
mod palette;
mod tile;

impl<'a> Nes<'a> {
    fn get_active_pattern_table(&mut self, bit: PPUCTRL) -> u16 {
        if self.ppuctrl.contains(bit) {
            0x1000
        } else {
            0x0000
        }
    }

    fn get_tile_pixel_from_planes(
        tile_first_plane: u8,
        tile_second_plane: u8,
        which_pixel: u8,
    ) -> u8 {
        (tile_first_plane >> (7 - which_pixel) & 0x1)
            | ((tile_second_plane >> (7 - which_pixel) & 0x1) << 1)
    }

    fn get_sprite_size(&self) -> u16 {
        if self.ppuctrl.contains(PPUCTRL::SPRITE_SIZE_16) {
            16
        } else {
            8
        }
    }

    fn write_sprites(&mut self, current_scanline: u16) {
        // PPU has a 32 byte memory that works as a secondary OAM that contains
        // the 8 sprites for this line. We start by doing a binary search

        let mut secondary_oam: [u8; 32] = [0; 32];
        let mut secondary_oam_index = 0;
        //Sprites can be 8x8 or 8x16, based on PPUCTRL
        let sprite_size = self.get_sprite_size();

        for i in 0..=63 {
            // A sprite is composed by 4 bytes:
            // Byte 0 = y position - 1
            // Byte 1 = tile number
            // Byte 2 = Attributes
            // Byte 3 = x position

            let current_sprite_y_index = i * 4;
            let y_pos = (self.oam_ram[current_sprite_y_index] as u16).wrapping_add(1);

            if current_scanline >= y_pos && current_scanline < (y_pos + sprite_size) {
                if secondary_oam_index == 32 {
                    //There are more than 8 sprites on this line -> sprite overflow
                    self.ppustatus.insert(PPUSTATUS::SPRITE_OVERFLOW);
                } else {
                    secondary_oam[secondary_oam_index..=secondary_oam_index + 3].copy_from_slice(
                        &self.oam_ram[current_sprite_y_index..=current_sprite_y_index + 3],
                    );
                    secondary_oam_index += 4;
                }
            }
        }
        let number_sprites_scanline = secondary_oam_index / 4;
        //TODO handle sprite 0 hit
        //TODO handle sprite priority correctly
        for i in 0..number_sprites_scanline {
            let sprite_index = i * 4;
            let sprite_y_position = secondary_oam[sprite_index];
            let sprite_tile_index = secondary_oam[sprite_index + 1];
            let sprite_attributes = secondary_oam[sprite_index + 2];
            let sprite_x_position = secondary_oam[sprite_index + 3];

            let pattern_table: u16 =
            // If sprite_size is 8, the pattern table depends of PPUCTRL bit
            // If sprite_size if 16, the LSB of the index indicates the table
                if sprite_size == 8 {
                    self.get_active_pattern_table(PPUCTRL::SPRITE_PATTERN_TABLE)
                } else {
                    (u16::from(sprite_tile_index) & 0x1) * 0x1000
                };

            let tile_address = pattern_table.wrapping_add(u16::from(sprite_tile_index) << 4);

            //TODO NOT TRUE! Depends if I must draw the sprite flipped
            let current_tile_row = current_scanline
                .wrapping_sub(u16::from(sprite_y_position))
                .wrapping_sub(1);

            let tile_row_address = tile_address.wrapping_add(current_tile_row);
            let tile_first_plane = self.read_ppu_byte(tile_row_address);
            let tile_second_plane = self.read_ppu_byte(tile_row_address.wrapping_add(8));

            let palette_msb = sprite_attributes & 0x3;

            for current_pixel in 0..=7 {
                //To handle pixel mirroring
                let pixel_to_render = if sprite_attributes & 0x40 == 0 {
                    current_pixel
                } else {
                    7 - current_pixel
                };

                let palette_lsb = Nes::get_tile_pixel_from_planes(
                    tile_first_plane,
                    tile_second_plane,
                    pixel_to_render,
                );

                if palette_lsb == 0x0 {
                    //Transparent pixel -> nothing to do here
                    continue;
                }

                let palette_index = (palette_msb << 2) | (palette_lsb as u8);

                // Nes palettes are 6 bit and the PPU only uses 6 bits to retrieve the value from
                // the system palette
                let palette_for_pixel =
                    self.read_ppu_byte(0x3F10 + u16::from(palette_index)) & 0x3F;

                let rgb_color = NES_PALETTE[palette_for_pixel as usize];

                let x_pos = u16::from(sprite_x_position).wrapping_add(u16::from(current_pixel));

                if x_pos < 256 {
                    let index_screen = (256 * current_scanline) + x_pos;
                    self.screen[index_screen as usize] = rgb_color;
                }
            }
        }
    }

    pub fn execute_ppu(&mut self, cpu_cycles: u32) {
        let ppu_cycles = cpu_cycles * 3;
        let clock_current_scanline = self.clock_current_scanline.wrapping_add(ppu_cycles);

        self.clock_current_scanline = clock_current_scanline;
        if clock_current_scanline >= 341 {
            //new scanline!
            match self.current_scanline {
                0..=239 => {
                    //TODO maybe I should emulate the PPU clock by clock?
                    if self.ppumask.contains(PPUMASK::BACKGROUND_ENABLED) {
                        self.render_tiles(self.current_scanline as u16);
                        //self.write_background(self.current_scanline as u16);
                    }
                    if self.ppumask.contains(PPUMASK::SPRITES_ENABLED) {
                        self.write_sprites(self.current_scanline as u16);
                    }
                    self.current_scanline += 1;
                    //self.ppustatus.insert(PPUSTATUS::SPRITE_0_HIT);
                }
                240 => {
                    //set HBlank, check if NMI is active and raise
                    let mut ppustatus = self.ppustatus;
                    ppustatus.insert(PPUSTATUS::V_BLANK);
                    ppustatus.remove(PPUSTATUS::SPRITE_0_HIT);
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
            self.clock_current_scanline -= 341;
        }
    }
}
