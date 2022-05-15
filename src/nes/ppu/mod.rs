use crate::nes::Nes;
use crate::nes::system_bus::{PPUCTRL, PPUSTATUS};

impl<'a> Nes<'a> {
    pub fn execute_ppu(&mut self, cpu_cycles: u32) {
        let ppu_cycles = cpu_cycles as i32 * 3;
        let clock_current_scanline = self.clock_current_scanline - ppu_cycles;

        if clock_current_scanline <= 0 {
            //new scanline!
            let next_scanline = self.current_scanline + 1;

            match next_scanline {
                242 => {
                    //set HBlank, check if NMI is active and raise
                    let mut ppustatus = self.ppustatus;
                    ppustatus.insert(PPUSTATUS::V_BLANK);
                    self.ppustatus = ppustatus;

                    let ppuctrl = self.ppuctrl;
                    if ppuctrl.contains(PPUCTRL::NMI_ENABLED) {
                        self.raise_nmi();
                    }
                    self.clock_current_scanline += 341;
                    self.current_scanline += 1;
                }
                //Finished scanlines, reset
                261 => {
                    self.clock_current_scanline += 341;
                    self.current_scanline = 0;
                }
                _ => {
                    //TODO draw scanline
                    self.clock_current_scanline += 341;
                    self.current_scanline += 1;
                }
            }
        }
    }
}
