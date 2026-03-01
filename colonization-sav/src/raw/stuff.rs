use crate::error::Result;

/// STUFF section. Large mixed-data section between INDIAN and TILE maps.
/// Contains unit counts, foreign affairs, tribe dwelling counts,
/// viewport position, and many unknown fields.
///
/// Rather than parse every sub-field, we parse the known fields and
/// preserve the raw bytes for round-trip fidelity.
/// Foreign affairs report sub-struct (16 bytes).
#[derive(Debug, Clone, Copy, Default)]
pub struct ForeignAffairsReport {
    pub populations: [u8; 4], // per European nation
    pub unknown36ab: [u8; 4],
    pub merchant_marine: [u8; 4], // per European nation
    pub ship_counts: [u8; 4],     // per European nation
}

/// Per-nation unit counts (19 bytes: one per unit type).
#[derive(Debug, Clone, Copy, Default)]
pub struct NationUnitCounts {
    pub colonist: u8,
    pub soldier: u8,
    pub pioneer: u8,
    pub missionary: u8,
    pub dragoon: u8,
    pub scout: u8,
    pub tory_regular: u8,
    pub continental_cavalry: u8,
    pub tory_cavalry: u8,
    pub continental_army: u8,
    pub treasure: u8,
    pub artillery: u8,
    pub wagon_train: u8,
    pub caravel: u8,
    pub merchantman: u8,
    pub galleon: u8,
    pub privateer: u8,
    pub frigate: u8,
    pub man_o_war: u8,
}

impl NationUnitCounts {
    pub const SIZE: usize = 19;

    pub fn read(data: &[u8]) -> Self {
        Self {
            colonist: data[0],
            soldier: data[1],
            pioneer: data[2],
            missionary: data[3],
            dragoon: data[4],
            scout: data[5],
            tory_regular: data[6],
            continental_cavalry: data[7],
            tory_cavalry: data[8],
            continental_army: data[9],
            treasure: data[10],
            artillery: data[11],
            wagon_train: data[12],
            caravel: data[13],
            merchantman: data[14],
            galleon: data[15],
            privateer: data[16],
            frigate: data[17],
            man_o_war: data[18],
        }
    }

    pub fn write(&self, buf: &mut [u8]) {
        buf[0] = self.colonist;
        buf[1] = self.soldier;
        buf[2] = self.pioneer;
        buf[3] = self.missionary;
        buf[4] = self.dragoon;
        buf[5] = self.scout;
        buf[6] = self.tory_regular;
        buf[7] = self.continental_cavalry;
        buf[8] = self.tory_cavalry;
        buf[9] = self.continental_army;
        buf[10] = self.treasure;
        buf[11] = self.artillery;
        buf[12] = self.wagon_train;
        buf[13] = self.caravel;
        buf[14] = self.merchantman;
        buf[15] = self.galleon;
        buf[16] = self.privateer;
        buf[17] = self.frigate;
        buf[18] = self.man_o_war;
    }
}

/// Tribe data block (8 bytes, one per Indian nation).
#[derive(Debug, Clone, Copy, Default)]
pub struct TribeDataBlock {
    pub inca: u8,
    pub aztec: u8,
    pub arawak: u8,
    pub iroquois: u8,
    pub cherokee: u8,
    pub apache: u8,
    pub sioux: u8,
    pub tupi: u8,
}

impl TribeDataBlock {
    pub const SIZE: usize = 8;

    pub fn read(data: &[u8]) -> Self {
        Self {
            inca: data[0],
            aztec: data[1],
            arawak: data[2],
            iroquois: data[3],
            cherokee: data[4],
            apache: data[5],
            sioux: data[6],
            tupi: data[7],
        }
    }

    pub fn write(&self, buf: &mut [u8]) {
        buf[0] = self.inca;
        buf[1] = self.aztec;
        buf[2] = self.arawak;
        buf[3] = self.iroquois;
        buf[4] = self.cherokee;
        buf[5] = self.apache;
        buf[6] = self.sioux;
        buf[7] = self.tupi;
    }
}

