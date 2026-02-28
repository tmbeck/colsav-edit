use crate::error::Result;

/// PLAYER section. 4 entries, each 52 bytes.
/// (name: 24, country_name: 24, player_flags: 1, control: 1, founded_colonies: 1, diplomacy: 1)
pub const PLAYER_SIZE: usize = 52;
pub const PLAYER_COUNT: usize = 4;

#[derive(Debug, Clone)]
pub struct Player {
    pub name_raw: [u8; 24],         // 24 bytes, raw (may contain embedded nulls)
    pub country_name_raw: [u8; 24], // 24 bytes, raw
    pub named_new_world: bool,      // bit 0 of player_flags (1-byte bit_struct)
    pub player_flags_raw: u8,       // preserve raw byte for unknowns
    pub control: u8,                // 0=player, 1=AI, 2=withdrawn
    pub founded_colonies: u8,
    pub diplomacy: u8,
}

impl Player {
    /// Get the player name as a string (up to first null).
    pub fn name(&self) -> &str {
        let end = self.name_raw.iter().position(|&b| b == 0).unwrap_or(24);
        std::str::from_utf8(&self.name_raw[..end]).unwrap_or("")
    }

    /// Get the country name as a string (up to first null).
    pub fn country_name(&self) -> &str {
        let end = self.country_name_raw.iter().position(|&b| b == 0).unwrap_or(24);
        std::str::from_utf8(&self.country_name_raw[..end]).unwrap_or("")
    }

    pub fn read(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        let mut name_raw = [0u8; 24];
        name_raw.copy_from_slice(&data[pos..pos + 24]);
        pos += 24;

        let mut country_name_raw = [0u8; 24];
        country_name_raw.copy_from_slice(&data[pos..pos + 24]);
        pos += 24;

        let player_flags_raw = data[pos];
        let named_new_world = (player_flags_raw & 0x01) != 0;
        pos += 1;

        let control = data[pos]; pos += 1;
        let founded_colonies = data[pos]; pos += 1;
        let diplomacy = data[pos];

        Ok(Player {
            name_raw,
            country_name_raw,
            named_new_world,
            player_flags_raw,
            control,
            founded_colonies,
            diplomacy,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; PLAYER_SIZE];
        let mut pos = 0;

        buf[pos..pos + 24].copy_from_slice(&self.name_raw);
        pos += 24;
        buf[pos..pos + 24].copy_from_slice(&self.country_name_raw);
        pos += 24;

        buf[pos] = self.player_flags_raw;
        pos += 1;
        buf[pos] = self.control; pos += 1;
        buf[pos] = self.founded_colonies; pos += 1;
        buf[pos] = self.diplomacy;

        buf
    }
}

pub fn read_players(data: &[u8]) -> Result<Vec<Player>> {
    let mut players = Vec::with_capacity(PLAYER_COUNT);
    for i in 0..PLAYER_COUNT {
        let offset = i * PLAYER_SIZE;
        players.push(Player::read(&data[offset..])?);
    }
    Ok(players)
}

pub fn write_players(players: &[Player]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(PLAYER_COUNT * PLAYER_SIZE);
    for p in players {
        buf.extend_from_slice(&p.write());
    }
    buf
}
