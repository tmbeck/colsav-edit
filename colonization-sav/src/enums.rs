use std::fmt;

/// Helper macro: define a u8 enum with Display, TryFrom<u8>, and Into<u8>.
macro_rules! sav_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            $( $(#[$vmeta:meta])* $Variant:ident = $val:expr ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u8)]
        $vis enum $Name {
            $( $(#[$vmeta])* $Variant = $val ),+
        }

        impl TryFrom<u8> for $Name {
            type Error = u8;
            fn try_from(v: u8) -> Result<Self, u8> {
                match v {
                    $( $val => Ok(Self::$Variant), )+
                    other => Err(other),
                }
            }
        }

        impl From<$Name> for u8 {
            fn from(v: $Name) -> u8 {
                v as u8
            }
        }

        impl fmt::Display for $Name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // Use Debug name, but replace underscores with spaces
                let s = format!("{:?}", self);
                f.write_str(&s)
            }
        }
    };
}

sav_enum! {
    pub enum Difficulty {
        Discoverer = 0,
        Explorer = 1,
        Conquistador = 2,
        Governor = 3,
        Viceroy = 4,
    }
}

sav_enum! {
    pub enum ControlType {
        Player = 0,
        Ai = 1,
        Withdrawn = 2,
    }
}

sav_enum! {
    pub enum TechType {
        SemiNomadic = 0,
        Agrarian = 1,
        Advanced = 2,
        Civilized = 3,
    }
}

sav_enum! {
    pub enum UnitType {
        Colonist = 0x00,
        Soldier = 0x01,
        Pioneer = 0x02,
        Missionary = 0x03,
        Dragoon = 0x04,
        Scout = 0x05,
        ToryRegular = 0x06,
        ContinentalCavalry = 0x07,
        ToryCavalry = 0x08,
        ContinentalArmy = 0x09,
        Treasure = 0x0A,
        Artillery = 0x0B,
        WagonTrain = 0x0C,
        Caravel = 0x0D,
        Merchantman = 0x0E,
        Galleon = 0x0F,
        Privateer = 0x10,
        Frigate = 0x11,
        ManOWar = 0x12,
        Brave = 0x13,
        ArmedBrave = 0x14,
        MountedBrave = 0x15,
        MountedWarrior = 0x16,
    }
}

sav_enum! {
    pub enum OccupationType {
        Farmer = 0x00,
        SugarPlanter = 0x01,
        TobaccoPlanter = 0x02,
        CottonPlanter = 0x03,
        FurTrapper = 0x04,
        Lumberjack = 0x05,
        OreMiner = 0x06,
        SilverMiner = 0x07,
        Fisherman = 0x08,
        Distiller = 0x09,
        Tobacconist = 0x0A,
        Weaver = 0x0B,
        FurTrader = 0x0C,
        Carpenter = 0x0D,
        Blacksmith = 0x0E,
        Gunsmith = 0x0F,
        Preacher = 0x10,
        Statesman = 0x11,
        Teacher = 0x12,
        Unknown13 = 0x13,
    }
}

sav_enum! {
    pub enum ProfessionType {
        ExpertFarmer = 0x00,
        MasterSugarPlanter = 0x01,
        MasterTobaccoPlanter = 0x02,
        MasterCottonPlanter = 0x03,
        ExpertFurTrapper = 0x04,
        ExpertLumberjack = 0x05,
        ExpertOreMiner = 0x06,
        ExpertSilverMiner = 0x07,
        ExpertFisherman = 0x08,
        MasterDistiller = 0x09,
        MasterTobacconist = 0x0A,
        MasterWeaver = 0x0B,
        MasterFurTrader = 0x0C,
        MasterCarpenter = 0x0D,
        MasterBlacksmith = 0x0E,
        MasterGunsmith = 0x0F,
        FirebrandPreacher = 0x10,
        ElderStatesman = 0x11,
        Student = 0x12,
        FreeColonist = 0x13,
        HardyPioneer = 0x14,
        VeteranSoldier = 0x15,
        SeasonedScout = 0x16,
        VeteranDragoon = 0x17,
        JesuitMissionary = 0x18,
        IndenturedServant = 0x19,
        PettyCriminal = 0x1A,
        IndianConvert = 0x1B,
        FreeColonistAlt = 0x1C,
    }
}

sav_enum! {
    /// Nation IDs (byte-sized). European powers 0-3, Indian nations 4-11, None = 0xFF.
    pub enum NationType {
        England = 0x00,
        France = 0x01,
        Spain = 0x02,
        Netherlands = 0x03,
        Inca = 0x04,
        Aztec = 0x05,
        Arawak = 0x06,
        Iroquois = 0x07,
        Cherokee = 0x08,
        Apache = 0x09,
        Sioux = 0x0A,
        Tupi = 0x0B,
        // None = 0xFF handled specially
    }
}