#[derive(Debug, Clone)]
pub struct Stuff {
    pub unknown34: [u8; 12],
    pub all_unit_counts: [u8; 4],       // per European nation
    pub unknwn_nation_data_35: [u8; 4], // per European nation
    pub unknown36aa: [u8; 8],
    pub foreign_affairs: ForeignAffairsReport,
    pub unknwn_nation_data_36ac1: [u16; 4],
    pub unknwn_nation_data_36ac2: [u16; 4],
    pub unknwn_nation_data_36ac3: [u8; 4],
    pub unit_counts: [NationUnitCounts; 4],
    pub unknown36ac: [u8; 416],
    pub average_colony: [u16; 4],
    pub show_colony_prod_quantities: u8,
    pub unknown_tribe_data_1: TribeDataBlock,
    pub unknown_tribe_data_2: TribeDataBlock,
    pub tribe_dwelling_count: TribeDataBlock,
    pub unknown_tribe_data_4: TribeDataBlock,
    pub unknown_tribe_data_5: TribeDataBlock,
    pub unknown_tribe_data_6: TribeDataBlock,
    pub unknown36b: [u8; 104],
    pub selector_x: u16,
    pub selector_y: u16,
    pub zoom_level: u8,
    pub unknown37: u8,
    pub viewport_x: u16,
    pub viewport_y: u16,
}

impl Stuff {
    /// Byte size of the Stuff section.
    pub fn byte_size() -> usize {
        // 12 + 4 + 4 + 8 + 16 + 8 + 8 + 4 + 76 + 416 + 8 + 1 + 48 + 104 + 4 + 4 + 1 + 1 + 2 + 2
        // Let's be precise:
        // 12 + 4 + 4 + 8 = 28
        // + 16 (foreign affairs) = 44
        // + 8 (36ac1) + 8 (36ac2) + 4 (36ac3) = 64
        // + 19*4 (unit_counts) = 76 → 140
        // + 416 = 556
        // + 8 (average_colony) + 1 = 565
        // + 8*6 (tribe data blocks) = 48 → 613
        // + 104 = 717
        // + 2+2+1+1+2+2 = 10 → 727
        727
    }

