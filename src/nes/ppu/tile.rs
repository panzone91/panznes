use crate::nes::ppu::palette::NES_PALETTE;
use crate::nes::system_bus::PPUCTRL;
use crate::Nes;
use std::ops::Mul;

impl<'a> Nes<'a> {
    fn get_main_nametable(&mut self) -> u16 {
        return match self.ppuctrl.bits() & 0x3 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => panic!("Error nametable"),
        };
    }

    fn get_tile_pixels(palette_msb: u8, tile_first_plane: u8, tile_second_plane: u8) -> [u8; 8] {
        let mut pixels = [0x0 as u8; 8];

        for i in 0..=7 {
            let palette_lsb =
                (tile_first_plane >> (7 - i) & 0x1) | ((tile_second_plane >> (7 - i) & 0x1) << 1);
            pixels[i] = (palette_msb << 2) | (palette_lsb as u8);
        }
        return pixels;
    }

    pub fn render_tiles(&mut self, current_scanline: u16) {
        // First, let's retrieve the tile row we need to render
        // Since we have scrolling, it's possible that the current line
        // Is on the bottom nametable (mod 0x2000)
        let line = current_scanline.wrapping_add(u16::from(self.vertical_scroll_origin));
        let nametable = (self.get_main_nametable() + if line >= 240 { 0x800 } else { 0 }) & 0x2FFF;
        let row_in_nametable = line % 240; //line mod 240

        let tile_index_start_row = (row_in_nametable / 8) * 32;

        let tile_x_scroll = u16::from(self.horizontal_scroll_origin) >> 3;
        let starting_addr = nametable
            .wrapping_add(tile_index_start_row)
            .wrapping_add(tile_x_scroll);
        let mut current_pixel = 0;

        //I need to retrieve 33 tiles for this row. A row is 32 tiles,
        //however it's possible to see 33 tiles because of scrolling
        for row_tile in 0..=32 {
            let offset: u16 = if row_tile > (31 - tile_x_scroll) {
                0x400
            } else {
                0x000
            };
            let tile_address = starting_addr
                .wrapping_add(offset)
                .wrapping_add(row_tile as u16)
                & 0x2FFF;
            let nametable_index = tile_address & 0x3FF;

            let attribute_table = (tile_address & 0xFC00) + 0x3C0;

            let attribute_table_index =
                (nametable_index >> 7) << 3 | ((nametable_index & 0x1F) >> 2);

            let tile_index = self.read_ppu_byte(tile_address);
            let pattern_table: u16 =
                self.get_active_pattern_table(PPUCTRL::BACKGROUND_PATTERN_TABLE);

            let tile_offset_y =
                (current_scanline.wrapping_add(self.vertical_scroll_origin as u16)) & 0x7;

            let tile_address = pattern_table
                .wrapping_add(u16::from(tile_index) << 4)
                .wrapping_add(u16::from(tile_offset_y));

            let tile_first_plane = self.read_ppu_byte(tile_address);
            let tile_second_plane = self.read_ppu_byte(tile_address.wrapping_add(8));

            let attribute_table_entry =
                self.read_ppu_byte(attribute_table.wrapping_add(attribute_table_index));
            let internal_group_index =
                ((nametable_index & 0x40) >> 5) | ((nametable_index & 0x2) >> 1);

            let palette_msb = (attribute_table_entry >> (internal_group_index * 2)) & 0x3;

            //The attribute table contains the MSB of the palette to use for this group
            //let palette_msb = ((attribute_table_entry & 0x3) << (internal_group_index * 2))
            //   >> (internal_group_index * 2);

            let pixels = Nes::get_tile_pixels(palette_msb, tile_first_plane, tile_second_plane);

            let tile_x_offset = self.horizontal_scroll_origin & 0x7;
            match row_tile {
                //First tile: I only need the pixels from tile_x_offset
                0 => {
                    for i in tile_x_offset..=7 {
                        self.render_pixel(
                            pixels[i as usize],
                            current_pixel,
                            current_scanline as u8,
                        );
                        current_pixel = current_pixel.wrapping_add(1);
                    }
                }
                1..=31 => {
                    for i in 0..=7 {
                        self.render_pixel(pixels[i], current_pixel, current_scanline as u8);
                        current_pixel = current_pixel.wrapping_add(1);
                    }
                }
                //Last tile: I only need to render the tile_x_offset bits
                32 => {
                    if (tile_x_offset > 0) {
                        for i in 0..=tile_x_offset - 1 {
                            self.render_pixel(
                                pixels[i as usize],
                                current_pixel,
                                current_scanline as u8,
                            );
                            current_pixel = current_pixel.wrapping_add(1);
                        }
                    }
                }
                //This should never happens...
                _ => {
                    panic!("Too many tiles in this row")
                }
            }
        }
    }

    fn render_pixel(&mut self, palette: u8, x: u8, y: u8) {
        //If using palette 0 the pixel is transparent -> use default color
        let palette_address = if palette & 0x3 == 0 {
            0x3F00
        } else {
            0x3F00 + u16::from(palette)
        };

        // Nes palettes are 6 bit and the PPU only uses 6 bits to retrieve the value from
        // the system palette
        let palette_for_pixel = self.read_ppu_byte(palette_address) & 0x3F;

        let rgb_color = NES_PALETTE[palette_for_pixel as usize];

        let index_screen = u16::from(y).mul(256).wrapping_add(u16::from(x));
        self.screen[index_screen as usize] = rgb_color;
    }

    pub(super) fn write_background(&mut self, current_scanline: u16) {
        //Let's find the first tile of the screen
        //Each tile is 8x8 and a row has 32 tiles, so the current top left tile of the screen
        //starts at (ppu_scroll_x / 8 + (ppu_scroll_y/8) * 32)

        let tile_offset = u16::from(
            (self.horizontal_scroll_origin >> 3) + ((self.vertical_scroll_origin >> 3) << 5),
        );

        //Now, we need to render the current_scanline of this screen.
        let nametable_row = (current_scanline >> 0x3) << 5;
        //Which of the 8 rows in the tile we are rendering?
        let current_tile_row = current_scanline & 0x7;

        let pattern_table: u16 = self.get_active_pattern_table(PPUCTRL::BACKGROUND_PATTERN_TABLE);

        for i in 0..=31 {
            let nametable_index = tile_offset.wrapping_add(nametable_row).wrapping_add(i);
            //TODO the starting point depends on scrolling registers
            let nametable: u16 = self.get_main_nametable();
            let tile_index = self.read_ppu_byte(nametable.wrapping_add(nametable_index));
            //Each tile is 16 bytes long, so the address of the tile is base + (index *16)
            let tile_address = pattern_table.wrapping_add(u16::from(tile_index) << 4);

            // Each pixel in a tile is 2 bit, however these are not consecutive bits. The 16 bytes
            // of the tiles are divided by 2 8-byte "planes". Each byte in a plane represent one of
            // the 8 rows of the tile and the 2 bits are generated by taking a bit from each plane
            let tile_row_address = tile_address.wrapping_add(current_tile_row);
            let tile_first_plane = self.read_ppu_byte(tile_row_address);
            let tile_second_plane = self.read_ppu_byte(tile_row_address.wrapping_add(8));

            //TODO conmpute attribute table based on nametable address
            let attribute_table = nametable + 0x3C0 as u16;

            //The attribute table divides the 960 tiles of nametable into 8x8 blocks of
            //4x4 tiles (each block contains 16 tiles). We can get the attribute table entry using
            // the nametable index:
            //
            // 111 111 -> the 3 MSBs are the 3 MSB of the tile index,
            // while the 3 LSBs are bit 2, 3 and 4 of the index

            let attribute_table_index =
                (nametable_index >> 7) << 3 | ((nametable_index & 0x1F) >> 2);

            let attribute_table_entry = self.read_ppu_byte(attribute_table + attribute_table_index);

            //Each byte in the attribute table is the 2 MSB for the palette for a 4x4 block inside
            //the 8x8 block
            let internal_group_index =
                ((nametable_index & 0x40) >> 5) | ((nametable_index & 0x2) >> 1);

            //The attribute table contains the MSB of the palette to use for this group
            let palette_msb = ((attribute_table_entry & 0x3) << (internal_group_index * 2))
                >> (internal_group_index * 2);

            for current_pixel in 0..=7 {
                let palette_lsb = Nes::get_tile_pixel_from_planes(
                    tile_first_plane,
                    tile_second_plane,
                    current_pixel,
                );

                let palette_index = (palette_msb << 2) | (palette_lsb as u8);

                //If using palette 0 the pixel is transparent -> use default color
                let palette_address = if palette_lsb == 0 {
                    0x3F00
                } else {
                    0x3F00 + u16::from(palette_index)
                };

                // Nes palettes are 6 bit and the PPU only uses 6 bits to retrieve the value from
                // the system palette
                let palette_for_pixel = self.read_ppu_byte(palette_address) & 0x3F;

                let rgb_color = NES_PALETTE[palette_for_pixel as usize];

                let index_screen = (256 * current_scanline) + (i * 8) + current_pixel as u16;
                self.screen[index_screen as usize] = rgb_color;
            }
        }
    }
}
