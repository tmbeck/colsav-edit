use std::fs;
use std::path::Path;

use crate::error::{SaveError, Result};
use crate::raw::header::{Header, HEADER_SIZE};
use crate::raw::player::{Player, PLAYER_SIZE, PLAYER_COUNT, read_players, write_players};
use crate::raw::colony::{Colony, COLONY_SIZE};
use crate::raw::unit::{Unit, UNIT_SIZE};
use crate::raw::nation::{Nation, NATION_COUNT};
use crate::raw::tribe::{Tribe, TRIBE_SIZE};
use crate::raw::indian::{Indian, INDIAN_COUNT};
use crate::raw::stuff::Stuff;
use crate::raw::maps::{MapLayer, Connectivity};
use crate::raw::trade_route::{TradeRoute, TRADE_ROUTE_COUNT};

/// Size of the "OTHER" section between PLAYER and COLONY.
/// From JSON: unknown51a (18 bytes) + click_before_open_colony x,y (2×2=4 bytes)
///   + unknown51b (2 bytes) = 24 bytes.
/// NOTE: Prior notes said 22 bytes. We determine the true size empirically
/// by storing the raw bytes and verifying round-trip.
const OTHER_SIZE: usize = 24;

/// Number of bytes between CONNECTIVITY and TRADE_ROUTE:
/// unknown_map38c2 (9×2=18) + unknown_map38c3 (16) + strategy (14×2=28)
/// + unknown_map38d (10) + prime_resource_seed (1) + unknown39d (1) = 74
const TAIL_FIXED_SIZE: usize = 74;

/// Top-level save file: all parsed sections plus raw data for round-trip.
#[derive(Debug, Clone)]
pub struct SaveFile {
    pub header: Header,
    pub players: Vec<Player>,
    pub other: Vec<u8>,          // raw OTHER section (24 bytes)
    pub colonies: Vec<Colony>,
    pub units: Vec<Unit>,
    pub nations: Vec<Nation>,
    pub tribes: Vec<Tribe>,
    pub indians: Vec<Indian>,
    pub stuff: Stuff,
    pub tile_map: MapLayer,
    pub mask_map: MapLayer,
    pub path_map: MapLayer,
    pub seen_map: MapLayer,
    pub connectivity: Connectivity,
    pub tail_fixed: Vec<u8>,     // 74 bytes of trailing fields before trade routes
    pub trade_routes: Vec<TradeRoute>,
    pub trailing: Vec<u8>,       // any bytes after trade routes (should be empty)
}

