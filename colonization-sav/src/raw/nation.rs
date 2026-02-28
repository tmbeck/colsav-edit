use crate::bits::{BitReader, BitWriter};
use crate::goods::Goods16;
use crate::error::Result;

/// NATION section. 4 entries (European powers only).
/// Each nation record is variable but follows a fixed layout.
/// Based on the JSON schema, total size per nation = ~316 bytes.
///
/// Let's count precisely from the schema:
/// 1 + 1 + 3 + 1 + 1 + 4(founding_fathers) + 1 + 2 + 2 + 2 + 2 + 2 + 2 + 1 + 1 + 4 + 2 + 2
/// + 2 + 4 + 4 + 4 + 2 + 2 + 2 + 4×1(relations_nations) + 8×1(relations_indian)
/// + 4 + 2 + 6 + trade(16 + 32 + 64 + 64 + 64) = ... complicated.
/// We'll just parse field by field and track the total.

pub const NATION_COUNT: usize = 4;

/// Founding fathers bitfield (25 individual + 7 unused = 32 bits = 4 bytes).
#[derive(Debug, Clone, Copy, Default)]
pub struct FoundingFathers {
    pub adam_smith: bool,
    pub jakob_fugger: bool,
    pub peter_minuit: bool,
    pub peter_stuyvesant: bool,
    pub jan_de_witt: bool,
    pub ferdinand_magellan: bool,
    pub francisco_coronado: bool,
    pub hernando_de_soto: bool,
    pub henry_hudson: bool,
    pub sieur_de_la_salle: bool,
    pub hernan_cortes: bool,
    pub george_washington: bool,
    pub paul_revere: bool,
    pub francis_drake: bool,
    pub john_paul_jones: bool,
    pub thomas_jefferson: bool,
    pub pocahontas: bool,
    pub thomas_paine: bool,
    pub simon_bolivar: bool,
    pub benjamin_franklin: bool,
    pub william_brewster: bool,
    pub william_penn: bool,
    pub jean_de_brebeuf: bool,
    pub juan_de_sepulveda: bool,
    pub bartolme_de_las_casas: bool,
    pub unused_bits: u8,             // 7 bits (preserved for round-trip)
}

impl FoundingFathers {
    pub fn read(data: &[u8]) -> Self {
        let mut r = BitReader::new(&data[..4]);
        Self {
            adam_smith: r.read_bool(),
            jakob_fugger: r.read_bool(),
            peter_minuit: r.read_bool(),
            peter_stuyvesant: r.read_bool(),
            jan_de_witt: r.read_bool(),
            ferdinand_magellan: r.read_bool(),
            francisco_coronado: r.read_bool(),
            hernando_de_soto: r.read_bool(),
            henry_hudson: r.read_bool(),
            sieur_de_la_salle: r.read_bool(),
            hernan_cortes: r.read_bool(),
            george_washington: r.read_bool(),
            paul_revere: r.read_bool(),
            francis_drake: r.read_bool(),
            john_paul_jones: r.read_bool(),
            thomas_jefferson: r.read_bool(),
            pocahontas: r.read_bool(),
            thomas_paine: r.read_bool(),
            simon_bolivar: r.read_bool(),
            benjamin_franklin: r.read_bool(),
            william_brewster: r.read_bool(),
            william_penn: r.read_bool(),
            jean_de_brebeuf: r.read_bool(),
            juan_de_sepulveda: r.read_bool(),
            bartolme_de_las_casas: r.read_bool(),
            unused_bits: r.read_u8(7),
        }
    }

    pub fn write(&self, buf: &mut [u8]) {
        let mut w = BitWriter::new(&mut buf[..4]);
        w.write_bool(self.adam_smith);
        w.write_bool(self.jakob_fugger);
        w.write_bool(self.peter_minuit);
        w.write_bool(self.peter_stuyvesant);
        w.write_bool(self.jan_de_witt);
        w.write_bool(self.ferdinand_magellan);
        w.write_bool(self.francisco_coronado);
        w.write_bool(self.hernando_de_soto);
        w.write_bool(self.henry_hudson);
        w.write_bool(self.sieur_de_la_salle);
        w.write_bool(self.hernan_cortes);
        w.write_bool(self.george_washington);
        w.write_bool(self.paul_revere);
        w.write_bool(self.francis_drake);
        w.write_bool(self.john_paul_jones);
        w.write_bool(self.thomas_jefferson);
        w.write_bool(self.pocahontas);
        w.write_bool(self.thomas_paine);
        w.write_bool(self.simon_bolivar);
        w.write_bool(self.benjamin_franklin);
        w.write_bool(self.william_brewster);
        w.write_bool(self.william_penn);
        w.write_bool(self.jean_de_brebeuf);
        w.write_bool(self.juan_de_sepulveda);
        w.write_bool(self.bartolme_de_las_casas);
        w.write_u8(7, self.unused_bits);
    }
}

