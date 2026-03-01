use crate::bits::{BitReader, BitWriter};
use crate::error::Result;

/// UNIT section. Each unit = 28 bytes.
pub const UNIT_SIZE: usize = 28;

#[derive(Debug, Clone)]
pub struct Unit {
    pub x: u8,
    pub y: u8,
    pub unit_type: u8, // unit_type enum
    // nation_info: 8-bit bit_struct
    pub nation_id: u8, // 4 bits (nation_4bit_type)
    pub vis_to_english: bool,
    pub vis_to_french: bool,
    pub vis_to_spanish: bool,
    pub vis_to_dutch: bool,
    // unknown15: 8-bit bit_struct
    pub unknown15_upper: u8, // 7 bits
    pub damaged: bool,       // 1 bit
    pub moves: u8,
    pub origin_settlement: u8,
    pub ai_plan_mode: u8, // ASCII char
    pub orders: u8,       // orders_type
    pub goto_x: u8,
    pub goto_y: u8,
    pub unknown18: u8,
    pub holds_occupied: u8,
    pub cargo_items: [u8; 6], // 3 × (2 × 4-bit cargo types) = 6 nibbles in 3 bytes
    pub cargo_hold: [u8; 6],  // 6 × u8 (cargo quantities)
    pub turns_worked: u8,
    pub profession_or_treasure: u8,
    pub next_unit_idx: i16,
    pub prev_unit_idx: i16,
}

