pub trait Memory {
    fn read_byte(&mut self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8);

    fn read_word(&mut self, pos: u16) -> u16 {
        let lo = self.read_byte(pos) as u16;
        let hi = self.read_byte(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn write_word(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.write_byte(pos, lo);
        self.write_byte(pos + 1, hi);
    }
}
