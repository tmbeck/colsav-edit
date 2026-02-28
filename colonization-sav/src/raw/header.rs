use crate::bits::{BitReader, BitWriter};
use crate::enums::*;
use crate::goods::Goods16;
use crate::error::{SaveError, Result};

/// SAV file header. Fixed size = 390 bytes.
/// Starts with "COLONIZE\0" magic marker.
pub const HEADER_SIZE: usize = 390;
const MAGIC: &[u8; 9] = b"COLONIZE\0";

#[derive(Debug, Clone)]
pub struct Header {
    // Offsets 0-8: magic "COLONIZE\0"
    pub unknown00: [u8; 3],          // 9-11
    pub map_size_x: u16,             // 12-13
    pub map_size_y: u16,             // 14-15
    pub tut1: u8,                    // 16
    pub unknown03: u8,               // 17
    pub game_options: GameOptions,   // 18-19 (16-bit bit_struct)
    pub colony_report_options: ColonyReportOptions, // 20-21 (16-bit bit_struct)
    pub tut2: u8,                    // 22
    pub tut3: u8,                    // 23
    pub unknown39: [u8; 2],          // 24-25
    pub year: u16,                   // 26-27
    pub season: u16,                 // 28-29 (0=spring, 1=autumn)
    pub turn: u16,                   // 30-31
    pub tile_selection_mode: u8,     // 32
    pub unknown40: u8,               // 33
    pub active_unit: i16,            // 34-35
    pub nation_turn: NationId,       // 36-37 (2-byte)
    pub curr_nation_map_view: NationId, // 38-39
    pub human_player: NationId,      // 40-41
    pub tribe_count: u16,            // 42-43
    pub unit_count: u16,             // 44-45
    pub colony_count: u16,           // 46-47
    pub trade_route_count: u16,      // 48-49 (unused per docs)
    pub show_entire_map: u16,        // 50-51
    pub fixed_nation_map_view: NationId, // 52-53
    pub difficulty: u8,              // 54
    pub unknown43a: u8,              // 55
    pub unknown43b: u8,              // 56
    pub founding_father: [u8; 25],   // 57-81
    pub unknown44aa: [u8; 2],        // 82-83
    pub manual_save_flag: u8,        // 84
    pub unknown44ab: u8,             // 85
    pub end_of_turn_sign: u16,       // 86-87
    pub nation_relation: [u8; 8],    // 88-95
    pub rebel_sentiment_report: i16, // 96-97
    pub unknown45a: [u8; 8],         // 98-105
    pub expeditionary_force: [u16; 4], // 106-113 (regulars, dragoons, man-o-wars, artillery)
    pub backup_force: [u16; 4],      // 114-121
    pub price_group_state: Goods16<u16>, // 122-153
    pub events: EventFlags,          // 154-155 (16-bit bit_struct)
    pub unknown05: [u8; 2],          // 156-157

    // ---- COMPUTED (not stored, derived from above) ----
    // Total: 9+3+2+2+1+1+2+2+1+1+2+2+2+2+1+1+2+2+2+2+2+2+2+2+2+2+1+1+1+25+2+1+1+2+8+2+8+8+8+32+2+2
    // = 158 bytes? Let me recount...
    // Actually the header is 390 bytes total per JSON. We need to store remaining unknown bytes.
    pub remaining: Vec<u8>,          // bytes 158..390 (232 bytes of trailing data)
}

/// Game options (16-bit bit_struct, big-endian bit order).
#[derive(Debug, Clone, Copy, Default)]
pub struct GameOptions {
    pub tutorial_hints: bool,
    pub water_color_cycling: bool,
    pub combat_analysis: bool,
    pub autosave: bool,
    pub end_of_turn: bool,
    pub fast_piece_slide: bool,
    pub cheats_enabled: bool,
    pub show_foreign_moves: bool,
    pub unused01: u8,             // 7 bits (preserved for round-trip)
    pub show_indian_moves: bool,
}

impl GameOptions {
    pub fn read(data: &[u8]) -> Self {
        let mut r = BitReader::new(&data[..2]);
        let unused01 = r.read_u8(7);
        Self {
            unused01,
            tutorial_hints: r.read_bool(),
            water_color_cycling: r.read_bool(),
            combat_analysis: r.read_bool(),
            autosave: r.read_bool(),
            end_of_turn: r.read_bool(),
            fast_piece_slide: r.read_bool(),
            cheats_enabled: r.read_bool(),
            show_foreign_moves: r.read_bool(),
            show_indian_moves: r.read_bool(),
        }
    }

