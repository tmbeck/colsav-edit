use crate::bits::{BitReader, BitWriter};

use crate::error::Result;
use crate::goods::Goods16;

/// COLONY section. Each colony = 202 bytes.
pub const COLONY_SIZE: usize = 202;

#[derive(Debug, Clone)]
pub struct Colony {
    pub x: u8,
    pub y: u8,
    pub name_raw: [u8; 24], // 24 bytes raw (may contain embedded nulls)
    pub nation_id: u8,      // nation_type
    pub unknown08a: u8,
    pub colony_flags: u8, // 8-bit bit_struct (preserve raw)
    pub unknown08b: [u8; 2],
    pub population: u8,
    pub occupation: [u8; 32], // occupation_type per colonist slot
    pub profession: [u8; 32], // profession_type per colonist slot
    pub duration: [u8; 16],   // 16 bytes, each containing two 4-bit durations
    pub tiles: [i8; 8],       // colonist index per adjacent tile (N,E,S,W,NW,NE,SE,SW)
    pub unknown10: [u8; 12],
    pub buildings: Buildings,              // 48-bit (6 bytes) bit_struct
    pub custom_house_flags: Goods16<bool>, // 16-bit bitmap
    pub unknown11: [u8; 6],
    pub hammers: u16,
    pub building_in_production: u8,
    pub warehouse_level: u8,
    pub unknown12a: u8,
    pub depletion_counter: u8,
    pub hammers_purchased: u16,
    pub stock: Goods16<i16>,           // 16 × i16
    pub population_on_map: [u8; 4],    // per European nation
    pub fortification_on_map: [u8; 4], // per European nation
    pub rebel_dividend: i32,
    pub rebel_divisor: i32,
}

/// Buildings bitfield (48 bits = 6 bytes).
#[derive(Debug, Clone, Copy, Default)]
pub struct Buildings {
    pub fortification: u8,        // 3 bits (level_3bit_type)
    pub armory: u8,               // 3 bits
    pub docks: u8,                // 3 bits
    pub town_hall: u8,            // 3 bits
    pub schoolhouse: u8,          // 3 bits
    pub warehouse: bool,          // 1 bit
    pub unused05a: bool,          // 1 bit
    pub stables: bool,            // 1 bit
    pub custom_house: bool,       // 1 bit
    pub printing_press: u8,       // 2 bits (level_2bit_type)
    pub weavers_house: u8,        // 3 bits
    pub tobacconists_house: u8,   // 3 bits
    pub rum_distillers_house: u8, // 3 bits
    pub capitol_unused: u8,       // 2 bits
    pub fur_traders_house: u8,    // 3 bits
    pub carpenters_shop: u8,      // 2 bits
    pub church: u8,               // 2 bits
    pub blacksmiths_house: u8,    // 3 bits
    pub unused05b: u8,            // 6 bits
}

impl Buildings {
    pub fn read(data: &[u8]) -> Self {
        let mut r = BitReader::new(&data[..6]);
        Buildings {
            fortification: r.read_u8(3),
            armory: r.read_u8(3),
            docks: r.read_u8(3),
            town_hall: r.read_u8(3),
            schoolhouse: r.read_u8(3),
            warehouse: r.read_bool(),
            unused05a: r.read_bool(),
            stables: r.read_bool(),
            custom_house: r.read_bool(),
            printing_press: r.read_u8(2),
            weavers_house: r.read_u8(3),
            tobacconists_house: r.read_u8(3),
            rum_distillers_house: r.read_u8(3),
            capitol_unused: r.read_u8(2),
            fur_traders_house: r.read_u8(3),
            carpenters_shop: r.read_u8(2),
            church: r.read_u8(2),
            blacksmiths_house: r.read_u8(3),
            unused05b: r.read_u8(6),
        }
    }

    pub fn write(&self, buf: &mut [u8]) {
        let mut w = BitWriter::new(&mut buf[..6]);
        w.write_u8(3, self.fortification);
        w.write_u8(3, self.armory);
        w.write_u8(3, self.docks);
        w.write_u8(3, self.town_hall);
        w.write_u8(3, self.schoolhouse);
        w.write_bool(self.warehouse);
        w.write_bool(self.unused05a);
        w.write_bool(self.stables);
        w.write_bool(self.custom_house);
        w.write_u8(2, self.printing_press);
        w.write_u8(3, self.weavers_house);
        w.write_u8(3, self.tobacconists_house);
        w.write_u8(3, self.rum_distillers_house);
        w.write_u8(2, self.capitol_unused);
        w.write_u8(3, self.fur_traders_house);
        w.write_u8(2, self.carpenters_shop);
        w.write_u8(2, self.church);
        w.write_u8(3, self.blacksmiths_house);
        w.write_u8(6, self.unused05b);
    }
}

