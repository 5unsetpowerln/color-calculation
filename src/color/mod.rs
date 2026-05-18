use crate::bitstream::{BitRead, BitWrite};

pub mod operator;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// 色距離
    pub fn distance(&self, other: &Self) -> u32 {
        let dr = self.r as i32 - other.r as i32;
        let dg = self.g as i32 - other.g as i32;
        let db = self.b as i32 - other.b as i32;

        (dr * dr + dg * dg + db * db) as u32
    }
}

impl BitWrite for Color {
    fn bit_write_msb(&self, writer: &mut crate::bitstream::BitWriter) {
        writer.write_msb(self.r as u32, 8);
        writer.write_msb(self.g as u32, 8);
        writer.write_msb(self.b as u32, 8);
    }
}

impl BitRead for Color {
    fn bit_read_msb(reader: &mut crate::bitstream::BitReader) -> Self {
        let r = reader.read_msb(8).unwrap() as u8;
        let g = reader.read_msb(8).unwrap() as u8;
        let b = reader.read_msb(8).unwrap() as u8;
        Self { r, g, b }
    }
}
