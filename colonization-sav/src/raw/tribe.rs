use crate::bits::{BitReader, BitWriter};
use crate::error::Result;

/// TRIBE section. Variable count (from header.tribe_count).
/// Each tribe dwelling = 18 bytes.
pub const TRIBE_SIZE: usize = 18;

#[derive(Debug, Clone, Copy, Default)]
pub struct TribeBLCS {
    pub brave_missing: bool,
    pub learned: bool,
    pub capital: bool,
    pub scouted: bool,
    pub unused: u8,             // 4 bits
}

impl TribeBLCS {
    pub fn read_byte(b: u8) -> Self {
        let mut r = BitReader::new(std::slice::from_ref(&b));
        Self {
            brave_missing: r.read_bool(),
            learned: r.read_bool(),
            capital: r.read_bool(),
            scouted: r.read_bool(),
            unused: r.read_u8(4),
        }
    }

    pub fn write_byte(&self) -> u8 {
        let mut buf = [0u8; 1];
        let mut w = BitWriter::new(&mut buf);
        w.write_bool(self.brave_missing);
        w.write_bool(self.learned);
        w.write_bool(self.capital);
        w.write_bool(self.scouted);
        w.write_u8(4, self.unused);
        buf[0]
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TribeMission {
    pub nation_id: u8,          // 4 bits (nation_4bit_type)
    pub expert: bool,           // 1 bit
    pub unknown: u8,            // 3 bits
}

impl TribeMission {
    pub fn read_byte(b: u8) -> Self {
        let mut r = BitReader::new(std::slice::from_ref(&b));
        Self {
            nation_id: r.read_u8(4),
            expert: r.read_bool(),
            unknown: r.read_u8(3),
        }
    }

    pub fn write_byte(&self) -> u8 {
        let mut buf = [0u8; 1];
        let mut w = BitWriter::new(&mut buf);
        w.write_u8(4, self.nation_id);
        w.write_bool(self.expert);
        w.write_u8(3, self.unknown);
        buf[0]
    }
}

/// Per-nation alarm for a tribe dwelling.
#[derive(Debug, Clone, Copy, Default)]
pub struct TribeAlarm {
    pub friction: u8,
    pub attacks: u8,
}

#[derive(Debug, Clone)]
pub struct Tribe {
    pub x: u8,
    pub y: u8,
    pub nation_id: u8,          // nation_type (Indian nation 4-11)
    pub blcs: TribeBLCS,
    pub population: u8,
    pub mission: TribeMission,
    pub growth_counter: u8,
    pub unknown28a: u8,         // always seems to be 0xFF
    pub last_bought: u8,
    pub last_sold: u8,
    pub alarm: [TribeAlarm; 4], // per European power
}

impl Tribe {
    pub fn read(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        let x = data[pos]; pos += 1;
        let y = data[pos]; pos += 1;
        let nation_id = data[pos]; pos += 1;
        let blcs = TribeBLCS::read_byte(data[pos]); pos += 1;
        let population = data[pos]; pos += 1;
        let mission = TribeMission::read_byte(data[pos]); pos += 1;
        let growth_counter = data[pos]; pos += 1;
        let unknown28a = data[pos]; pos += 1;
        let last_bought = data[pos]; pos += 1;
        let last_sold = data[pos]; pos += 1;

        let mut alarm = [TribeAlarm::default(); 4];
        for a in &mut alarm {
            a.friction = data[pos]; pos += 1;
            a.attacks = data[pos]; pos += 1;
        }

        Ok(Tribe {
            x, y, nation_id, blcs, population, mission,
            growth_counter, unknown28a, last_bought, last_sold,
            alarm,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; TRIBE_SIZE];
        let mut pos = 0;

        buf[pos] = self.x; pos += 1;
        buf[pos] = self.y; pos += 1;
        buf[pos] = self.nation_id; pos += 1;
        buf[pos] = self.blcs.write_byte(); pos += 1;
        buf[pos] = self.population; pos += 1;
        buf[pos] = self.mission.write_byte(); pos += 1;
        buf[pos] = self.growth_counter; pos += 1;
        buf[pos] = self.unknown28a; pos += 1;
        buf[pos] = self.last_bought; pos += 1;
        buf[pos] = self.last_sold; pos += 1;

        for a in &self.alarm {
            buf[pos] = a.friction; pos += 1;
            buf[pos] = a.attacks; pos += 1;
        }

        buf
    }
}
