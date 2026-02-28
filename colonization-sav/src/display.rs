use std::fmt;

use crate::enums::{
    ControlType, Difficulty, NationType, OrdersType, ProfessionType, Season, UnitType, CargoType,
};
use crate::goods::GOODS_NAMES;
use crate::raw::{Colony, Header, Nation, Player, Unit};

fn unknown_u8(value: u8) -> String {
    format!("Unknown(0x{value:02X})")
}

fn constructable_name(value: u8) -> String {
    match value {
        0x00 => "Stockade".to_string(),
        0x01 => "Fort".to_string(),
        0x02 => "Fortress".to_string(),
        0x03 => "Armory".to_string(),
        0x04 => "Magazine".to_string(),
        0x05 => "Arsenal".to_string(),
        0x06 => "Docks".to_string(),
        0x07 => "Drydock".to_string(),
        0x08 => "Shipyard".to_string(),
        0x09 => "Town Hall".to_string(),
        0x0C => "Schoolhouse".to_string(),
        0x0D => "College".to_string(),
        0x0E => "University".to_string(),
        0x0F => "Warehouse".to_string(),
        0x10 => "Warehouse Expansion".to_string(),
        0x11 => "Stable".to_string(),
        0x12 => "Custom House".to_string(),
        0x13 => "Printing Press".to_string(),
        0x14 => "Newspaper".to_string(),
        0x15 => "Weaver's House".to_string(),
        0x16 => "Weaver's Shop".to_string(),
        0x17 => "Textile Mill".to_string(),
        0x18 => "Tobacconist's House".to_string(),
        0x19 => "Tobacconist's Shop".to_string(),
        0x1A => "Cigar Factory".to_string(),
        0x1B => "Rum Distiller's House".to_string(),
        0x1C => "Rum Distiller's Shop".to_string(),
        0x1D => "Rum Factory".to_string(),
        0x20 => "Fur Trader's House".to_string(),
        0x21 => "Fur Trading Post".to_string(),
        0x22 => "Fur Factory".to_string(),
        0x23 => "Carpenter's Shop".to_string(),
        0x24 => "Lumber Mill".to_string(),
        0x25 => "Church".to_string(),
        0x26 => "Cathedral".to_string(),
        0x27 => "Blacksmith's House".to_string(),
        0x28 => "Blacksmith's Shop".to_string(),
        0x29 => "Iron Works".to_string(),
        0x2A => "Artillery".to_string(),
        0x2B => "Wagon Train".to_string(),
        0x2C => "Caravel".to_string(),
        0x2D => "Merchantman".to_string(),
        0x2E => "Galleon".to_string(),
        0x2F => "Privateer".to_string(),
        0x30 => "Frigate".to_string(),
        0xFF => "Nothing".to_string(),
        other => unknown_u8(other),
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit_type = UnitType::try_from(self.unit_type)
            .map(|v| v.to_string())
            .unwrap_or_else(|v| unknown_u8(v));
        let power = NationType::try_from(self.nation_id)
            .map(|v| v.to_string())
            .unwrap_or_else(|v| unknown_u8(v));
        let occupation = ProfessionType::try_from(self.profession_or_treasure)
            .map(|v| v.to_string())
            .unwrap_or_else(|v| unknown_u8(v));
        let order = OrdersType::try_from(self.orders)
            .map(|v| v.to_string())
            .unwrap_or_else(|v| unknown_u8(v));

        writeln!(f, "Type: {unit_type}")?;
        if matches!(UnitType::try_from(self.unit_type), Ok(UnitType::Pioneer)) {
            writeln!(f, "  Tools: {}", self.turns_worked)?;
        }
        writeln!(f, "Position: {}, {}", self.x, self.y)?;
        writeln!(f, "  Power: {power}")?;
        writeln!(f, "  Occupation: {occupation}")?;
        writeln!(f, "  Order: {order}")?;
        writeln!(f, "  Destination: ({}, {})", self.goto_x, self.goto_y)?;

        let cargo_count = usize::min(self.holds_occupied as usize, self.cargo_items.len());
        for i in 0..cargo_count {
            let cargo_name = CargoType::try_from(self.cargo_items[i])
                .map(|v| v.to_string())
                .unwrap_or_else(|v| unknown_u8(v));
            writeln!(f, "    Cargo {}: {} {}", i + 1, cargo_name, self.cargo_hold[i])?;
        }

        Ok(())
    }
}

impl fmt::Display for Colony {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let power = NationType::try_from(self.nation_id)
            .map(|v| v.to_string())
            .unwrap_or_else(|v| unknown_u8(v));

        writeln!(f, "Name: {}", self.name())?;
        writeln!(f, "Position: ({}, {})", self.x, self.y)?;
        writeln!(f, "  Power: {power}")?;
        writeln!(f, "  Population: {}", self.population)?;
        writeln!(f, "  Hammers: {}", self.hammers)?;
        writeln!(f, "  Constructing: {}", constructable_name(self.building_in_production))?;
        writeln!(f, "  Bells: {}", self.rebel_dividend)?;
        writeln!(f, "  Storage:")?;

        for (idx, goods_name) in GOODS_NAMES.iter().enumerate() {
            writeln!(f, "    {goods_name:<11}: {:>6}", self.stock[idx])?;
        }

        Ok(())
    }
}

impl fmt::Display for Nation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Tax Rate: {}", self.tax_rate)?;
        writeln!(f, "  Gold: {}", self.gold)?;
        writeln!(f, "  Liberty Bells: {}", self.liberty_bells_total)?;
        writeln!(f, "  Founding Fathers: {}", self.founding_father_count)?;
        writeln!(f, "  Rebel Sentiment: {}", self.rebel_sentiment)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let control = ControlType::try_from(self.control)
            .map(|v| v.to_string())
            .unwrap_or_else(|v| unknown_u8(v));

        writeln!(f, "Player: {}", self.name())?;
        writeln!(f, "  Country: {}", self.country_name())?;
        writeln!(f, "  Control: {control}")?;
        writeln!(f, "  Founded Colonies: {}", self.founded_colonies)?;
        writeln!(f, "  Diplomacy: {}", self.diplomacy)
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let difficulty = Difficulty::try_from(self.difficulty)
            .map(|v| v.to_string())
            .unwrap_or_else(|v| unknown_u8(v));
        let season = Season::try_from(self.season as u8)
            .map(|v| v.to_string())
            .unwrap_or_else(|v| unknown_u8(v as u8));

        writeln!(f, "Colonization Save File")?;
        writeln!(f, "  Year: {}, {}", self.year, season)?;
        writeln!(f, "  Turn: {}", self.turn)?;
        writeln!(f, "  Difficulty: {}", difficulty)?;
        writeln!(f, "  Map: {} x {}", self.map_size_x, self.map_size_y)?;
        writeln!(f, "  Colonies: {}", self.colony_count)?;
        writeln!(f, "  Units: {}", self.unit_count)?;
        writeln!(f, "  Tribes: {}", self.tribe_count)?;
        writeln!(f, "  Human Player: {}", self.human_player)
    }
}
