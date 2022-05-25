use crate::nes::ppu::registers::PPUCTRL;
use crate::Nes;

impl<'a> Nes<'a> {
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
}
