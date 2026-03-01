use crate::bits::{BitReader, BitWriter};
use crate::error::Result;

/// TRADE_ROUTE section. Always 12 entries.
pub const TRADE_ROUTE_COUNT: usize = 12;

/// A single trade route stop.
#[derive(Debug, Clone, Copy, Default)]
pub struct TradeRouteStop {
    pub colony_index: u16,
    /// Nibble pair: high 4 bits = unloads_count, low 4 bits = loads_count
    pub unloads_count: u8, // 4 bits
    pub loads_count: u8, // 4 bits
    /// 6 cargo type nibbles for loads (3 bytes = 6 × 4-bit)
    pub loads_cargo: [u8; 6],
    /// 6 cargo type nibbles for unloads (3 bytes = 6 × 4-bit)
    pub unloads_cargo: [u8; 6],
    pub unknown: u8,
}

impl TradeRouteStop {
    /// Byte size of one stop: 2 + 1 + 3 + 3 + 1 = 10
    pub const SIZE: usize = 10;

    pub fn read(data: &[u8]) -> Self {
        let mut pos = 0;

        let colony_index = u16::from_le_bytes([data[pos], data[pos + 1]]);
        pos += 2;

        // loads_and_unloads_count: bit_struct (1 byte)
        let count_byte = data[pos];
        pos += 1;
        let mut cr = BitReader::new(std::slice::from_ref(&count_byte));
        let unloads_count = cr.read_u8(4);
        let loads_count = cr.read_u8(4);

        // loads_cargo: 6 nibbles in 3 bytes
        let mut loads_cargo = [0u8; 6];
        for i in 0..3 {
            loads_cargo[i * 2] = (data[pos] >> 4) & 0x0F;
            loads_cargo[i * 2 + 1] = data[pos] & 0x0F;
            pos += 1;
        }

        // unloads_cargo: 6 nibbles in 3 bytes
        let mut unloads_cargo = [0u8; 6];
        for i in 0..3 {
            unloads_cargo[i * 2] = (data[pos] >> 4) & 0x0F;
            unloads_cargo[i * 2 + 1] = data[pos] & 0x0F;
            pos += 1;
        }

        let unknown = data[pos];

        Self {
            colony_index,
            unloads_count,
            loads_count,
            loads_cargo,
            unloads_cargo,
            unknown,
        }
    }

    pub fn write(&self, buf: &mut [u8]) {
        let mut pos = 0;

        buf[pos..pos + 2].copy_from_slice(&self.colony_index.to_le_bytes());
        pos += 2;

        // Pack loads_and_unloads_count
        {
            let mut count_buf = [0u8; 1];
            let mut w = BitWriter::new(&mut count_buf);
            w.write_u8(4, self.unloads_count);
            w.write_u8(4, self.loads_count);
            buf[pos] = count_buf[0];
        }
        pos += 1;

        // Pack loads_cargo: 6 nibbles → 3 bytes
        for i in 0..3 {
            buf[pos] = (self.loads_cargo[i * 2] << 4) | (self.loads_cargo[i * 2 + 1] & 0x0F);
            pos += 1;
        }

        // Pack unloads_cargo: 6 nibbles → 3 bytes
        for i in 0..3 {
            buf[pos] = (self.unloads_cargo[i * 2] << 4) | (self.unloads_cargo[i * 2 + 1] & 0x0F);
            pos += 1;
        }

        buf[pos] = self.unknown;
    }
}

#[derive(Debug, Clone)]
pub struct TradeRoute {
    pub name_raw: [u8; 32], // 32 bytes, raw preservation
    pub land_or_sea: u8,    // trade_route_type (0=land, 1=sea)
    pub stops_count: u8,
    pub stops: [TradeRouteStop; 4],
}

impl TradeRoute {
    /// Byte size of one trade route:
    /// 32 (name) + 1 (type) + 1 (stops_count) + 4 × 10 (stops) = 74
    pub const SIZE: usize = 74;

    pub fn name(&self) -> &str {
        let end = self.name_raw.iter().position(|&b| b == 0).unwrap_or(32);
        std::str::from_utf8(&self.name_raw[..end]).unwrap_or("")
    }

    pub fn read(data: &[u8]) -> Result<Self> {
        let mut pos = 0;

        let mut name_raw = [0u8; 32];
        name_raw.copy_from_slice(&data[pos..pos + 32]);
        pos += 32;

        let land_or_sea = data[pos];
        pos += 1;
        let stops_count = data[pos];
        pos += 1;

        let mut stops = [TradeRouteStop::default(); 4];
        for stop in &mut stops {
            *stop = TradeRouteStop::read(&data[pos..]);
            pos += TradeRouteStop::SIZE;
        }

        Ok(TradeRoute {
            name_raw,
            land_or_sea,
            stops_count,
            stops,
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Self::SIZE];
        let mut pos = 0;

        buf[pos..pos + 32].copy_from_slice(&self.name_raw);
        pos += 32;

        buf[pos] = self.land_or_sea;
        pos += 1;
        buf[pos] = self.stops_count;
        pos += 1;

        for stop in &self.stops {
            stop.write(&mut buf[pos..]);
            pos += TradeRouteStop::SIZE;
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_route_stop_round_trip() {
        let stop = TradeRouteStop {
            colony_index: 321,
            unloads_count: 4,
            loads_count: 2,
            loads_cargo: [1, 2, 3, 4, 5, 6],
            unloads_cargo: [6, 5, 4, 3, 2, 1],
            unknown: 0xAB,
        };

        let mut buf = [0u8; TradeRouteStop::SIZE];
        stop.write(&mut buf);
        let parsed = TradeRouteStop::read(&buf);

        assert_eq!(parsed.colony_index, stop.colony_index);
        assert_eq!(parsed.unloads_count, stop.unloads_count);
        assert_eq!(parsed.loads_count, stop.loads_count);
        assert_eq!(parsed.loads_cargo, stop.loads_cargo);
        assert_eq!(parsed.unloads_cargo, stop.unloads_cargo);
        assert_eq!(parsed.unknown, stop.unknown);
    }

    #[test]
    fn test_trade_route_name() {
        let mut name_raw = [0u8; 32];
        name_raw[..10].copy_from_slice(b"Sugar Run\0");
        let route = TradeRoute {
            name_raw,
            land_or_sea: 0,
            stops_count: 0,
            stops: [TradeRouteStop::default(); 4],
        };

        assert_eq!(route.name(), "Sugar Run");
    }
}