/// Relation to a nation or tribe (8-bit bit_struct).
#[derive(Debug, Clone, Copy, Default)]
pub struct Relation {
    pub attitude: u8,              // 4 bits
    pub status: u8,                // 3 bits (relation_3bit_type)
    pub irritated_or_unused: bool, // 1 bit
}

impl Relation {
    pub fn read_byte(b: u8) -> Self {
        let mut r = BitReader::new(std::slice::from_ref(&b));
        Self {
            attitude: r.read_u8(4),
            status: r.read_u8(3),
            irritated_or_unused: r.read_bool(),
        }
    }

    pub fn write_byte(&self) -> u8 {
        let mut buf = [0u8; 1];
        let mut w = BitWriter::new(&mut buf);
        w.write_u8(4, self.attitude);
        w.write_u8(3, self.status);
        w.write_bool(self.irritated_or_unused);
        buf[0]
    }
}

/// Trade data for a nation.
#[derive(Debug, Clone)]
pub struct NationTrade {
    pub euro_price: Goods16<u8>,           // 16 × u8
    pub intrinsic_volume: Goods16<i16>,    // 16 × i16
    pub gold: Goods16<i32>,               // 16 × i32
    pub tons_traded: Goods16<i32>,        // 16 × i32
    pub tons_traded2: Goods16<i32>,       // 16 × i32
}

impl NationTrade {
    pub fn byte_size() -> usize {
        16 + 32 + 64 + 64 + 64  // = 240
    }

    pub fn read(data: &[u8]) -> Self {
        let mut pos = 0;
        let euro_price = Goods16::<u8>::read(&data[pos..]);
        pos += 16;
        let intrinsic_volume = Goods16::<i16>::read_le(&data[pos..]);
        pos += 32;
        let gold = Goods16::<i32>::read_le(&data[pos..]);
        pos += 64;
        let tons_traded = Goods16::<i32>::read_le(&data[pos..]);
        pos += 64;
        let tons_traded2 = Goods16::<i32>::read_le(&data[pos..]);
        let _ = pos + 64;

        Self { euro_price, intrinsic_volume, gold, tons_traded, tons_traded2 }
    }

    pub fn write(&self, buf: &mut [u8]) {
        let mut pos = 0;
        self.euro_price.write(&mut buf[pos..]);
        pos += 16;
        self.intrinsic_volume.write_le(&mut buf[pos..]);
        pos += 32;
        self.gold.write_le(&mut buf[pos..]);
        pos += 64;
        self.tons_traded.write_le(&mut buf[pos..]);
        pos += 64;
        self.tons_traded2.write_le(&mut buf[pos..]);
    }
}

#[derive(Debug, Clone)]
pub struct Nation {
    pub unknown19: u8,
    pub tax_rate: u8,
    pub recruit: [u8; 3],          // 3 × profession_type
    pub unused07: u8,
    pub recruit_count: u8,
    pub founding_fathers: FoundingFathers, // 4 bytes
    pub unknown21: u8,
    pub liberty_bells_total: i16,
    pub liberty_bells_last_turn: i16,
    pub unknown22: [u8; 2],
    pub next_founding_father: i16,
    pub founding_father_count: u16,
    pub prob_founding_father_count_end: [u8; 2],
    pub villages_burned: u8,
    pub rebel_sentiment: i8,
    pub unknown23: [u8; 4],
    pub artillery_bought_count: u16,
    pub boycott_bitmap: Goods16<bool>,  // 16-bit bitmap
    pub royal_money: i32,
    pub unknown24b: [u8; 4],
    pub gold: i32,
    pub current_crosses: u16,
    pub needed_crosses: u16,
    pub point_return_from_europe: [u8; 2], // x, y
    pub relation_by_nations: [Relation; 4],
    pub relation_by_indian: [Relation; 8],
    pub unknown26a: [u8; 4],
    pub unknown26b: [u8; 2],
    pub unknown26c: [u8; 6],
    pub trade: NationTrade,
}

impl Nation {
    /// Byte size of one Nation record.
    pub fn byte_size() -> usize {
        // Count: 1+1+3+1+1+4+1+2+2+2+2+2+2+1+1+4+2+2+4+4+4+2+2+2+4+8+4+2+6+240
        // = 1+1+3+1+1+4+1+2+2+2+2+2+2+1+1+4+2+2+4+4+4+2+2+2+4+8+4+2+6+240
        // Let me add: 1+1+3+1+1 = 7, +4=11, +1=12, +2+2+2+2+2+2 = 24, +1+1 = 26,
        // +4 = 30, +2 = 32, +2+4+4+4 = 46, +2+2+2 = 52, +4+8 = 64, +4+2+6 = 76, +240 = 316
        316
    }