/// Nation type that also supports 0xFF = None.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NationId {
    Nation(NationType),
    None,
}

impl NationId {
    pub fn from_u8(v: u8) -> Self {
        if v == 0xFF {
            NationId::None
        } else {
            match NationType::try_from(v) {
                Ok(n) => NationId::Nation(n),
                Err(_) => NationId::None,
            }
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            NationId::Nation(n) => n as u8,
            NationId::None => 0xFF,
        }
    }

    pub fn from_u16_le(data: &[u8]) -> Self {
        Self::from_u8(data[0])
    }

    pub fn to_u16_le(self) -> [u8; 2] {
        match self {
            NationId::Nation(n) => [n as u8, 0x00],
            NationId::None => [0xFF, 0xFF],
        }
    }
}

impl fmt::Display for NationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NationId::Nation(n) => write!(f, "{n}"),
            NationId::None => write!(f, "None"),
        }
    }
}

sav_enum! {
    pub enum CargoType {
        Food = 0,
        Sugar = 1,
        Tobacco = 2,
        Cotton = 3,
        Furs = 4,
        Lumber = 5,
        Ore = 6,
        Silver = 7,
        Horses = 8,
        Rum = 9,
        Cigars = 10,
        Cloth = 11,
        Coats = 12,
        TradeGoods = 13,
        Tools = 14,
        Muskets = 15,
    }
}

sav_enum! {
    pub enum OrdersType {
        None = 0x00,
        Sentry = 0x01,
        Trading = 0x02,
        Goto = 0x03,
        Fortify = 0x05,
        Fortified = 0x06,
        Plow = 0x08,
        Road = 0x09,
        UnknownA = 0x0A,
        UnknownB = 0x0B,
        UnknownC = 0x0C,
    }
}

sav_enum! {
    pub enum FortificationLevel {
        None = 0,
        Stockade = 1,
        Fort = 2,
        Fortress = 3,
    }
}

sav_enum! {
    pub enum TradeRouteType {
        Land = 0,
        Sea = 1,
    }
}

sav_enum! {
    pub enum Season {
        Spring = 0,
        Autumn = 1,
    }
}

/// Terrain type (5-bit encoding from TILE section).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TerrainType {
    Tundra = 0b00000,
    Desert = 0b00001,
    Plains = 0b00010,
    Prairie = 0b00011,
    Grassland = 0b00100,
    Savannah = 0b00101,
    Marsh = 0b00110,
    Swamp = 0b00111,
    TundraForest = 0b01000,
    DesertForest = 0b01001,
    PlainsForest = 0b01010,
    PrairieForest = 0b01011,
    GrasslandForest = 0b01100,
    SavannahForest = 0b01101,
    MarshForest = 0b01110,
    SwampForest = 0b01111,
    /// Deprecated forest variant (same as regular forest).
    TundraForestW = 0b10000,
    DesertForestW = 0b10001,
    PlainsForestW = 0b10010,
    PrairieForestW = 0b10011,
    GrasslandForestW = 0b10100,
    SavannahForestW = 0b10101,
    MarshForestW = 0b10110,
    SwampForestW = 0b10111,
    Arctic = 0b11000,
    Ocean = 0b11001,
    SeaLane = 0b11010,
}

impl TryFrom<u8> for TerrainType {
    type Error = u8;
    fn try_from(v: u8) -> Result<Self, u8> {
        match v {
            0b00000 => Ok(Self::Tundra),
            0b00001 => Ok(Self::Desert),
            0b00010 => Ok(Self::Plains),
            0b00011 => Ok(Self::Prairie),
            0b00100 => Ok(Self::Grassland),
            0b00101 => Ok(Self::Savannah),
            0b00110 => Ok(Self::Marsh),
            0b00111 => Ok(Self::Swamp),
            0b01000 => Ok(Self::TundraForest),
            0b01001 => Ok(Self::DesertForest),
            0b01010 => Ok(Self::PlainsForest),
            0b01011 => Ok(Self::PrairieForest),
            0b01100 => Ok(Self::GrasslandForest),
            0b01101 => Ok(Self::SavannahForest),
            0b01110 => Ok(Self::MarshForest),
            0b01111 => Ok(Self::SwampForest),
            0b10000 => Ok(Self::TundraForestW),
            0b10001 => Ok(Self::DesertForestW),
            0b10010 => Ok(Self::PlainsForestW),
            0b10011 => Ok(Self::PrairieForestW),
            0b10100 => Ok(Self::GrasslandForestW),
            0b10101 => Ok(Self::SavannahForestW),
            0b10110 => Ok(Self::MarshForestW),
            0b10111 => Ok(Self::SwampForestW),
            0b11000 => Ok(Self::Arctic),
            0b11001 => Ok(Self::Ocean),
            0b11010 => Ok(Self::SeaLane),
            other => Err(other),
        }
    }
}

