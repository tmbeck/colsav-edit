/// Bitfield reader for parsing sub-byte fields from a byte slice.
///
/// Reads bits from MSB to LSB within each byte (big-endian bit order),
/// matching the SAV format's bit_struct convention from pavelbel's JSON.
pub struct BitReader<'a> {
    data: &'a [u8],
    bit_pos: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, bit_pos: 0 }
    }

    /// Read `n` bits (1..=32) as a u32 value, MSB first.
    pub fn read_bits(&mut self, n: usize) -> u32 {
        assert!(n > 0 && n <= 32);
        let mut val: u32 = 0;
        for _ in 0..n {
            let byte_idx = self.bit_pos / 8;
            let bit_idx = 7 - (self.bit_pos % 8); // MSB first
            let bit = (self.data[byte_idx] >> bit_idx) & 1;
            val = (val << 1) | bit as u32;
            self.bit_pos += 1;
        }
        val
    }

    /// Read a single bit as bool.
    pub fn read_bool(&mut self) -> bool {
        self.read_bits(1) == 1
    }

    /// Read `n` bits as a u8.
    pub fn read_u8(&mut self, n: usize) -> u8 {
        self.read_bits(n) as u8
    }

    /// Current bit position.
    pub fn position(&self) -> usize {
        self.bit_pos
    }

    /// Skip `n` bits.
    pub fn skip(&mut self, n: usize) {
        self.bit_pos += n;
    }
}

/// Bitfield writer for packing sub-byte fields into a byte slice.
///
/// Writes bits from MSB to LSB within each byte (big-endian bit order).
pub struct BitWriter<'a> {
    data: &'a mut [u8],
    bit_pos: usize,
}

impl<'a> BitWriter<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        // Zero out the buffer first
        for b in data.iter_mut() {
            *b = 0;
        }
        Self { data, bit_pos: 0 }
    }

    /// Write `n` bits (1..=32) from the low bits of `val`, MSB first.
    pub fn write_bits(&mut self, n: usize, val: u32) {
        assert!(n > 0 && n <= 32);
        for i in (0..n).rev() {
            let bit = (val >> i) & 1;
            let byte_idx = self.bit_pos / 8;
            let bit_idx = 7 - (self.bit_pos % 8);
            if bit == 1 {
                self.data[byte_idx] |= 1 << bit_idx;
            }
            self.bit_pos += 1;
        }
    }

    /// Write a single bool bit.
    pub fn write_bool(&mut self, val: bool) {
        self.write_bits(1, val as u32);
    }

    /// Write `n` bits from a u8.
    pub fn write_u8(&mut self, n: usize, val: u8) {
        self.write_bits(n, val as u32);
    }

    /// Skip `n` bits (leave as zero).
    pub fn skip(&mut self, n: usize) {
        self.bit_pos += n;
    }

    /// Current bit position.
    pub fn position(&self) -> usize {
        self.bit_pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_bits() {
        let mut buf = [0u8; 6];

        // Write a 48-bit building bitfield pattern
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_bits(3, 0b001); // fortification = stockade
            w.write_bits(3, 0b011); // armory = level 2
            w.write_bits(3, 0b000); // docks = none
            w.write_bits(3, 0b001); // town_hall = level 1
            w.write_bits(3, 0b000); // schoolhouse = none
            w.write_bool(true);     // warehouse
            w.write_bool(false);    // unused
            w.write_bool(false);    // stables
            w.write_bool(true);     // custom_house
            w.write_bits(2, 0b01); // printing_press = level 1
            w.write_bits(3, 0b001); // weavers_house = level 1
            w.write_bits(3, 0b000); // tobacconists
            w.write_bits(3, 0b000); // rum_distillers
            w.write_bits(2, 0b00); // capitol
            w.write_bits(3, 0b000); // fur_traders
            w.write_bits(2, 0b01); // carpenters = level 1
            w.write_bits(2, 0b00); // church
            w.write_bits(3, 0b000); // blacksmiths
            w.write_bits(6, 0);    // unused
        }

        // Read it back
        let mut r = BitReader::new(&buf);
        assert_eq!(r.read_bits(3), 0b001); // fortification
        assert_eq!(r.read_bits(3), 0b011); // armory
        assert_eq!(r.read_bits(3), 0b000); // docks
        assert_eq!(r.read_bits(3), 0b001); // town_hall
        assert_eq!(r.read_bits(3), 0b000); // schoolhouse
        assert!(r.read_bool());             // warehouse
        assert!(!r.read_bool());            // unused
        assert!(!r.read_bool());            // stables
        assert!(r.read_bool());             // custom_house
        assert_eq!(r.read_bits(2), 0b01);  // printing_press
        assert_eq!(r.read_bits(3), 0b001); // weavers
    }

    #[test]
    fn read_write_single_byte() {
        let mut buf = [0u8; 1];
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_bits(4, 0b0011); // nation_id = Netherlands
            w.write_bool(true);       // vis English
            w.write_bool(false);      // vis French
            w.write_bool(true);       // vis Spanish
            w.write_bool(false);      // vis Dutch
        }
        let mut r = BitReader::new(&buf);
        assert_eq!(r.read_bits(4), 0b0011);
        assert!(r.read_bool());
        assert!(!r.read_bool());
        assert!(r.read_bool());
        assert!(!r.read_bool());
    }
}
