use crate::memory::Memory;
use crate::nes::{FlagRegister, Interrupt, Nes};

pub enum AddressMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

pub struct Instruction {
    pub opcode: u8,
    pub address_mode: AddressMode,
    pub cycles: u32,
}

pub const OPCODES: [Instruction; 256] = [
    Instruction {
        opcode: 0x00,
        cycles: 7,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x01,
        cycles: 6,
        address_mode: AddressMode::IndirectX,
    },
    Instruction {
        opcode: 0x02,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x03,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x04,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x05,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x06,
        cycles: 5,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x07,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x08,
        cycles: 3,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x09,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x0A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x0B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x0C,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x0D,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x0E,
        cycles: 6,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x0F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x10,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x11,
        cycles: 5,
        address_mode: AddressMode::IndirectY,
    },
    Instruction {
        opcode: 0x12,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x13,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x14,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x15,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x16,
        cycles: 6,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x17,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x18,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x19,
        cycles: 4,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0x1A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x1B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x1C,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x1D,
        cycles: 4,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x1E,
        cycles: 7,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x1F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x20,
        cycles: 6,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x21,
        cycles: 6,
        address_mode: AddressMode::IndirectX,
    },
    Instruction {
        opcode: 0x22,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x23,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x24,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x25,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x26,
        cycles: 5,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x27,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x28,
        cycles: 4,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x29,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x2A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x2B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x2C,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x2D,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x2E,
        cycles: 6,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x2F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x30,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x31,
        cycles: 5,
        address_mode: AddressMode::IndirectY,
    },
    Instruction {
        opcode: 0x32,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x33,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x34,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x35,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x36,
        cycles: 6,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x37,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x38,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x39,
        cycles: 4,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0x3A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x3B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x3C,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x3D,
        cycles: 4,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x3E,
        cycles: 7,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x3F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x40,
        cycles: 6,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x41,
        cycles: 6,
        address_mode: AddressMode::IndirectX,
    },
    Instruction {
        opcode: 0x42,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x43,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x44,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x45,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x46,
        cycles: 5,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x47,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x48,
        cycles: 3,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x49,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x4A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x4B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x4C,
        cycles: 3,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x4D,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x4E,
        cycles: 6,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x4F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x50,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x51,
        cycles: 5,
        address_mode: AddressMode::IndirectY,
    },
    Instruction {
        opcode: 0x52,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x53,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x54,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x55,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x56,
        cycles: 6,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x57,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x58,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x59,
        cycles: 4,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0x5A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x5B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x5C,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x5D,
        cycles: 4,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x5E,
        cycles: 7,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x5F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x60,
        cycles: 6,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x61,
        cycles: 6,
        address_mode: AddressMode::IndirectX,
    },
    Instruction {
        opcode: 0x62,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x63,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x64,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x65,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x66,
        cycles: 5,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x67,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x68,
        cycles: 4,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x69,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x6A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x6B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x6C,
        cycles: 5,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x6D,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x6E,
        cycles: 6,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x6F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x70,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x71,
        cycles: 5,
        address_mode: AddressMode::IndirectY,
    },
    Instruction {
        opcode: 0x72,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x73,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x74,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x75,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x76,
        cycles: 6,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x77,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x78,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x79,
        cycles: 4,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0x7A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x7B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x7C,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x7D,
        cycles: 4,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x7E,
        cycles: 7,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x7F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x80,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x81,
        cycles: 6,
        address_mode: AddressMode::IndirectX,
    },
    Instruction {
        opcode: 0x82,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x83,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x84,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x85,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x86,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0x87,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x88,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x89,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x8A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x8B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x8C,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x8D,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x8E,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0x8F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x90,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0x91,
        cycles: 6,
        address_mode: AddressMode::IndirectY,
    },
    Instruction {
        opcode: 0x92,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x93,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x94,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x95,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0x96,
        cycles: 4,
        address_mode: AddressMode::ZeroPageY,
    },
    Instruction {
        opcode: 0x97,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x98,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x99,
        cycles: 6,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0x9A,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x9B,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x9C,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x9D,
        cycles: 5,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0x9E,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0x9F,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xA0,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xA1,
        cycles: 6,
        address_mode: AddressMode::IndirectX,
    },
    Instruction {
        opcode: 0xA2,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xA3,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xA4,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xA5,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xA6,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xA7,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xA8,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xA9,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xAA,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xAB,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xAC,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xAD,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xAE,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xAF,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xB0,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xB1,
        cycles: 5,
        address_mode: AddressMode::IndirectY,
    },
    Instruction {
        opcode: 0xB2,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xB3,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xB4,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0xB5,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0xB6,
        cycles: 4,
        address_mode: AddressMode::ZeroPageY,
    },
    Instruction {
        opcode: 0xB7,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xB8,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xB9,
        cycles: 4,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0xBA,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xBB,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xBC,
        cycles: 4,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0xBD,
        cycles: 4,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0xBE,
        cycles: 4,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0xBF,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xC0,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xC1,
        cycles: 6,
        address_mode: AddressMode::IndirectX,
    },
    Instruction {
        opcode: 0xC2,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xC3,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xC4,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xC5,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xC6,
        cycles: 5,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xC7,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xC8,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xC9,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xCA,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xCB,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xCC,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xCD,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xCE,
        cycles: 6,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xCF,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xD0,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xD1,
        cycles: 5,
        address_mode: AddressMode::IndirectY,
    },
    Instruction {
        opcode: 0xD2,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xD3,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xD4,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xD5,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0xD6,
        cycles: 6,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0xD7,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xD8,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xD9,
        cycles: 4,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0xDA,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xDB,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xDC,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xDD,
        cycles: 4,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0xDE,
        cycles: 7,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0xDF,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xE0,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xE1,
        cycles: 6,
        address_mode: AddressMode::IndirectX,
    },
    Instruction {
        opcode: 0xE2,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xE3,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xE4,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xE5,
        cycles: 3,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xE6,
        cycles: 5,
        address_mode: AddressMode::ZeroPage,
    },
    Instruction {
        opcode: 0xE7,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xE8,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xE9,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xEA,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xEB,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xEc,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xED,
        cycles: 4,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xEE,
        cycles: 6,
        address_mode: AddressMode::Absolute,
    },
    Instruction {
        opcode: 0xEF,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xF0,
        cycles: 2,
        address_mode: AddressMode::Immediate,
    },
    Instruction {
        opcode: 0xF1,
        cycles: 5,
        address_mode: AddressMode::IndirectY,
    },
    Instruction {
        opcode: 0xF2,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xF3,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xF4,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xF5,
        cycles: 4,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0xF6,
        cycles: 6,
        address_mode: AddressMode::ZeroPageX,
    },
    Instruction {
        opcode: 0xF7,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xF8,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xF9,
        cycles: 4,
        address_mode: AddressMode::AbsoluteY,
    },
    Instruction {
        opcode: 0xFA,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xFB,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xFC,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
    Instruction {
        opcode: 0xFD,
        cycles: 4,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0xFE,
        cycles: 7,
        address_mode: AddressMode::AbsoluteX,
    },
    Instruction {
        opcode: 0xFF,
        cycles: 2,
        address_mode: AddressMode::Implied,
    },
];

impl<'a> Nes<'a> {
    pub(super) fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.flag.insert(FlagRegister::ZERO);
        } else {
            self.flag.remove(FlagRegister::ZERO);
        }

        if result >> 7 == 1 {
            self.flag.insert(FlagRegister::NEGATIV);
        } else {
            self.flag.remove(FlagRegister::NEGATIV);
        }
    }

    pub(super) fn read_instruction_operand_8bit(&mut self) -> u8 {
        let operand = self.read_byte(self.prog_counter);
        self.prog_counter += 1;
        return operand;
    }

    pub(super) fn read_instruction_operand_16bit(&mut self) -> u16 {
        let operand = self.read_word(self.prog_counter);
        self.prog_counter += 2;
        return operand;
    }

    pub(super) fn is_page_cross(addr1: u16, addr2: u16) -> bool {
        return addr1 & 0xFF00 != addr2 & 0xFF00;
    }

    pub(super) fn get_operand_address(&mut self, address_mode: &AddressMode) -> (u16, bool) {
        return match address_mode {
            AddressMode::Immediate => {
                let current_pc = self.prog_counter;
                self.prog_counter += 1;
                (current_pc, false)
            }
            AddressMode::ZeroPage => {
                let addr = self.read_instruction_operand_8bit();
                (addr as u16, false)
            }
            AddressMode::ZeroPageX => {
                let addr = self.read_instruction_operand_8bit();
                (addr.wrapping_add(self.x) as u16, false)
            }
            AddressMode::ZeroPageY => {
                let addr = self.read_instruction_operand_8bit();
                (addr.wrapping_add(self.y) as u16, false)
            }
            AddressMode::Absolute => {
                let addr = self.read_instruction_operand_16bit();
                (addr, false)
            }
            AddressMode::AbsoluteX => {
                let addr = self.read_instruction_operand_16bit();
                let final_address = addr.wrapping_add(self.x as u16);
                (final_address, Nes::is_page_cross(addr, final_address))
            }
            AddressMode::AbsoluteY => {
                let addr = self.read_instruction_operand_16bit();
                let final_address = addr.wrapping_add(self.y as u16);
                (final_address, Nes::is_page_cross(addr, final_address))
            }
            AddressMode::IndirectX => {
                let addr = self.read_instruction_operand_8bit().wrapping_add(self.x);
                let final_address = self.read_word(addr as u16);
                (final_address, false)
            }
            AddressMode::IndirectY => {
                let operand = self.read_instruction_operand_8bit();
                let address = self.read_word(operand as u16);
                let final_address = address.wrapping_add(self.y as u16);
                (final_address, Nes::is_page_cross(address, final_address))
            }
            _ => (0, false), //TODO: Should be a panic?
        };
    }

    pub(super) fn lda(&mut self, opcode: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.a = value;
        self.update_zero_and_negative_flags(self.a);
        return if is_page_cross {
            opcode.cycles + 1
        } else {
            opcode.cycles
        }; //Based on opcode
    }

    pub(super) fn ldx(&mut self, opcode: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.x = value;
        self.update_zero_and_negative_flags(self.x);
        return if is_page_cross {
            opcode.cycles + 1
        } else {
            opcode.cycles
        }; //Based on opcode
    }

    pub(super) fn ldy(&mut self, opcode: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&opcode.address_mode);
        let value = self.read_byte(operand);
        self.y = value;
        self.update_zero_and_negative_flags(self.y);
        return if is_page_cross {
            opcode.cycles + 1
        } else {
            opcode.cycles
        }; //Based on opcode
    }

    pub(super) fn st(&mut self, var: u8, opcode: &Instruction) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&opcode.address_mode);
        self.write_byte(operand, var);
        return if is_page_cross {
            opcode.cycles + 1
        } else {
            opcode.cycles
        }; //Based on opcode
    }

    const STACK_PAGE: u16 = 0x100;

    pub(super) fn push(&mut self, data: u8) -> u32 {
        let stack_address: u16 = Nes::STACK_PAGE.wrapping_add(self.stack_ptr as u16);
        self.write_byte(stack_address, data);
        self.stack_ptr = self.stack_ptr.wrapping_sub(1);
        return 3;
    }

    pub(super) fn pop(&mut self) -> (u8, u32) {
        self.stack_ptr = self.stack_ptr.wrapping_add(1);
        let stack_address: u16 = Nes::STACK_PAGE.wrapping_add(self.stack_ptr as u16);
        let value = self.read_byte(stack_address);
        (value, 4)
    }

    pub(super) fn arithmetic_register_a(
        &mut self,
        arithmetic_op: fn(a: u8, carry: u8, data: u8) -> u16,
        instruction: &Instruction,
    ) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let carry = if self.flag.contains(FlagRegister::CARRY) {
            1
        } else {
            0
        };

        let value = self.read_byte(operand);

        let result_16bit = arithmetic_op(self.a, carry, value);
        let result_8bit = result_16bit as u8;

        //Handle flags
        if result_16bit > 0xff {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        //If the two sign bits from a and value are the same but result is different we have overflow
        if (self.a ^ result_8bit) & (value ^ result_8bit) & 0x80 != 0 {
            self.flag.insert(FlagRegister::OVERFLOW);
        } else {
            self.flag.remove(FlagRegister::OVERFLOW);
        }

        self.update_zero_and_negative_flags(result_8bit);

        //Update result
        self.a = result_8bit;
        return if is_page_cross {
            instruction.cycles + 1
        } else {
            instruction.cycles
        };
    }

    pub(super) fn add(a: u8, carry: u8, data: u8) -> u16 {
        u16::from(a)
            .wrapping_add(u16::from(carry))
            .wrapping_add(u16::from(data))
    }

    pub(super) fn adc(&mut self, instruction: &Instruction) -> u32 {
        return self.arithmetic_register_a(Nes::add, instruction);
    }

    pub(super) fn sub(a: u8, carry: u8, data: u8) -> u16 {
        u16::from(a)
            .wrapping_add(u16::from(carry))
            .wrapping_sub(1)
            .wrapping_sub(u16::from(data))
    }

    pub(super) fn sbc(&mut self, instruction: &Instruction) -> u32 {
        self.arithmetic_register_a(Nes::sub, instruction)
    }

    pub(super) fn logical_register_a(
        &mut self,
        logical_op: fn(a: u8, data: u8) -> u8,
        instruction: &Instruction,
    ) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let result = logical_op(self.a, value);

        //Handle flags
        self.update_zero_and_negative_flags(result);
        //Update result
        self.a = result;
        return if is_page_cross {
            instruction.cycles + 1
        } else {
            instruction.cycles
        };
    }

    pub(super) fn and(a: u8, data: u8) -> u8 {
        a & data
    }

    pub(super) fn xor(a: u8, data: u8) -> u8 {
        a ^ data
    }

    pub(super) fn or(a: u8, data: u8) -> u8 {
        a | data
    }

    pub(super) fn compare(&mut self, instruction: &Instruction, compare_with: u8) -> u32 {
        let (operand, is_page_cross) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let result = compare_with.wrapping_sub(value);
        if compare_with >= value {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }

        self.update_zero_and_negative_flags(result);
        return if is_page_cross {
            instruction.cycles + 1
        } else {
            instruction.cycles
        };
    }

    pub(super) fn bit(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);
        if (self.a & value) == 0 {
            self.flag.insert(FlagRegister::ZERO);
        } else {
            self.flag.remove(FlagRegister::ZERO);
        }

        self.flag.set(FlagRegister::NEGATIV, value & 0b10000000 > 0);
        self.flag
            .set(FlagRegister::OVERFLOW, value & 0b01000000 > 0);
        return instruction.cycles;
    }

    pub(super) fn increase_in_memory(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand).wrapping_add(1);
        self.write_byte(operand, value);
        self.update_zero_and_negative_flags(value);
        return instruction.cycles;
    }

    pub(super) fn decrease_in_memory(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand).wrapping_sub(1);
        self.write_byte(operand, value);
        self.update_zero_and_negative_flags(value);
        return instruction.cycles;
    }

    pub(super) fn shl(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let rot_value = value << 1;
        if (value & 0x80) != 0 {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    pub(super) fn shl_acc(&mut self) -> u32 {
        let rot_value = self.a << 1;
        if (self.a & 0x80) != 0 {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    pub(super) fn shr(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);

        let rot_value = value >> 1;
        if (value & 0x01) != 0 {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    pub(super) fn shr_acc(&mut self) -> u32 {
        let rot_value = self.a >> 1;
        if (self.a & 0x01) != 0 {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    pub(super) fn rol(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) {
            1
        } else {
            0
        };

        let mut rot_value = value << 1;
        if (value & 0x80) != 0 {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        if old_carry == 1 {
            rot_value = rot_value | 0x01
        }

        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    pub(super) fn rol_acc(&mut self) -> u32 {
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) {
            1
        } else {
            0
        };

        let mut rot_value = self.a << 1;
        if (self.a & 0x80) != 0 {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        if old_carry == 1 {
            rot_value = rot_value | 0x01
        }

        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    pub(super) fn ror(&mut self, instruction: &Instruction) -> u32 {
        let (operand, _) = self.get_operand_address(&instruction.address_mode);
        let value = self.read_byte(operand);
        let old_carry: u8 = if self.flag.contains(FlagRegister::CARRY) {
            1
        } else {
            0
        };

        let mut rot_value = value >> 1;
        if (value & 0x01) != 0 {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        if old_carry == 1 {
            rot_value = rot_value | 0x80
        }

        self.write_byte(operand, rot_value);
        self.update_zero_and_negative_flags(rot_value);
        return instruction.cycles;
    }

    pub(super) fn ror_acc(&mut self) -> u32 {
        let old_carry = self.flag.contains(FlagRegister::CARRY);

        let mut rot_value = self.a >> 1;
        if (self.a & 0x01) != 0 {
            self.flag.insert(FlagRegister::CARRY)
        } else {
            self.flag.remove(FlagRegister::CARRY)
        }
        if old_carry {
            rot_value = rot_value | 0x80
        }

        self.a = rot_value;
        self.update_zero_and_negative_flags(self.a);
        return 2;
    }

    pub(super) fn conditional_jump(&mut self, instruction: &Instruction, condition: bool) -> u32 {
        let offset: i8 = self.read_instruction_operand_8bit() as i8;
        if condition == true {
            let jump_addr = (self.prog_counter as i32).wrapping_add(i32::from(offset)) as u16;
            let is_page_cross = Nes::is_page_cross(self.prog_counter, jump_addr);
            self.prog_counter = jump_addr;
            return if is_page_cross {
                instruction.cycles + 2
            } else {
                instruction.cycles + 1
            };
        } else {
            return instruction.cycles;
        }
    }

    pub(in crate::nes) fn raise_interrupt(&mut self, interrupt_type: Interrupt) -> u32 {
        if matches!(interrupt_type, Interrupt::IRQ) && self.flag.contains(FlagRegister::IRQ_DISABLE)
        {
            return 0;
        }

        if matches!(interrupt_type, Interrupt::IRQ) {
            self.reset();
            return 7;
        }

        let break_flag = match interrupt_type {
            Interrupt::IRQ | Interrupt::NMI => 0,
            Interrupt::RESET | Interrupt::BREAK => 1,
        };
        if break_flag != 0 {
            self.flag.insert(FlagRegister::BREAK)
        } else {
            self.flag.remove(FlagRegister::BREAK)
        }

        let ret_pc = self.prog_counter;
        self.push((ret_pc >> 8) as u8);
        self.push((ret_pc & 0xFF) as u8);

        self.push(self.flag.bits);

        self.flag.insert(FlagRegister::IRQ_DISABLE);

        let interrupt_routine = match interrupt_type {
            Interrupt::IRQ | Interrupt::BREAK => 0xFFFE,
            Interrupt::NMI => 0xFFFA,
            Interrupt::RESET => 0xFFFC,
        };
        let interrupt_fn = self.read_word(interrupt_routine);
        self.prog_counter = interrupt_fn;
        7
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stack_ptr = 0xFD;
        self.flag = FlagRegister::from_bits_truncate(0b100100);
        self.prog_counter = self.read_word(0xFFFC);
    }
}
