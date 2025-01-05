use crate::nes::ppu::registers::{BACKGROUND_ENABLED, SPRITES_ENABLED, SPRITE_0_HIT, V_BLANK};
use crate::nes::Nes;

mod background_renderer;
mod memory;
mod palette;
pub(crate) mod registers;
mod sprite_renderer;
mod utilities;

impl Nes {
    pub fn execute_ppu(&mut self, cpu_cycles: u32) {
        //A CPU tick is equal to 3 PPU ticks...
        let ppu_cycles = cpu_cycles * 3;
        let clock_current_scanline = self.clock_current_scanline.wrapping_add(ppu_cycles);

        self.clock_current_scanline = clock_current_scanline;
        if clock_current_scanline >= 341 {
            match self.current_scanline {
                0..=239 => {
                    //TODO maybe I should emulate the PPU clock by clock?
                    if (self.ppumask & BACKGROUND_ENABLED) != 0 {
                        self.render_background(self.current_scanline as u16);
                    }
                    if (self.ppumask & SPRITES_ENABLED) != 0 {
                        self.render_sprites(self.current_scanline as u16);
                    }
                    self.current_scanline += 1;
                }
                240 => {
                    //set VBlank, check if NMI is active and raise
                    let mut ppustatus = self.ppustatus;
                    ppustatus = ppustatus | V_BLANK;
                    ppustatus = ppustatus & !SPRITE_0_HIT;
                    self.ppustatus = ppustatus;

                    self.current_scanline += 1;
                }
                //VBlank = do nothing
                241..=260 => {
                    self.current_scanline += 1;
                }
                //Finished scanlines, reset
                261 => {
                    self.current_scanline = 0;
                    self.ppustatus = self.ppustatus & !V_BLANK;
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
