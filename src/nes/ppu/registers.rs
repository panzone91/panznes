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

pub const NAMETABLE_ADDRESS: u8 = 0b00000011;
pub const VRAM_INCREMENT: u8 = 0b00000100;
pub const SPRITE_PATTERN_TABLE: u8 = 0b00001000;
pub const BACKGROUND_PATTERN_TABLE: u8 = 0b00010000;
pub const SPRITE_SIZE_16: u8 = 0b00100000;
pub const PPU_MASTER_SLAVE: u8 = 0b01000000;
pub const NMI_ENABLED: u8 = 0b10000000;

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
pub const GREYSCALE: u8 = 0b00000001;
pub const BACKGROUD_LEFT_ENABLED: u8 = 0b00000010;
pub const SPRITE_LEFT_ENABLED: u8 = 0b00000100;
pub const BACKGROUND_ENABLED: u8 = 0b00001000;
pub const SPRITES_ENABLED: u8 = 0b00010000;
pub const RED_EMPHASIZE: u8 = 0b00100000;
pub const GREEN_EMPHASIZE: u8 = 0b01000000;
pub const BLUE_EMPHASIZE: u8 = 0b10000000;

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
pub const GARBAGE: u8 = 0b00011111;
pub const SPRITE_OVERFLOW: u8 = 0b00100000;
pub const SPRITE_0_HIT: u8 = 0b01000000;
pub const V_BLANK: u8 = 0b10000000;