    pub fn read(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        let unknown19 = data[pos]; pos += 1;
        let tax_rate = data[pos]; pos += 1;

        let mut recruit = [0u8; 3];
        recruit.copy_from_slice(&data[pos..pos + 3]);
        pos += 3;

        let unused07 = data[pos]; pos += 1;
        let recruit_count = data[pos]; pos += 1;

        let founding_fathers = FoundingFathers::read(&data[pos..]);
        pos += 4;

        let unknown21 = data[pos]; pos += 1;

        let liberty_bells_total = i16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let liberty_bells_last_turn = i16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let mut unknown22 = [0u8; 2];
        unknown22.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;

        let next_founding_father = i16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let founding_father_count = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let mut prob_founding_father_count_end = [0u8; 2];
        prob_founding_father_count_end.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;

        let villages_burned = data[pos]; pos += 1;
        let rebel_sentiment = data[pos] as i8; pos += 1;

        let mut unknown23 = [0u8; 4];
        unknown23.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let artillery_bought_count = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let boycott_bitmap = Goods16::<bool>::read_bitmap_le(&data[pos..]);
        pos += 2;

        let royal_money = i32::from_le_bytes([
            data[pos], data[pos + 1], data[pos + 2], data[pos + 3],
        ]);
        pos += 4;

        let mut unknown24b = [0u8; 4];
        unknown24b.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let gold = i32::from_le_bytes([
            data[pos], data[pos + 1], data[pos + 2], data[pos + 3],
        ]);
        pos += 4;

        let current_crosses = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let needed_crosses = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let mut point_return_from_europe = [0u8; 2];
        point_return_from_europe.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;

        let mut relation_by_nations = [Relation::default(); 4];
        for r in &mut relation_by_nations {
            *r = Relation::read_byte(data[pos]);
            pos += 1;
        }

        let mut relation_by_indian = [Relation::default(); 8];
        for r in &mut relation_by_indian {
            *r = Relation::read_byte(data[pos]);
            pos += 1;
        }

        let mut unknown26a = [0u8; 4];
        unknown26a.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let mut unknown26b = [0u8; 2];
        unknown26b.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;

        let mut unknown26c = [0u8; 6];
        unknown26c.copy_from_slice(&data[pos..pos + 6]);
        pos += 6;

        let trade = NationTrade::read(&data[pos..]);

        Ok(Nation {
            unknown19, tax_rate, recruit, unused07, recruit_count,
            founding_fathers, unknown21, liberty_bells_total,
            liberty_bells_last_turn, unknown22, next_founding_father,
            founding_father_count, prob_founding_father_count_end,
            villages_burned, rebel_sentiment, unknown23,
            artillery_bought_count, boycott_bitmap, royal_money,
            unknown24b, gold, current_crosses, needed_crosses,
            point_return_from_europe, relation_by_nations,
            relation_by_indian, unknown26a, unknown26b, unknown26c,
            trade,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Self::byte_size()];
        let mut pos = 0;

        buf[pos] = self.unknown19; pos += 1;
        buf[pos] = self.tax_rate; pos += 1;
        buf[pos..pos + 3].copy_from_slice(&self.recruit);
        pos += 3;
        buf[pos] = self.unused07; pos += 1;
        buf[pos] = self.recruit_count; pos += 1;

        self.founding_fathers.write(&mut buf[pos..]);
        pos += 4;

        buf[pos] = self.unknown21; pos += 1;
        buf[pos..pos + 2].copy_from_slice(&self.liberty_bells_total.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.liberty_bells_last_turn.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.unknown22);
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.next_founding_father.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.founding_father_count.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.prob_founding_father_count_end);
        pos += 2;
        buf[pos] = self.villages_burned; pos += 1;
        buf[pos] = self.rebel_sentiment as u8; pos += 1;
        buf[pos..pos + 4].copy_from_slice(&self.unknown23);
        pos += 4;
        buf[pos..pos + 2].copy_from_slice(&self.artillery_bought_count.to_le_bytes());
        pos += 2;
        self.boycott_bitmap.write_bitmap_le(&mut buf[pos..]);
        pos += 2;
        buf[pos..pos + 4].copy_from_slice(&self.royal_money.to_le_bytes());
        pos += 4;
        buf[pos..pos + 4].copy_from_slice(&self.unknown24b);
        pos += 4;
        buf[pos..pos + 4].copy_from_slice(&self.gold.to_le_bytes());
        pos += 4;
        buf[pos..pos + 2].copy_from_slice(&self.current_crosses.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.needed_crosses.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.point_return_from_europe);
        pos += 2;

        for r in &self.relation_by_nations {
            buf[pos] = r.write_byte();
            pos += 1;
        }
        for r in &self.relation_by_indian {
            buf[pos] = r.write_byte();
            pos += 1;
        }

        buf[pos..pos + 4].copy_from_slice(&self.unknown26a);
        pos += 4;
        buf[pos..pos + 2].copy_from_slice(&self.unknown26b);
        pos += 2;
        buf[pos..pos + 6].copy_from_slice(&self.unknown26c);
        pos += 6;

        self.trade.write(&mut buf[pos..]);

        buf
    }
}