    pub fn write(&self, buf: &mut [u8]) {
        let mut w = BitWriter::new(&mut buf[..2]);
        w.write_u8(7, self.unused01);
        w.write_bool(self.tutorial_hints);
        w.write_bool(self.water_color_cycling);
        w.write_bool(self.combat_analysis);
        w.write_bool(self.autosave);
        w.write_bool(self.end_of_turn);
        w.write_bool(self.fast_piece_slide);
        w.write_bool(self.cheats_enabled);
        w.write_bool(self.show_foreign_moves);
        w.write_bool(self.show_indian_moves);
    }
}

/// Colony report options (16-bit bit_struct).
/// Note: true means the option is DISABLED.
#[derive(Debug, Clone, Copy, Default)]
pub struct ColonyReportOptions {
    pub labels_on_cargo_and_terrain: bool,
    pub labels_on_buildings: bool,
    pub report_new_cargos_available: bool,
    pub report_inefficient_government: bool,
    pub report_tools_needed_for_production: bool,
    pub report_raw_materials_shortages: bool,
    pub report_food_shortages: bool,
    pub report_when_colonists_trained: bool,
    pub report_sons_of_liberty_membership: bool,
    pub report_rebel_majorities: bool,
    pub unused_bits: u8,                   // 6 bits (preserved for round-trip)
}

impl ColonyReportOptions {
    pub fn read(data: &[u8]) -> Self {
        let mut r = BitReader::new(&data[..2]);
        Self {
            labels_on_cargo_and_terrain: r.read_bool(),
            labels_on_buildings: r.read_bool(),
            report_new_cargos_available: r.read_bool(),
            report_inefficient_government: r.read_bool(),
            report_tools_needed_for_production: r.read_bool(),
            report_raw_materials_shortages: r.read_bool(),
            report_food_shortages: r.read_bool(),
            report_when_colonists_trained: r.read_bool(),
            report_sons_of_liberty_membership: r.read_bool(),
            report_rebel_majorities: r.read_bool(),
            unused_bits: r.read_u8(6),
        }
    }

    pub fn write(&self, buf: &mut [u8]) {
        let mut w = BitWriter::new(&mut buf[..2]);
        w.write_bool(self.labels_on_cargo_and_terrain);
        w.write_bool(self.labels_on_buildings);
        w.write_bool(self.report_new_cargos_available);
        w.write_bool(self.report_inefficient_government);
        w.write_bool(self.report_tools_needed_for_production);
        w.write_bool(self.report_raw_materials_shortages);
        w.write_bool(self.report_food_shortages);
        w.write_bool(self.report_when_colonists_trained);
        w.write_bool(self.report_sons_of_liberty_membership);
        w.write_bool(self.report_rebel_majorities);
        w.write_u8(6, self.unused_bits);
    }
}

/// Event flags (16-bit bit_struct).
#[derive(Debug, Clone, Copy, Default)]
pub struct EventFlags {
    pub discovery_of_the_new_world: bool,
    pub building_a_colony: bool,
    pub meeting_the_natives: bool,
    pub the_aztec_empire: bool,
    pub the_inca_nation: bool,
    pub discovery_of_the_pacific_ocean: bool,
    pub entering_indian_village: bool,
    pub the_fountain_of_youth: bool,
    pub cargo_from_the_new_world: bool,
    pub meeting_fellow_europeans: bool,
    pub colony_burning: bool,
    pub colony_destroyed: bool,
    pub indian_raid: bool,
    pub woodcut14: bool,
    pub woodcut15: bool,
    pub woodcut16: bool,
}

impl EventFlags {
    pub fn read(data: &[u8]) -> Self {
        let mut r = BitReader::new(&data[..2]);
        Self {
            discovery_of_the_new_world: r.read_bool(),
            building_a_colony: r.read_bool(),
            meeting_the_natives: r.read_bool(),
            the_aztec_empire: r.read_bool(),
            the_inca_nation: r.read_bool(),
            discovery_of_the_pacific_ocean: r.read_bool(),
            entering_indian_village: r.read_bool(),
            the_fountain_of_youth: r.read_bool(),
            cargo_from_the_new_world: r.read_bool(),
            meeting_fellow_europeans: r.read_bool(),
            colony_burning: r.read_bool(),
            colony_destroyed: r.read_bool(),
            indian_raid: r.read_bool(),
            woodcut14: r.read_bool(),
            woodcut15: r.read_bool(),
            woodcut16: r.read_bool(),
        }
    }