impl SaveFile {
    /// Read and parse a SAV file from disk.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = fs::read(path.as_ref())?;
        Self::from_bytes(&data)
    }

    /// Parse a SAV file from a byte slice.
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let total = data.len();
        let mut pos: usize = 0;

        // ── HEAD (390 bytes) ──
        // The 390-byte header block includes:
        //   bytes 0-157:  parsed header fields (158 bytes)
        //   bytes 158-365: PLAYER data (4 × 52 = 208 bytes)
        //   bytes 366-389: OTHER data (24 bytes)
        // These are stored in header.remaining (232 bytes).
        if total < HEADER_SIZE {
            return Err(SaveError::UnexpectedEof {
                offset: 0,
                needed: HEADER_SIZE,
                available: total,
            });
        }
        let mut header = Header::read(&data[pos..])?;
        pos += HEADER_SIZE;

        let map_x = header.map_size_x as usize;
        let map_y = header.map_size_y as usize;
        let colony_count = header.colony_count as usize;
        let unit_count = header.unit_count as usize;
        let tribe_count = header.tribe_count as usize;

        // ── PLAYER + OTHER (extracted from header.remaining, NOT from file stream) ──
        let player_total = PLAYER_COUNT * PLAYER_SIZE;
        let players = read_players(&header.remaining[..player_total])?;
        let other = header.remaining[player_total..player_total + OTHER_SIZE].to_vec();
        // Clear remaining since we've extracted its contents
        header.remaining = Vec::new();

        // ── COLONY (colony_count × 202 bytes) ──
        let colony_total = colony_count * COLONY_SIZE;
        Self::check_eof(pos, colony_total, total, "COLONY")?;
        let mut colonies = Vec::with_capacity(colony_count);
        for _ in 0..colony_count {
            colonies.push(Colony::read(&data[pos..])?);
            pos += COLONY_SIZE;
        }

        // ── UNIT (unit_count × 28 bytes) ──
        let unit_total = unit_count * UNIT_SIZE;
        Self::check_eof(pos, unit_total, total, "UNIT")?;
        let mut units = Vec::with_capacity(unit_count);
        for _ in 0..unit_count {
            units.push(Unit::read(&data[pos..])?);
            pos += UNIT_SIZE;
        }

        // ── NATION (4 × 316 = 1264 bytes) ──
        let nation_size = Nation::byte_size();
        let nation_total = NATION_COUNT * nation_size;
        Self::check_eof(pos, nation_total, total, "NATION")?;
        let mut nations = Vec::with_capacity(NATION_COUNT);
        for _ in 0..NATION_COUNT {
            nations.push(Nation::read(&data[pos..])?);
            pos += nation_size;
        }

        // ── TRIBE (tribe_count × 18 bytes) ──
        let tribe_total = tribe_count * TRIBE_SIZE;
        Self::check_eof(pos, tribe_total, total, "TRIBE")?;
        let mut tribes = Vec::with_capacity(tribe_count);
        for _ in 0..tribe_count {
            tribes.push(Tribe::read(&data[pos..])?);
            pos += TRIBE_SIZE;
        }

        // ── INDIAN (8 × 78 = 624 bytes) ──
        let indian_size = Indian::byte_size();
        let indian_total = INDIAN_COUNT * indian_size;
        Self::check_eof(pos, indian_total, total, "INDIAN")?;
        let mut indians = Vec::with_capacity(INDIAN_COUNT);
        for _ in 0..INDIAN_COUNT {
            indians.push(Indian::read(&data[pos..])?);
            pos += indian_size;
        }

        // ── STUFF (727 bytes) ──
        let stuff_size = Stuff::byte_size();
        Self::check_eof(pos, stuff_size, total, "STUFF")?;
        let stuff = Stuff::read(&data[pos..])?;
        pos += stuff_size;

        // ── MAP LAYERS (4 × map_y × map_x bytes each) ──
        let map_layer_size = map_y * map_x;
        Self::check_eof(pos, map_layer_size * 4, total, "MAP_LAYERS")?;

        let tile_map = MapLayer::read(&data[pos..], map_y, map_x)?;
        pos += map_layer_size;

        let mask_map = MapLayer::read(&data[pos..], map_y, map_x)?;
        pos += map_layer_size;

        let path_map = MapLayer::read(&data[pos..], map_y, map_x)?;
        pos += map_layer_size;

        let seen_map = MapLayer::read(&data[pos..], map_y, map_x)?;
        pos += map_layer_size;

        // ── CONNECTIVITY (540 bytes) ──
        Self::check_eof(pos, Connectivity::TOTAL_SIZE, total, "CONNECTIVITY")?;
        let connectivity = Connectivity::read(&data[pos..])?;
        pos += Connectivity::TOTAL_SIZE;

        // ── TAIL FIXED (74 bytes: misc trailing fields before trade routes) ──
        Self::check_eof(pos, TAIL_FIXED_SIZE, total, "TAIL_FIXED")?;
        let tail_fixed = data[pos..pos + TAIL_FIXED_SIZE].to_vec();
        pos += TAIL_FIXED_SIZE;

        // ── TRADE_ROUTE (12 × 74 = 888 bytes) ──
        let trade_total = TRADE_ROUTE_COUNT * TradeRoute::SIZE;
        Self::check_eof(pos, trade_total, total, "TRADE_ROUTE")?;
        let mut trade_routes = Vec::with_capacity(TRADE_ROUTE_COUNT);
        for _ in 0..TRADE_ROUTE_COUNT {
            trade_routes.push(TradeRoute::read(&data[pos..])?);
            pos += TradeRoute::SIZE;
        }

        // ── TRAILING (anything left) ──
        let trailing = if pos < total {
            data[pos..].to_vec()
        } else {
            Vec::new()
        };

        Ok(SaveFile {
            header,
            players,
            other,
            colonies,
            units,
            nations,
            tribes,
            indians,
            stuff,
            tile_map,
            mask_map,
            path_map,
            seen_map,
            connectivity,
            tail_fixed,
            trade_routes,
            trailing,
        })
    }

    /// Serialize the entire save file back to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let map_x = self.header.map_size_x as usize;
        let map_y = self.header.map_size_y as usize;
        let map_layer_size = map_y * map_x;

        // Estimate total size for efficient allocation
        let estimated = HEADER_SIZE
            + self.colonies.len() * COLONY_SIZE
            + self.units.len() * UNIT_SIZE
            + NATION_COUNT * Nation::byte_size()
            + self.tribes.len() * TRIBE_SIZE
            + INDIAN_COUNT * Indian::byte_size()
            + Stuff::byte_size()
            + map_layer_size * 4
            + Connectivity::TOTAL_SIZE
            + TAIL_FIXED_SIZE
            + TRADE_ROUTE_COUNT * TradeRoute::SIZE
            + self.trailing.len();

        let mut buf = Vec::with_capacity(estimated);

        // Reconstruct header.remaining from players + other before writing
        let mut header = self.header.clone();
        let mut remaining = Vec::with_capacity(PLAYER_COUNT * PLAYER_SIZE + OTHER_SIZE);
        remaining.extend_from_slice(&write_players(&self.players));
        remaining.extend_from_slice(&self.other);
        header.remaining = remaining;

        buf.extend_from_slice(&header.write());

        for colony in &self.colonies {
            buf.extend_from_slice(&colony.write());
        }
        for unit in &self.units {
            buf.extend_from_slice(&unit.write());
        }
        for nation in &self.nations {
            buf.extend_from_slice(&nation.write());
        }
        for tribe in &self.tribes {
            buf.extend_from_slice(&tribe.write());
        }
        for indian in &self.indians {
            buf.extend_from_slice(&indian.write());
        }

        buf.extend_from_slice(&self.stuff.write());

        buf.extend_from_slice(&self.tile_map.write());
        buf.extend_from_slice(&self.mask_map.write());
        buf.extend_from_slice(&self.path_map.write());
        buf.extend_from_slice(&self.seen_map.write());

        buf.extend_from_slice(&self.connectivity.write());
        buf.extend_from_slice(&self.tail_fixed);

        for tr in &self.trade_routes {
            buf.extend_from_slice(&tr.write());
        }

        buf.extend_from_slice(&self.trailing);

        buf
    }

    /// Write the save file to disk.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::write(path.as_ref(), self.to_bytes())?;
        Ok(())
    }

    /// Helper: check there are enough bytes remaining.
    fn check_eof(pos: usize, needed: usize, total: usize, _section: &'static str) -> Result<()> {
        if pos + needed > total {
            Err(SaveError::UnexpectedEof {
                offset: pos,
                needed,
                available: total.saturating_sub(pos),
            })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip test: read a real SAV file, write it back, compare bytes.
    /// This test requires a SAV file at the expected path.
    #[test]
    fn round_trip_colony01() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../saves/COLONY01.SAV");
        if !Path::new(path).exists() {
            eprintln!("Skipping round_trip test: {} not found", path);
            return;
        }

        let original = fs::read(path).expect("read original");
        let save = SaveFile::from_bytes(&original).expect("parse");
        let written = save.to_bytes();

        assert_eq!(
            original.len(),
            written.len(),
            "Length mismatch: original={}, written={}",
            original.len(),
            written.len()
        );

        // Find first byte difference for debugging
        for (i, (a, b)) in original.iter().zip(written.iter()).enumerate() {
            if a != b {
                panic!(
                    "Byte mismatch at offset 0x{:04X} ({}): original=0x{:02X}, written=0x{:02X}",
                    i, i, a, b
                );
            }
        }
    }

    /// Round-trip all available SAV files.
    #[test]
    fn round_trip_all_saves() {
        let saves_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../saves");
        let dir = Path::new(saves_dir);
        if !dir.exists() {
            eprintln!("Skipping round_trip_all_saves: {} not found", saves_dir);
            return;
        }

        let mut count = 0;
        for entry in fs::read_dir(dir).expect("read saves dir") {
            let entry = entry.expect("dir entry");
            let path = entry.path();
            if path.extension().map(|e| e == "SAV").unwrap_or(false) {
                let original = fs::read(&path).expect("read file");
                let save = match SaveFile::from_bytes(&original) {
                    Ok(s) => s,
                    Err(e) => {
                        panic!("Failed to parse {}: {}", path.display(), e);
                    }
                };
                let written = save.to_bytes();

                assert_eq!(
                    original.len(),
                    written.len(),
                    "{}: length mismatch: original={}, written={}",
                    path.display(),
                    original.len(),
                    written.len()
                );

                for (i, (a, b)) in original.iter().zip(written.iter()).enumerate() {
                    if a != b {
                        panic!(
                            "{}: byte mismatch at offset 0x{:04X}: original=0x{:02X}, written=0x{:02X}",
                            path.display(), i, a, b
                        );
                    }
                }
                count += 1;
                eprintln!("  ✓ {} ({} bytes)", path.display(), original.len());
            }
        }
        eprintln!("Round-trip verified {} SAV files", count);
    }
}