impl From<TerrainType> for u8 {
    fn from(v: TerrainType) -> u8 {
        v as u8
    }
}

impl fmt::Display for TerrainType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Hills/river modifier (3-bit encoding from TILE section).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HillsRiver {
    Nothing = 0b000,
    Hills = 0b001,
    River = 0b010,
    RiverHills = 0b011,
    Unknown = 0b100,
    Mountains = 0b101,
    MajorRiver = 0b110,
}

impl TryFrom<u8> for HillsRiver {
    type Error = u8;
    fn try_from(v: u8) -> Result<Self, u8> {
        match v {
            0b000 => Ok(Self::Nothing),
            0b001 => Ok(Self::Hills),
            0b010 => Ok(Self::River),
            0b011 => Ok(Self::RiverHills),
            0b100 => Ok(Self::Unknown),
            0b101 => Ok(Self::Mountains),
            0b110 => Ok(Self::MajorRiver),
            other => Err(other),
        }
    }
}

impl From<HillsRiver> for u8 {
    fn from(v: HillsRiver) -> u8 {
        v as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_type_try_from_valid() {
        let variants = [
            UnitType::Colonist,
            UnitType::Soldier,
            UnitType::Pioneer,
            UnitType::Missionary,
            UnitType::Dragoon,
            UnitType::Scout,
            UnitType::ToryRegular,
            UnitType::ContinentalCavalry,
            UnitType::ToryCavalry,
            UnitType::ContinentalArmy,
            UnitType::Treasure,
            UnitType::Artillery,
            UnitType::WagonTrain,
            UnitType::Caravel,
            UnitType::Merchantman,
            UnitType::Galleon,
            UnitType::Privateer,
            UnitType::Frigate,
            UnitType::ManOWar,
            UnitType::Brave,
            UnitType::ArmedBrave,
            UnitType::MountedBrave,
            UnitType::MountedWarrior,
        ];

        for variant in variants {
            let value = u8::from(variant);
            assert_eq!(UnitType::try_from(value), Ok(variant));
        }
    }

    #[test]
    fn test_unit_type_try_from_invalid() {
        assert_eq!(UnitType::try_from(0xFF), Err(0xFF));
    }

    #[test]
    fn test_nation_id_none() {
        assert_eq!(NationId::from_u8(0xFF), NationId::None);
    }

    #[test]
    fn test_nation_id_round_trip_u16() {
        let ids = [
            NationId::Nation(NationType::England),
            NationId::Nation(NationType::France),
            NationId::Nation(NationType::Spain),
            NationId::Nation(NationType::Netherlands),
        ];

        for id in ids {
            let bytes = id.to_u16_le();
            let parsed = NationId::from_u16_le(&bytes);
            assert_eq!(parsed, id);
        }
    }

    #[test]
    fn test_terrain_type_try_from_all_valid() {
        let variants = [
            TerrainType::Tundra,
            TerrainType::Desert,
            TerrainType::Plains,
            TerrainType::Prairie,
            TerrainType::Grassland,
            TerrainType::Savannah,
            TerrainType::Marsh,
            TerrainType::Swamp,
            TerrainType::TundraForest,
            TerrainType::DesertForest,
            TerrainType::PlainsForest,
            TerrainType::PrairieForest,
            TerrainType::GrasslandForest,
            TerrainType::SavannahForest,
            TerrainType::MarshForest,
            TerrainType::SwampForest,
            TerrainType::TundraForestW,
            TerrainType::DesertForestW,
            TerrainType::PlainsForestW,
            TerrainType::PrairieForestW,
            TerrainType::GrasslandForestW,
            TerrainType::SavannahForestW,
            TerrainType::MarshForestW,
            TerrainType::SwampForestW,
            TerrainType::Arctic,
            TerrainType::Ocean,
            TerrainType::SeaLane,
        ];

        for variant in variants {
            let value = u8::from(variant);
            assert_eq!(TerrainType::try_from(value), Ok(variant));
        }
    }

    #[test]
    fn test_hills_river_round_trip() {
        let variants = [
            HillsRiver::Nothing,
            HillsRiver::Hills,
            HillsRiver::River,
            HillsRiver::RiverHills,
            HillsRiver::Unknown,
            HillsRiver::Mountains,
            HillsRiver::MajorRiver,
        ];

        for variant in variants {
            let value = u8::from(variant);
            assert_eq!(HillsRiver::try_from(value), Ok(variant));
        }
    }
}
