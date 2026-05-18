pub struct BitStream {
    buffer: Vec<u8>,
    bit_rem: u8,
}

impl BitStream {
    pub fn new(buffer: Vec<u8>, bit_rem: u8) -> Self {
        Self { buffer, bit_rem }
    }

    pub fn reader(&self) -> BitReader<'_> {
        BitReader::new(&self.buffer)
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut buffer = self.buffer;
        buffer.push(self.bit_rem);
        buffer
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let bit_rem = *bytes.last().unwrap();
        Self {
            buffer: bytes[..bytes.len() - 1].to_vec(),
            bit_rem,
        }
    }
}

impl From<BitWriter> for BitStream {
    fn from(value: BitWriter) -> Self {
        Self::new(value.buffer, value.bit_rem)
    }
}

pub struct BitWriter {
    buffer: Vec<u8>,
    bit_rem: u8, // バッファの最後の要素の空きビット数
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            bit_rem: 0,
        }
    }

    pub fn write_msb(&mut self, value: u32, size: usize) {
        let mut remaining = size;
        while remaining > 0 {
            if self.bit_rem == 0 {
                self.buffer.push(0);
                self.bit_rem = 8;
            }
            let take = remaining.min(self.bit_rem as usize);
            let bits = ((value >> (remaining - take)) & ((1u32 << take) - 1)) as u8;
            let shift = self.bit_rem as usize - take;
            *self.buffer.last_mut().unwrap() |= bits << shift;
            self.bit_rem -= take as u8;
            remaining -= take;
        }
    }

    #[inline]
    pub fn to_stream(self) -> BitStream {
        BitStream::from(self)
    }
}

pub trait BitWrite {
    fn bit_write_msb(&self, writer: &mut BitWriter);
}

pub struct BitReader<'a> {
    buffer: &'a [u8],
    byte_idx: usize,
    bit_rem: u8, // 現在バイトの未読ビット数 (1..=8)
}

impl<'a> BitReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            byte_idx: 0,
            bit_rem: 8,
        }
    }

    pub fn read_msb(&mut self, size: usize) -> Option<u32> {
        let mut remaining = size;
        let mut result: u32 = 0;
        while remaining > 0 {
            if self.bit_rem == 0 {
                self.byte_idx += 1;
                self.bit_rem = 8;
            }
            if self.byte_idx >= self.buffer.len() {
                return None;
            }
            let take = remaining.min(self.bit_rem as usize);
            let shift = self.bit_rem as usize - take;
            let bits = (self.buffer[self.byte_idx] as u32 >> shift) & ((1u32 << take) - 1);
            result = (result << take) | bits;
            self.bit_rem -= take as u8;
            remaining -= take;
        }
        Some(result)
    }
}

pub trait BitRead {
    fn bit_read_msb(reader: &mut BitReader) -> Self;
}
