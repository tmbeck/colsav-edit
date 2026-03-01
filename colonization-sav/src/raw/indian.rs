use crate::bits::{BitReader, BitWriter};
use crate::error::Result;
use crate::goods::Goods16;
use crate::raw::nation::Relation;

/// INDIAN section. Always 8 entries (one per Indian nation).
/// Each entry ~80 bytes.
pub const INDIAN_COUNT: usize = 8;

#[derive(Debug, Clone, Copy, Default)]
pub struct TribeFlags {
    pub unknown01: u8, // 7 bits
    pub extinct: bool, // 1 bit
}

impl TribeFlags {
    pub fn read_byte(b: u8) -> Self {
        let mut r = BitReader::new(std::slice::from_ref(&b));
        Self {
            unknown01: r.read_u8(7),
            extinct: r.read_bool(),
        }
    }

    pub fn write_byte(&self) -> u8 {
        let mut buf = [0u8; 1];
        let mut w = BitWriter::new(&mut buf);
        w.write_u8(7, self.unknown01);
        w.write_bool(self.extinct);
        buf[0]
    }
}

#[derive(Debug, Clone)]
pub struct Indian {
    pub capitol_x: u8,
    pub capitol_y: u8,
    pub tech: u8, // tech_type
    pub tribe_flags: TribeFlags,
    pub unknown31b: [u8; 3],
    pub muskets: u8, // not including muskets equipped by braves
    pub horse_herds: u8,
    pub unknown31c: u8,
    pub horse_breeding: u16,
    pub unknown31d: [u8; 2],
    pub stock: Goods16<i16>, // 16 × i16 = 32 bytes
    pub unknown32: [u8; 12],
    pub relation_by_nations: [Relation; 4],
    pub zeros33: [u8; 8],
    pub alarm_by_player: [u16; 4],
}

impl Indian {
    /// Byte size of one Indian record.
    pub fn byte_size() -> usize {
        // 1+1+1+1+3+1+1+1+2+2+32+12+4+8+8 = 78
        78
    }

    pub fn read(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        let capitol_x = data[pos];
        pos += 1;
        let capitol_y = data[pos];
        pos += 1;
        let tech = data[pos];
        pos += 1;
        let tribe_flags = TribeFlags::read_byte(data[pos]);
        pos += 1;

        let mut unknown31b = [0u8; 3];
        unknown31b.copy_from_slice(&data[pos..pos + 3]);
        pos += 3;

        let muskets = data[pos];
        pos += 1;
        let horse_herds = data[pos];
        pos += 1;
        let unknown31c = data[pos];
        pos += 1;

        let horse_breeding = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let mut unknown31d = [0u8; 2];
        unknown31d.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;

        let stock = Goods16::<i16>::read_le(&data[pos..]);
        pos += 32;

        let mut unknown32 = [0u8; 12];
        unknown32.copy_from_slice(&data[pos..pos + 12]);
        pos += 12;

        let mut relation_by_nations = [Relation::default(); 4];
        for r in &mut relation_by_nations {
            *r = Relation::read_byte(data[pos]);
            pos += 1;
        }

        let mut zeros33 = [0u8; 8];
        zeros33.copy_from_slice(&data[pos..pos + 8]);
        pos += 8;

        let mut alarm_by_player = [0u16; 4];
        for a in &mut alarm_by_player {
            *a = u16::from_le_bytes([data[pos], data[pos + 1]]);
            pos += 2;
        }

        Ok(Indian {
            capitol_x,
            capitol_y,
            tech,
            tribe_flags,
            unknown31b,
            muskets,
            horse_herds,
            unknown31c,
            horse_breeding,
            unknown31d,
            stock,
            unknown32,
            relation_by_nations,
            zeros33,
            alarm_by_player,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Self::byte_size()];
        let mut pos = 0;

        buf[pos] = self.capitol_x;
        pos += 1;
        buf[pos] = self.capitol_y;
        pos += 1;
        buf[pos] = self.tech;
        pos += 1;
        buf[pos] = self.tribe_flags.write_byte();
        pos += 1;

        buf[pos..pos + 3].copy_from_slice(&self.unknown31b);
        pos += 3;

        buf[pos] = self.muskets;
        pos += 1;
        buf[pos] = self.horse_herds;
        pos += 1;
        buf[pos] = self.unknown31c;
        pos += 1;

        buf[pos..pos + 2].copy_from_slice(&self.horse_breeding.to_le_bytes());
        pos += 2;

        buf[pos..pos + 2].copy_from_slice(&self.unknown31d);
        pos += 2;

        self.stock.write_le(&mut buf[pos..]);
        pos += 32;

        buf[pos..pos + 12].copy_from_slice(&self.unknown32);
        pos += 12;

        for r in &self.relation_by_nations {
            buf[pos] = r.write_byte();
            pos += 1;
        }

        buf[pos..pos + 8].copy_from_slice(&self.zeros33);
        pos += 8;

        for a in &self.alarm_by_player {
            buf[pos..pos + 2].copy_from_slice(&a.to_le_bytes());
            pos += 2;
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tribe_flags_round_trip() {
        let flags = TribeFlags {
            unknown01: 42,
            extinct: true,
        };

        let byte = flags.write_byte();
        let parsed = TribeFlags::read_byte(byte);

        assert_eq!(parsed.unknown01, 42);
        assert!(parsed.extinct);
    }
}