    pub fn write(&self, buf: &mut [u8]) {
        let mut w = BitWriter::new(&mut buf[..2]);
        w.write_bool(self.discovery_of_the_new_world);
        w.write_bool(self.building_a_colony);
        w.write_bool(self.meeting_the_natives);
        w.write_bool(self.the_aztec_empire);
        w.write_bool(self.the_inca_nation);
        w.write_bool(self.discovery_of_the_pacific_ocean);
        w.write_bool(self.entering_indian_village);
        w.write_bool(self.the_fountain_of_youth);
        w.write_bool(self.cargo_from_the_new_world);
        w.write_bool(self.meeting_fellow_europeans);
        w.write_bool(self.colony_burning);
        w.write_bool(self.colony_destroyed);
        w.write_bool(self.indian_raid);
        w.write_bool(self.woodcut14);
        w.write_bool(self.woodcut15);
        w.write_bool(self.woodcut16);
    }
}

impl Header {
    /// Parse header from a byte slice. Must be at least HEADER_SIZE bytes.
    pub fn read(data: &[u8]) -> Result<Self> {
        if data.len() < HEADER_SIZE {
            return Err(SaveError::UnexpectedEof {
                offset: 0,
                needed: HEADER_SIZE,
                available: data.len(),
            });
        }

        if &data[0..9] != MAGIC {
            return Err(SaveError::InvalidMagic);
        }

        let mut pos = 9;

        let mut unknown00 = [0u8; 3];
        unknown00.copy_from_slice(&data[pos..pos + 3]);
        pos += 3;

        let map_size_x = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let map_size_y = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let tut1 = data[pos]; pos += 1;
        let unknown03 = data[pos]; pos += 1;

        let game_options = GameOptions::read(&data[pos..]);
        pos += 2;

        let colony_report_options = ColonyReportOptions::read(&data[pos..]);
        pos += 2;

        let tut2 = data[pos]; pos += 1;
        let tut3 = data[pos]; pos += 1;

        let mut unknown39 = [0u8; 2];
        unknown39.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;

        let year = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let season = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let turn = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let tile_selection_mode = data[pos]; pos += 1;
        let unknown40 = data[pos]; pos += 1;

        let active_unit = i16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let nation_turn = NationId::from_u16_le(&data[pos..]);
        pos += 2;
        let curr_nation_map_view = NationId::from_u16_le(&data[pos..]);
        pos += 2;
        let human_player = NationId::from_u16_le(&data[pos..]);
        pos += 2;

        let tribe_count = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let unit_count = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let colony_count = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let trade_route_count = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let show_entire_map = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;
        let fixed_nation_map_view = NationId::from_u16_le(&data[pos..]);
        pos += 2;

        let difficulty = data[pos]; pos += 1;
        let unknown43a = data[pos]; pos += 1;
        let unknown43b = data[pos]; pos += 1;

        let mut founding_father = [0u8; 25];
        founding_father.copy_from_slice(&data[pos..pos + 25]);
        pos += 25;

        let mut unknown44aa = [0u8; 2];
        unknown44aa.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;

        let manual_save_flag = data[pos]; pos += 1;
        let unknown44ab = data[pos]; pos += 1;

        let end_of_turn_sign = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let mut nation_relation = [0u8; 8];
        nation_relation.copy_from_slice(&data[pos..pos + 8]);
        pos += 8;

        let rebel_sentiment_report = i16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        let mut unknown45a = [0u8; 8];
        unknown45a.copy_from_slice(&data[pos..pos + 8]);
        pos += 8;

        let mut expeditionary_force = [0u16; 4];
        for ef in &mut expeditionary_force {
            *ef = u16::from_le_bytes([data[pos], data[pos + 1]]);
            pos += 2;
        }

        let mut backup_force = [0u16; 4];
        for bf in &mut backup_force {
            *bf = u16::from_le_bytes([data[pos], data[pos + 1]]);
            pos += 2;
        }

        let price_group_state = Goods16::<u16>::read_le(&data[pos..]);
        pos += 32;

        let events = EventFlags::read(&data[pos..]);
        pos += 2;

        let mut unknown05 = [0u8; 2];
        unknown05.copy_from_slice(&data[pos..pos + 2]);
        pos += 2;

        // Remaining bytes up to HEADER_SIZE
        let remaining = data[pos..HEADER_SIZE].to_vec();

        Ok(Header {
            unknown00,
            map_size_x,
            map_size_y,
            tut1,
            unknown03,
            game_options,
            colony_report_options,
            tut2,
            tut3,
            unknown39,
            year,
            season,
            turn,
            tile_selection_mode,
            unknown40,
            active_unit,
            nation_turn,
            curr_nation_map_view,
            human_player,
            tribe_count,
            unit_count,
            colony_count,
            trade_route_count,
            show_entire_map,
            fixed_nation_map_view,
            difficulty,
            unknown43a,
            unknown43b,
            founding_father,
            unknown44aa,
            manual_save_flag,
            unknown44ab,
            end_of_turn_sign,
            nation_relation,
            rebel_sentiment_report,
            unknown45a,
            expeditionary_force,
            backup_force,
            price_group_state,
            events,
            unknown05,
            remaining,
        })
    }

