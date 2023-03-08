use crate::nes::ppu::palette::NES_PALETTE;
use crate::nes::ppu::registers::PPUCTRL;
use crate::Nes;
use std::ops::Mul;

impl Nes {
    pub(super) fn get_active_pattern_table(&mut self, bit: PPUCTRL) -> u16 {
        if self.ppuctrl.contains(bit) {
            0x1000
        } else {
            0x0000
        }
    }

    pub(super) fn retrieve_tile_row(&mut self, tile_index: u8, y: u8) -> (u8, u8) {
        let pattern_table: u16 = self.get_active_pattern_table(PPUCTRL::BACKGROUND_PATTERN_TABLE);

        let tile_address = pattern_table
            //A tile is 16 bytes, so the address of the tile is pattern_table + (tile_index * 16)
            .wrapping_add(u16::from(tile_index) << 4)
            //Each byte represents a row
            .wrapping_add(u16::from(y));

        //Each tile row is composed by 2 bytes
        let tile_first_plane = self.read_ppu_byte(tile_address);
        let tile_second_plane = self.read_ppu_byte(tile_address.wrapping_add(8));
        return (tile_first_plane, tile_second_plane);
    }

    pub(super) fn render_pixel(&mut self, palette_address: u16, x: u8, y: u8) {
        // Nes palettes are 6 bit and the PPU only uses 6 bits to retrieve the value from
        // the system palette
        let palette_for_pixel = self.read_ppu_byte(palette_address) & 0x3F;

        let rgb_color = NES_PALETTE[palette_for_pixel as usize];

        //Nes screen is 256x240
        let index_screen = u16::from(y).mul(256).wrapping_add(u16::from(x));
        self.screen[index_screen as usize] = rgb_color;
    }
}
