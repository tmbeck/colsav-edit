use crate::error::Result;

/// Map layer: a 2D grid of raw bytes, rows × cols.
/// Each map layer is map_size_y rows × map_size_x cols, 1 byte per tile.
///
/// TILE: 5-bit terrain + 3-bit hills_river per byte (bit_struct, MSB first)
/// MASK: 8 individual bit flags per byte (has_unit, has_city, suppress, road, purchased, pacific, plowed, unused)
/// PATH: 4-bit region_id + 4-bit visitor_nation per byte
/// SEEN: 4-bit score + 4 visibility bits (E/F/S/D) per byte
///
/// We store each layer as a flat Vec<u8> with dimensions for indexing.
/// The bit-level interpretation is left to higher-level code.

#[derive(Debug, Clone)]
pub struct MapLayer {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<u8>,
}

impl MapLayer {
    pub fn byte_size(rows: usize, cols: usize) -> usize {
        rows * cols
    }

    pub fn read(data: &[u8], rows: usize, cols: usize) -> Result<Self> {
        let size = rows * cols;
        let mut map_data = vec![0u8; size];
        map_data.copy_from_slice(&data[..size]);
        Ok(Self {
            rows,
            cols,
            data: map_data,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Get the byte at (row, col).
    pub fn get(&self, row: usize, col: usize) -> u8 {
        self.data[row * self.cols + col]
    }

    /// Set the byte at (row, col).
    pub fn set(&mut self, row: usize, col: usize, val: u8) {
        self.data[row * self.cols + col] = val;
    }
}

// Type aliases for clarity
pub type TileMap = MapLayer;
pub type MaskMap = MapLayer;
pub type PathMap = MapLayer;
pub type SeenMap = MapLayer;

/// Connectivity section: sea_lane (18×15) + land (18×15).
/// Each byte = 8 directional connectivity bits.
#[derive(Debug, Clone)]
pub struct Connectivity {
    pub sea_lane: Vec<u8>, // 18 × 15 = 270 bytes
    pub land: Vec<u8>,     // 18 × 15 = 270 bytes
}

impl Connectivity {
    pub const ROWS: usize = 18;
    pub const COLS: usize = 15;
    pub const SECTION_SIZE: usize = Self::ROWS * Self::COLS; // 270
    pub const TOTAL_SIZE: usize = Self::SECTION_SIZE * 2; // 540

    pub fn read(data: &[u8]) -> Result<Self> {
        let mut sea_lane = vec![0u8; Self::SECTION_SIZE];
        sea_lane.copy_from_slice(&data[..Self::SECTION_SIZE]);

        let mut land = vec![0u8; Self::SECTION_SIZE];
        land.copy_from_slice(&data[Self::SECTION_SIZE..Self::TOTAL_SIZE]);

        Ok(Self { sea_lane, land })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::TOTAL_SIZE);
        buf.extend_from_slice(&self.sea_lane);
        buf.extend_from_slice(&self.land);
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_layer_get_set() {
        let mut layer = MapLayer::read(&[1, 2, 3, 4], 2, 2).expect("map read should succeed");

        assert_eq!(layer.get(0, 0), 1);
        assert_eq!(layer.get(1, 1), 4);

        layer.set(1, 0, 9);
        assert_eq!(layer.get(1, 0), 9);
    }

    #[test]
    fn test_connectivity_round_trip() {
        let sea_lane: Vec<u8> = (0..Connectivity::SECTION_SIZE)
            .map(|i| u8::try_from(i % 251).expect("value should fit in u8"))
            .collect();
        let land: Vec<u8> = (0..Connectivity::SECTION_SIZE)
            .map(|i| u8::try_from((i * 3) % 251).expect("value should fit in u8"))
            .collect();

        let conn = Connectivity { sea_lane, land };
        let bytes = conn.write();
        let parsed = Connectivity::read(&bytes).expect("connectivity parse should succeed");

        assert_eq!(parsed.sea_lane, conn.sea_lane);
        assert_eq!(parsed.land, conn.land);
    }
}