    /// Serialize header back to bytes.
    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; HEADER_SIZE];
        let mut pos = 0;

        buf[pos..pos + 9].copy_from_slice(MAGIC);
        pos += 9;

        buf[pos..pos + 3].copy_from_slice(&self.unknown00);
        pos += 3;

        buf[pos..pos + 2].copy_from_slice(&self.map_size_x.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.map_size_y.to_le_bytes());
        pos += 2;

        buf[pos] = self.tut1; pos += 1;
        buf[pos] = self.unknown03; pos += 1;

        self.game_options.write(&mut buf[pos..]);
        pos += 2;
        self.colony_report_options.write(&mut buf[pos..]);
        pos += 2;

        buf[pos] = self.tut2; pos += 1;
        buf[pos] = self.tut3; pos += 1;
        buf[pos..pos + 2].copy_from_slice(&self.unknown39);
        pos += 2;

        buf[pos..pos + 2].copy_from_slice(&self.year.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.season.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.turn.to_le_bytes());
        pos += 2;

        buf[pos] = self.tile_selection_mode; pos += 1;
        buf[pos] = self.unknown40; pos += 1;

        buf[pos..pos + 2].copy_from_slice(&self.active_unit.to_le_bytes());
        pos += 2;

        buf[pos..pos + 2].copy_from_slice(&self.nation_turn.to_u16_le());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.curr_nation_map_view.to_u16_le());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.human_player.to_u16_le());
        pos += 2;

        buf[pos..pos + 2].copy_from_slice(&self.tribe_count.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.unit_count.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.colony_count.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.trade_route_count.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.show_entire_map.to_le_bytes());
        pos += 2;
        buf[pos..pos + 2].copy_from_slice(&self.fixed_nation_map_view.to_u16_le());
        pos += 2;

        buf[pos] = self.difficulty; pos += 1;
        buf[pos] = self.unknown43a; pos += 1;
        buf[pos] = self.unknown43b; pos += 1;

        buf[pos..pos + 25].copy_from_slice(&self.founding_father);
        pos += 25;
        buf[pos..pos + 2].copy_from_slice(&self.unknown44aa);
        pos += 2;
        buf[pos] = self.manual_save_flag; pos += 1;
        buf[pos] = self.unknown44ab; pos += 1;
        buf[pos..pos + 2].copy_from_slice(&self.end_of_turn_sign.to_le_bytes());
        pos += 2;
        buf[pos..pos + 8].copy_from_slice(&self.nation_relation);
        pos += 8;
        buf[pos..pos + 2].copy_from_slice(&self.rebel_sentiment_report.to_le_bytes());
        pos += 2;
        buf[pos..pos + 8].copy_from_slice(&self.unknown45a);
        pos += 8;

        for ef in &self.expeditionary_force {
            buf[pos..pos + 2].copy_from_slice(&ef.to_le_bytes());
            pos += 2;
        }
        for bf in &self.backup_force {
            buf[pos..pos + 2].copy_from_slice(&bf.to_le_bytes());
            pos += 2;
        }

        self.price_group_state.write_le(&mut buf[pos..]);
        pos += 32;

        self.events.write(&mut buf[pos..]);
        pos += 2;

        buf[pos..pos + 2].copy_from_slice(&self.unknown05);
        pos += 2;

        // Copy remaining bytes
        let rem_len = self.remaining.len().min(HEADER_SIZE - pos);
        buf[pos..pos + rem_len].copy_from_slice(&self.remaining[..rem_len]);

        buf
    }
}