impl Unit {
    pub fn read(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        let x = data[pos];
        pos += 1;
        let y = data[pos];
        pos += 1;
        let unit_type = data[pos];
        pos += 1;

        // nation_info bit_struct (1 byte)
        let ni_byte = data[pos];
        pos += 1;
        let mut ni_reader = BitReader::new(std::slice::from_ref(&ni_byte));
        let nation_id = ni_reader.read_u8(4);
        let vis_to_english = ni_reader.read_bool();
        let vis_to_french = ni_reader.read_bool();
        let vis_to_spanish = ni_reader.read_bool();
        let vis_to_dutch = ni_reader.read_bool();

        // unknown15 bit_struct (1 byte)
        let u15_byte = data[pos];
        pos += 1;
        let mut u15_reader = BitReader::new(std::slice::from_ref(&u15_byte));
        let unknown15_upper = u15_reader.read_u8(7);
        let damaged = u15_reader.read_bool();

        let moves = data[pos];
        pos += 1;
        let origin_settlement = data[pos];
        pos += 1;
        let ai_plan_mode = data[pos];
        pos += 1;
        let orders = data[pos];
        pos += 1;
        let goto_x = data[pos];
        pos += 1;
        let goto_y = data[pos];
        pos += 1;
        let unknown18 = data[pos];
        pos += 1;
        let holds_occupied = data[pos];
        pos += 1;

        // cargo_items: 3 bytes, each byte = 2 × 4-bit cargo type
        let mut cargo_items = [0u8; 6];
        for i in 0..3 {
            cargo_items[i * 2] = (data[pos] >> 4) & 0x0F;
            cargo_items[i * 2 + 1] = data[pos] & 0x0F;
            pos += 1;
        }

        let mut cargo_hold = [0u8; 6];
        cargo_hold.copy_from_slice(&data[pos..pos + 6]);
        pos += 6;

        let turns_worked = data[pos];
        pos += 1;
        let profession_or_treasure = data[pos];
        pos += 1;

        let next_unit_idx = i16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let prev_unit_idx = i16::from_le_bytes([data[pos], data[pos + 1]]);

        Ok(Unit {
            x,
            y,
            unit_type,
            nation_id,
            vis_to_english,
            vis_to_french,
            vis_to_spanish,
            vis_to_dutch,
            unknown15_upper,
            damaged,
            moves,
            origin_settlement,
            ai_plan_mode,
            orders,
            goto_x,
            goto_y,
            unknown18,
            holds_occupied,
            cargo_items,
            cargo_hold,
            turns_worked,
            profession_or_treasure,
            next_unit_idx,
            prev_unit_idx,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; UNIT_SIZE];
        let mut pos = 0;

        buf[pos] = self.x;
        pos += 1;
        buf[pos] = self.y;
        pos += 1;
        buf[pos] = self.unit_type;
        pos += 1;

        // nation_info
        {
            let mut w = BitWriter::new(&mut buf[pos..=pos]);
            w.write_u8(4, self.nation_id);
            w.write_bool(self.vis_to_english);
            w.write_bool(self.vis_to_french);
            w.write_bool(self.vis_to_spanish);
            w.write_bool(self.vis_to_dutch);
        }
        pos += 1;

        // unknown15
        {
            let mut w = BitWriter::new(&mut buf[pos..=pos]);
            w.write_u8(7, self.unknown15_upper);
            w.write_bool(self.damaged);
        }
        pos += 1;

        buf[pos] = self.moves;
        pos += 1;
        buf[pos] = self.origin_settlement;
        pos += 1;
        buf[pos] = self.ai_plan_mode;
        pos += 1;
        buf[pos] = self.orders;
        pos += 1;
        buf[pos] = self.goto_x;
        pos += 1;
        buf[pos] = self.goto_y;
        pos += 1;
        buf[pos] = self.unknown18;
        pos += 1;
        buf[pos] = self.holds_occupied;
        pos += 1;

        // cargo_items: pack 6 nibbles into 3 bytes
        for i in 0..3 {
            buf[pos] = (self.cargo_items[i * 2] << 4) | (self.cargo_items[i * 2 + 1] & 0x0F);
            pos += 1;
        }

        buf[pos..pos + 6].copy_from_slice(&self.cargo_hold);
        pos += 6;

        buf[pos] = self.turns_worked;
        pos += 1;
        buf[pos] = self.profession_or_treasure;
        pos += 1;

        buf[pos..pos + 2].copy_from_slice(&self.next_unit_idx.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.prev_unit_idx.to_le_bytes());

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_unit() -> Unit {
        Unit {
            x: 10,
            y: 20,
            unit_type: 0x04,
            nation_id: 3,
            vis_to_english: true,
            vis_to_french: false,
            vis_to_spanish: true,
            vis_to_dutch: false,
            unknown15_upper: 0b0010101,
            damaged: false,
            moves: 7,
            origin_settlement: 2,
            ai_plan_mode: b'A',
            orders: 0x03,
            goto_x: 11,
            goto_y: 21,
            unknown18: 0xEE,
            holds_occupied: 2,
            cargo_items: [1, 2, 3, 4, 5, 6],
            cargo_hold: [10, 11, 12, 13, 14, 15],
            turns_worked: 9,
            profession_or_treasure: 1,
            next_unit_idx: 123,
            prev_unit_idx: -45,
        }
    }

    #[test]
    fn test_unit_nation_info_round_trip() {
        let unit = sample_unit();
        let bytes = unit.write();
        let parsed = Unit::read(&bytes).expect("unit parse should succeed");

        assert_eq!(parsed.nation_id, 3);
        assert!(parsed.vis_to_english);
        assert!(!parsed.vis_to_french);
        assert!(parsed.vis_to_spanish);
        assert!(!parsed.vis_to_dutch);
    }

    #[test]
    fn test_unit_cargo_nibble_packing() {
        let unit = sample_unit();
        let bytes = unit.write();

        assert_eq!(bytes[13], 0x12);
        assert_eq!(bytes[14], 0x34);
        assert_eq!(bytes[15], 0x56);

        let parsed = Unit::read(&bytes).expect("unit parse should succeed");
        assert_eq!(parsed.cargo_items, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_unit_damaged_flag() {
        let mut unit = sample_unit();
        unit.unknown15_upper = 0;
        unit.damaged = true;

        let bytes = unit.write();
        let parsed = Unit::read(&bytes).expect("unit parse should succeed");

        assert_eq!(parsed.unknown15_upper, 0);
        assert!(parsed.damaged);
    }
}