    pub fn read(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        let mut unknown34 = [0u8; 12];
        unknown34.copy_from_slice(&data[pos..pos + 12]);
        pos += 12;

        let mut all_unit_counts = [0u8; 4];
        all_unit_counts.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let mut unknwn_nation_data_35 = [0u8; 4];
        unknwn_nation_data_35.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let mut unknown36aa = [0u8; 8];
        unknown36aa.copy_from_slice(&data[pos..pos + 8]);
        pos += 8;

        // Foreign affairs report
        let mut fa = ForeignAffairsReport::default();
        fa.populations.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;
        fa.unknown36ab.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;
        fa.merchant_marine.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;
        fa.ship_counts.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let mut unknwn_nation_data_36ac1 = [0u16; 4];
        for v in &mut unknwn_nation_data_36ac1 {
            *v = u16::from_le_bytes([data[pos], data[pos + 1]]);
            pos += 2;
        }

        let mut unknwn_nation_data_36ac2 = [0u16; 4];
        for v in &mut unknwn_nation_data_36ac2 {
            *v = u16::from_le_bytes([data[pos], data[pos + 1]]);
            pos += 2;
        }

        let mut unknwn_nation_data_36ac3 = [0u8; 4];
        unknwn_nation_data_36ac3.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let mut unit_counts = [NationUnitCounts::default(); 4];
        for uc in &mut unit_counts {
            *uc = NationUnitCounts::read(&data[pos..]);
            pos += NationUnitCounts::SIZE;
        }

        let mut unknown36ac = [0u8; 416];
        unknown36ac.copy_from_slice(&data[pos..pos + 416]);
        pos += 416;

        let mut average_colony = [0u16; 4];
        for v in &mut average_colony {
            *v = u16::from_le_bytes([data[pos], data[pos + 1]]);
            pos += 2;
        }

        let show_colony_prod_quantities = data[pos];
        pos += 1;

        let unknown_tribe_data_1 = TribeDataBlock::read(&data[pos..]);
        pos += TribeDataBlock::SIZE;
        let unknown_tribe_data_2 = TribeDataBlock::read(&data[pos..]);
        pos += TribeDataBlock::SIZE;
        let tribe_dwelling_count = TribeDataBlock::read(&data[pos..]);
        pos += TribeDataBlock::SIZE;
        let unknown_tribe_data_4 = TribeDataBlock::read(&data[pos..]);
        pos += TribeDataBlock::SIZE;
        let unknown_tribe_data_5 = TribeDataBlock::read(&data[pos..]);
        pos += TribeDataBlock::SIZE;
        let unknown_tribe_data_6 = TribeDataBlock::read(&data[pos..]);
        pos += TribeDataBlock::SIZE;

        let mut unknown36b = [0u8; 104];
        unknown36b.copy_from_slice(&data[pos..pos + 104]);
        pos += 104;

        let selector_x = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let selector_y = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let zoom_level = data[pos];
        pos += 1;
        let unknown37 = data[pos];
        pos += 1;
        let viewport_x = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let viewport_y = u16::from_le_bytes([data[pos], data[pos + 1]]);

        Ok(Stuff {
            unknown34,
            all_unit_counts,
            unknwn_nation_data_35,
            unknown36aa,
            foreign_affairs: fa,
            unknwn_nation_data_36ac1,
            unknwn_nation_data_36ac2,
            unknwn_nation_data_36ac3,
            unit_counts,
            unknown36ac,
            average_colony,
            show_colony_prod_quantities,
            unknown_tribe_data_1,
            unknown_tribe_data_2,
            tribe_dwelling_count,
            unknown_tribe_data_4,
            unknown_tribe_data_5,
            unknown_tribe_data_6,
            unknown36b,
            selector_x,
            selector_y,
            zoom_level,
            unknown37,
            viewport_x,
            viewport_y,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Self::byte_size()];
        let mut pos = 0;

        buf[pos..pos + 12].copy_from_slice(&self.unknown34);
        pos += 12;

        buf[pos..pos + 4].copy_from_slice(&self.all_unit_counts);
        pos += 4;

        buf[pos..pos + 4].copy_from_slice(&self.unknwn_nation_data_35);
        pos += 4;

        buf[pos..pos + 8].copy_from_slice(&self.unknown36aa);
        pos += 8;

        // Foreign affairs
        buf[pos..pos + 4].copy_from_slice(&self.foreign_affairs.populations);
        pos += 4;
        buf[pos..pos + 4].copy_from_slice(&self.foreign_affairs.unknown36ab);
        pos += 4;
        buf[pos..pos + 4].copy_from_slice(&self.foreign_affairs.merchant_marine);
        pos += 4;
        buf[pos..pos + 4].copy_from_slice(&self.foreign_affairs.ship_counts);
        pos += 4;

        for v in &self.unknwn_nation_data_36ac1 {
            buf[pos..pos + 2].copy_from_slice(&v.to_le_bytes());
            pos += 2;
        }

        for v in &self.unknwn_nation_data_36ac2 {
            buf[pos..pos + 2].copy_from_slice(&v.to_le_bytes());
            pos += 2;
        }

        buf[pos..pos + 4].copy_from_slice(&self.unknwn_nation_data_36ac3);
        pos += 4;

        for uc in &self.unit_counts {
            uc.write(&mut buf[pos..]);
            pos += NationUnitCounts::SIZE;
        }

        buf[pos..pos + 416].copy_from_slice(&self.unknown36ac);
        pos += 416;

        for v in &self.average_colony {
            buf[pos..pos + 2].copy_from_slice(&v.to_le_bytes());
            pos += 2;
        }

        buf[pos] = self.show_colony_prod_quantities;
        pos += 1;

        self.unknown_tribe_data_1.write(&mut buf[pos..]);
        pos += TribeDataBlock::SIZE;
        self.unknown_tribe_data_2.write(&mut buf[pos..]);
        pos += TribeDataBlock::SIZE;
        self.tribe_dwelling_count.write(&mut buf[pos..]);
        pos += TribeDataBlock::SIZE;
        self.unknown_tribe_data_4.write(&mut buf[pos..]);
        pos += TribeDataBlock::SIZE;
        self.unknown_tribe_data_5.write(&mut buf[pos..]);
        pos += TribeDataBlock::SIZE;
        self.unknown_tribe_data_6.write(&mut buf[pos..]);
        pos += TribeDataBlock::SIZE;

        buf[pos..pos + 104].copy_from_slice(&self.unknown36b);
        pos += 104;

        buf[pos..pos + 2].copy_from_slice(&self.selector_x.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.selector_y.to_le_bytes());
        pos += 2;
        buf[pos] = self.zoom_level;
        pos += 1;
        buf[pos] = self.unknown37;
        pos += 1;
        buf[pos..pos + 2].copy_from_slice(&self.viewport_x.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.viewport_y.to_le_bytes());

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nation_unit_counts_round_trip() {
        let counts = NationUnitCounts {
            colonist: 1,
            soldier: 2,
            pioneer: 3,
            missionary: 4,
            dragoon: 5,
            scout: 6,
            tory_regular: 7,
            continental_cavalry: 8,
            tory_cavalry: 9,
            continental_army: 10,
            treasure: 11,
            artillery: 12,
            wagon_train: 13,
            caravel: 14,
            merchantman: 15,
            galleon: 16,
            privateer: 17,
            frigate: 18,
            man_o_war: 19,
        };

        let mut buf = [0u8; NationUnitCounts::SIZE];
        counts.write(&mut buf);
        let parsed = NationUnitCounts::read(&buf);

        assert_eq!(parsed.colonist, counts.colonist);
        assert_eq!(parsed.soldier, counts.soldier);
        assert_eq!(parsed.pioneer, counts.pioneer);
        assert_eq!(parsed.missionary, counts.missionary);
        assert_eq!(parsed.dragoon, counts.dragoon);
        assert_eq!(parsed.scout, counts.scout);
        assert_eq!(parsed.tory_regular, counts.tory_regular);
        assert_eq!(parsed.continental_cavalry, counts.continental_cavalry);
        assert_eq!(parsed.tory_cavalry, counts.tory_cavalry);
        assert_eq!(parsed.continental_army, counts.continental_army);
        assert_eq!(parsed.treasure, counts.treasure);
        assert_eq!(parsed.artillery, counts.artillery);
        assert_eq!(parsed.wagon_train, counts.wagon_train);
        assert_eq!(parsed.caravel, counts.caravel);
        assert_eq!(parsed.merchantman, counts.merchantman);
        assert_eq!(parsed.galleon, counts.galleon);
        assert_eq!(parsed.privateer, counts.privateer);
        assert_eq!(parsed.frigate, counts.frigate);
        assert_eq!(parsed.man_o_war, counts.man_o_war);
    }

    #[test]
    fn test_tribe_data_block_round_trip() {
        let block = TribeDataBlock {
            inca: 10,
            aztec: 11,
            arawak: 12,
            iroquois: 13,
            cherokee: 14,
            apache: 15,
            sioux: 16,
            tupi: 17,
        };

        let mut buf = [0u8; TribeDataBlock::SIZE];
        block.write(&mut buf);
        let parsed = TribeDataBlock::read(&buf);

        assert_eq!(parsed.inca, block.inca);
        assert_eq!(parsed.aztec, block.aztec);
        assert_eq!(parsed.arawak, block.arawak);
        assert_eq!(parsed.iroquois, block.iroquois);
        assert_eq!(parsed.cherokee, block.cherokee);
        assert_eq!(parsed.apache, block.apache);
        assert_eq!(parsed.sioux, block.sioux);
        assert_eq!(parsed.tupi, block.tupi);
    }
}
