use std::fmt;

/// Index constants for the 16 goods types.
pub const FOOD: usize = 0;
pub const SUGAR: usize = 1;
pub const TOBACCO: usize = 2;
pub const COTTON: usize = 3;
pub const FURS: usize = 4;
pub const LUMBER: usize = 5;
pub const ORE: usize = 6;
pub const SILVER: usize = 7;
pub const HORSES: usize = 8;
pub const RUM: usize = 9;
pub const CIGARS: usize = 10;
pub const CLOTH: usize = 11;
pub const COATS: usize = 12;
pub const TRADE_GOODS: usize = 13;
pub const TOOLS: usize = 14;
pub const MUSKETS: usize = 15;

pub const GOODS_NAMES: [&str; 16] = [
    "Food", "Sugar", "Tobacco", "Cotton", "Furs", "Lumber", "Ore", "Silver",
    "Horses", "Rum", "Cigars", "Cloth", "Coats", "Trade Goods", "Tools", "Muskets",
];

/// A fixed-size array of 16 values, one per goods type.
/// Generic over the element type (u8, u16, i16, i32, etc).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Goods16<T: Copy>(pub [T; 16]);

impl<T: Copy + Default> Default for Goods16<T> {
    fn default() -> Self {
        Self([T::default(); 16])
    }
}

impl<T: Copy + fmt::Display> fmt::Debug for Goods16<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("Goods16");
        for (i, name) in GOODS_NAMES.iter().enumerate() {
            s.field(name, &format_args!("{}", self.0[i]));
        }
        s.finish()
    }
}

impl<T: Copy> std::ops::Index<usize> for Goods16<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &T {
        &self.0[idx]
    }
}

impl<T: Copy> std::ops::IndexMut<usize> for Goods16<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.0[idx]
    }
}

impl Goods16<u16> {
    pub fn read_le(data: &[u8]) -> Self {
        let mut vals = [0u16; 16];
        for i in 0..16 {
            vals[i] = u16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
        }
        Self(vals)
    }

    pub fn write_le(&self, buf: &mut [u8]) {
        for i in 0..16 {
            let bytes = self.0[i].to_le_bytes();
            buf[i * 2] = bytes[0];
            buf[i * 2 + 1] = bytes[1];
        }
    }
}

impl Goods16<i16> {
    pub fn read_le(data: &[u8]) -> Self {
        let mut vals = [0i16; 16];
        for i in 0..16 {
            vals[i] = i16::from_le_bytes([data[i * 2], data[i * 2 + 1]]);
        }
        Self(vals)
    }

    pub fn write_le(&self, buf: &mut [u8]) {
        for i in 0..16 {
            let bytes = self.0[i].to_le_bytes();
            buf[i * 2] = bytes[0];
            buf[i * 2 + 1] = bytes[1];
        }
    }
}

impl Goods16<i32> {
    pub fn read_le(data: &[u8]) -> Self {
        let mut vals = [0i32; 16];
        for i in 0..16 {
            let off = i * 4;
            vals[i] = i32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
        }
        Self(vals)
    }

    pub fn write_le(&self, buf: &mut [u8]) {
        for i in 0..16 {
            let bytes = self.0[i].to_le_bytes();
            let off = i * 4;
            buf[off..off + 4].copy_from_slice(&bytes);
        }
    }
}

impl Goods16<u8> {
    pub fn read(data: &[u8]) -> Self {
        let mut vals = [0u8; 16];
        vals.copy_from_slice(&data[..16]);
        Self(vals)
    }

    pub fn write(&self, buf: &mut [u8]) {
        buf[..16].copy_from_slice(&self.0);
    }
}

impl Goods16<bool> {
    /// Read 16 boolean flags from a 2-byte (16-bit) little-endian bitmap.
    pub fn read_bitmap_le(data: &[u8]) -> Self {
        let bits = u16::from_le_bytes([data[0], data[1]]);
        let mut vals = [false; 16];
        for i in 0..16 {
            vals[i] = (bits >> i) & 1 == 1;
        }
        Self(vals)
    }

    /// Write 16 boolean flags to a 2-byte (16-bit) little-endian bitmap.
    pub fn write_bitmap_le(&self, buf: &mut [u8]) {
        let mut bits: u16 = 0;
        for i in 0..16 {
            if self.0[i] {
                bits |= 1 << i;
            }
        }
        let bytes = bits.to_le_bytes();
        buf[0] = bytes[0];
        buf[1] = bytes[1];
    }
}
