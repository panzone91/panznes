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