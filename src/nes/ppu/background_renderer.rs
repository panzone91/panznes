use crate::Nes;

impl<'a> Nes<'a> {
    pub(super) fn render_background(&mut self, current_scanline: u16) {
        let mut current_pixel = 0;

        for row_tile in 0..=32 {
            let tile_address = 0x2000 | (self.ppu_v & 0x0FFF);
            let fine_y = (self.ppu_v >> 12) & 0x7;
            let fine_x = self.ppu_x;

            let tile_index = self.read_ppu_byte(tile_address);
            let (tile_first_plane, tile_second_plane) =
                self.retrieve_tile_row(tile_index, fine_y as u8);

            let palette_msb = self.retrieve_attribute_table_value(tile_address);

            let pixels = Nes::get_tile_row_pixels(palette_msb, tile_first_plane, tile_second_plane);

            let pixels_to_draw = match row_tile {
                //First tile: I only need the pixels from tile_x_offset
                0 => fine_x..=7,
                //Middle tiles: I have to render the entire row
                1..=31 => 0..=7,
                //Last tile: I only need to render the tile_x_offset bits
                32 => {
                    if fine_x > 0 {
                        0..=fine_x - 1
                    } else {
                        1..=0
                    }
                }
                //This should never happens...
                _ => {
                    panic!("Too many tiles in this row")
                }
            };

            for i in pixels_to_draw {
                //If using palette 0 the pixel is transparent -> use default color
                //TODO maybe this part should be done by
                let palette_address = if pixels[i as usize] & 0x3 == 0 {
                    0x3F00
                } else {
                    0x3F00 + u16::from(pixels[i as usize])
                };
                self.render_pixel(palette_address, current_pixel, current_scanline as u8);
                self.background_hit_flag
                    [current_pixel as usize + (current_scanline as usize * 256)] =
                    palette_address != 0x3F00;
                current_pixel = current_pixel.wrapping_add(1);
            }

            if (self.ppu_v & 0x001F) == 31 {
                self.ppu_v &= !0x001F;
                self.ppu_v ^= 0x0400;
            } else {
                self.ppu_v += 1
            }
        }
        //Increment

        if (self.ppu_v & 0x7000) != 0x7000 {
            self.ppu_v += 0x1000;
        } else {
            self.ppu_v &= !0x7000;
            let mut y = (self.ppu_v & 0x03E0) >> 5;
            if y == 29 {
                y = 0;
                self.ppu_v ^= 0x0800
            } else if y == 31 {
                y = 0;
            } else {
                y += 1;
            }
            self.ppu_v = (self.ppu_v & !0x03E0) | (y << 5)
        }

        self.ppu_v = (self.ppu_v & 0xFBE0) | (u16::from(self.ppu_t & 0x041F));
    }

    fn retrieve_attribute_table_value(&mut self, nametable_tile_address: u16) -> u8 {
        let nametable_index = nametable_tile_address & 0x3FF;

        let attribute_table_address = (nametable_tile_address & 0xFC00) + 0x3C0;

        let attribute_table_index = (nametable_index >> 7) << 3 | ((nametable_index & 0x1F) >> 2);

        let attribute_table_entry =
            self.read_ppu_byte(attribute_table_address.wrapping_add(attribute_table_index));
        let internal_group_index = ((nametable_index & 0x40) >> 5) | ((nametable_index & 0x2) >> 1);

        return (attribute_table_entry >> (internal_group_index * 2)) & 0x3;
    }

    fn get_tile_row_pixels(
        palette_msb: u8,
        tile_first_plane: u8,
        tile_second_plane: u8,
    ) -> [u8; 8] {
        let mut pixels = [0x0 as u8; 8];

        for i in 0..=7 {
            let palette_lsb =
                (tile_first_plane >> (7 - i) & 0x1) | ((tile_second_plane >> (7 - i) & 0x1) << 1);
            pixels[i] = (palette_msb << 2) | (palette_lsb as u8);
        }
        return pixels;
    }
}
