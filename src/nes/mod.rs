use crate::nes::system_bus::{PPUCTRL, PPUMASK, PPUSTATUS};
use crate::Cartridge;
use bitflags::bitflags;

mod cpu;
mod ppu;
mod system_bus;

enum Interrupt {
    NMI,
    IRQ,
    RESET,
    BREAK,
}

bitflags! {
    pub(self) struct FlagRegister: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const IRQ_DISABLE       = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const UNUSED            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}

pub struct Nes<'a> {
    // Registers

    //accumulator
    pub(self) a: u8,
    //Index register x
    pub(self) x: u8,
    //Index register y
    pub(self) y: u8,
    //Stack pointer
    pub(self) stack_ptr: u8,
    //P (status)
    pub(self) flag: FlagRegister,
    //Program Counter
    pub(self) prog_counter: u16,

    //Main WRAM
    cpu_memory: [u8; 0x800],
    cartridge: Option<&'a Cartridge>,

    ppuctrl: PPUCTRL,
    ppumask: PPUMASK,
    ppustatus: PPUSTATUS,
    oam_addr: u8,
    oam_ram: [u8; 0x100],

    ppu_second_write: bool,

    horizontal_scroll_origin: u8,
    vertical_scroll_origin: u8,

    vram_addr: u16,
    ppu_memory: [u8; 0x10000],

    request_dma: bool,
    dma_src: u16,

    current_scanline: u32,
    clock_current_scanline: i32,

    raised_nmi: bool,
    palettes: [u8; 0x20],

    pub screen: [u32; 256 * 240],
}

impl<'a> Nes<'a> {
    pub fn create_nes() -> Nes<'a> {
        Nes {
            a: 0,
            x: 0,
            y: 0,
            stack_ptr: 0,
            flag: FlagRegister::from_bits_truncate(0),
            prog_counter: 0,

            //Main WRAM
            cpu_memory: [0x0; 0x800],
            cartridge: Option::None,

            ppuctrl: PPUCTRL::from_bits_truncate(0),
            ppumask: PPUMASK::from_bits_truncate(0),
            ppustatus: PPUSTATUS::from_bits_truncate(0),
            oam_addr: 0,
            oam_ram: [0x0; 0x100],

            ppu_second_write: false,

            horizontal_scroll_origin: 0,
            vertical_scroll_origin: 0,

            vram_addr: 0x0,
            ppu_memory: [0x0; 0x10000],

            request_dma: false,
            dma_src: 0x0,

            current_scanline: 0,
            clock_current_scanline: 0,

            raised_nmi: false,

            palettes: [0x0; 0x20],
            screen: [0x0; 256 * 240],
        }
    }

    pub fn insert_cartdrige(&mut self, cart: &'a Cartridge) {
        self.cartridge = Some(cart);
    }
}