impl Colony {
    /// Get the colony name as a string (up to first null).
    pub fn name(&self) -> &str {
        let end = self.name_raw.iter().position(|&b| b == 0).unwrap_or(24);
        std::str::from_utf8(&self.name_raw[..end]).unwrap_or("")
    }

    pub fn read(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        let x = data[pos];
        pos += 1;
        let y = data[pos];
        pos += 1;
        let mut name_raw = [0u8; 24];
        name_raw.copy_from_slice(&data[pos..pos + 24]);
        pos += 24;
        let nation_id = data[pos];
        pos += 1;
        let unknown08a = data[pos];
        pos += 1;
        let colony_flags = data[pos];
        pos += 1;
        let mut unknown08b = [0u8; 2];
        unknown08b.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;
        let population = data[pos];
        pos += 1;

        let mut occupation = [0u8; 32];
        occupation.copy_from_slice(&data[pos..pos + 32]);
        pos += 32;

        let mut profession = [0u8; 32];
        profession.copy_from_slice(&data[pos..pos + 32]);
        pos += 32;

        let mut duration = [0u8; 16];
        duration.copy_from_slice(&data[pos..pos + 16]);
        pos += 16;

        let mut tiles = [0i8; 8];
        for t in &mut tiles {
            *t = data[pos] as i8;
            pos += 1;
        }

        let mut unknown10 = [0u8; 12];
        unknown10.copy_from_slice(&data[pos..pos + 12]);
        pos += 12;

        let buildings = Buildings::read(&data[pos..]);
        pos += 6;

        let custom_house_flags = Goods16::<bool>::read_bitmap_le(&data[pos..]);
        pos += 2;

        let mut unknown11 = [0u8; 6];
        unknown11.copy_from_slice(&data[pos..pos + 6]);
        pos += 6;

        let hammers = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let building_in_production = data[pos];
        pos += 1;
        let warehouse_level = data[pos];
        pos += 1;
        let unknown12a = data[pos];
        pos += 1;
        let depletion_counter = data[pos];
        pos += 1;

        let hammers_purchased = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let stock = Goods16::<i16>::read_le(&data[pos..]);
        pos += 32;

        let mut population_on_map = [0u8; 4];
        population_on_map.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let mut fortification_on_map = [0u8; 4];
        fortification_on_map.copy_from_slice(&data[pos..pos + 4]);
        pos += 4;

        let rebel_dividend =
            i32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        pos += 4;

        let rebel_divisor =
            i32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]);
        let _ = pos + 4; // final pos

        Ok(Colony {
            x,
            y,
            name_raw,
            nation_id,
            unknown08a,
            colony_flags,
            unknown08b,
            population,
            occupation,
            profession,
            duration,
            tiles,
            unknown10,
            buildings,
            custom_house_flags,
            unknown11,
            hammers,
            building_in_production,
            warehouse_level,
            unknown12a,
            depletion_counter,
            hammers_purchased,
            stock,
            population_on_map,
            fortification_on_map,
            rebel_dividend,
            rebel_divisor,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; COLONY_SIZE];
        let mut pos = 0;

        buf[pos] = self.x;
        pos += 1;
        buf[pos] = self.y;
        pos += 1;
        buf[pos..pos + 24].copy_from_slice(&self.name_raw);
        pos += 24;
        buf[pos] = self.nation_id;
        pos += 1;
        buf[pos] = self.unknown08a;
        pos += 1;
        buf[pos] = self.colony_flags;
        pos += 1;
        buf[pos..pos + 2].copy_from_slice(&self.unknown08b);
        pos += 2;
        buf[pos] = self.population;
        pos += 1;
        buf[pos..pos + 32].copy_from_slice(&self.occupation);
        pos += 32;
        buf[pos..pos + 32].copy_from_slice(&self.profession);
        pos += 32;
        buf[pos..pos + 16].copy_from_slice(&self.duration);
        pos += 16;
        for t in &self.tiles {
            buf[pos] = *t as u8;
            pos += 1;
        }
        buf[pos..pos + 12].copy_from_slice(&self.unknown10);
        pos += 12;
        self.buildings.write(&mut buf[pos..]);
        pos += 6;
        self.custom_house_flags.write_bitmap_le(&mut buf[pos..]);
        pos += 2;
        buf[pos..pos + 6].copy_from_slice(&self.unknown11);
        pos += 6;
        buf[pos..pos + 2].copy_from_slice(&self.hammers.to_le_bytes());
        pos += 2;
        buf[pos] = self.building_in_production;
        pos += 1;
        buf[pos] = self.warehouse_level;
        pos += 1;
        buf[pos] = self.unknown12a;
        pos += 1;
        buf[pos] = self.depletion_counter;
        pos += 1;
        buf[pos..pos + 2].copy_from_slice(&self.hammers_purchased.to_le_bytes());
        pos += 2;
        self.stock.write_le(&mut buf[pos..]);
        pos += 32;
        buf[pos..pos + 4].copy_from_slice(&self.population_on_map);
        pos += 4;
        buf[pos..pos + 4].copy_from_slice(&self.fortification_on_map);
        pos += 4;
        buf[pos..pos + 4].copy_from_slice(&self.rebel_dividend.to_le_bytes());
        pos += 4;
        buf[pos..pos + 4].copy_from_slice(&self.rebel_divisor.to_le_bytes());

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buildings_round_trip() {
        let buildings = Buildings {
            fortification: 3,
            armory: 5,
            docks: 2,
            town_hall: 7,
            schoolhouse: 1,
            warehouse: true,
            unused05a: false,
            stables: true,
            custom_house: false,
            printing_press: 2,
            weavers_house: 4,
            tobacconists_house: 6,
            rum_distillers_house: 1,
            capitol_unused: 1,
            fur_traders_house: 3,
            carpenters_shop: 2,
            church: 1,
            blacksmiths_house: 5,
            unused05b: 0b101010,
        };

        let mut buf = [0u8; 6];
        buildings.write(&mut buf);
        let parsed = Buildings::read(&buf);

        assert_eq!(parsed.fortification, buildings.fortification);
        assert_eq!(parsed.armory, buildings.armory);
        assert_eq!(parsed.docks, buildings.docks);
        assert_eq!(parsed.town_hall, buildings.town_hall);
        assert_eq!(parsed.schoolhouse, buildings.schoolhouse);
        assert_eq!(parsed.warehouse, buildings.warehouse);
        assert_eq!(parsed.unused05a, buildings.unused05a);
        assert_eq!(parsed.stables, buildings.stables);
        assert_eq!(parsed.custom_house, buildings.custom_house);
        assert_eq!(parsed.printing_press, buildings.printing_press);
        assert_eq!(parsed.weavers_house, buildings.weavers_house);
        assert_eq!(parsed.tobacconists_house, buildings.tobacconists_house);
        assert_eq!(parsed.rum_distillers_house, buildings.rum_distillers_house);
        assert_eq!(parsed.capitol_unused, buildings.capitol_unused);
        assert_eq!(parsed.fur_traders_house, buildings.fur_traders_house);
        assert_eq!(parsed.carpenters_shop, buildings.carpenters_shop);
        assert_eq!(parsed.church, buildings.church);
        assert_eq!(parsed.blacksmiths_house, buildings.blacksmiths_house);
        assert_eq!(parsed.unused05b, buildings.unused05b);
    }

    #[test]
    fn test_buildings_all_max() {
        let buildings = Buildings {
            fortification: 7,
            armory: 7,
            docks: 7,
            town_hall: 7,
            schoolhouse: 7,
            warehouse: true,
            unused05a: true,
            stables: true,
            custom_house: true,
            printing_press: 3,
            weavers_house: 7,
            tobacconists_house: 7,
            rum_distillers_house: 7,
            capitol_unused: 3,
            fur_traders_house: 7,
            carpenters_shop: 3,
            church: 3,
            blacksmiths_house: 7,
            unused05b: 0b11_1111,
        };

        let mut buf = [0u8; 6];
        buildings.write(&mut buf);
        let parsed = Buildings::read(&buf);

        assert_eq!(parsed.fortification, 7);
        assert_eq!(parsed.armory, 7);
        assert_eq!(parsed.docks, 7);
        assert_eq!(parsed.town_hall, 7);
        assert_eq!(parsed.schoolhouse, 7);
        assert!(parsed.warehouse);
        assert!(parsed.unused05a);
        assert!(parsed.stables);
        assert!(parsed.custom_house);
        assert_eq!(parsed.printing_press, 3);
        assert_eq!(parsed.weavers_house, 7);
        assert_eq!(parsed.tobacconists_house, 7);
        assert_eq!(parsed.rum_distillers_house, 7);
        assert_eq!(parsed.capitol_unused, 3);
        assert_eq!(parsed.fur_traders_house, 7);
        assert_eq!(parsed.carpenters_shop, 3);
        assert_eq!(parsed.church, 3);
        assert_eq!(parsed.blacksmiths_house, 7);
        assert_eq!(parsed.unused05b, 0b11_1111);
    }

    #[test]
    fn test_colony_name() {
        let mut raw = [0u8; COLONY_SIZE];
        raw[2..11].copy_from_slice(b"Jamestown");
        raw[11] = 0;

        let colony = Colony::read(&raw).expect("colony parse should succeed");
        assert_eq!(colony.name(), "Jamestown");
    }
}
