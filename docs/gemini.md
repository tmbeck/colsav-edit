The save game files for the 1994 DOS version of Sid Meier's Colonization (COLONY00.SAV through COLONY09.SAV) are uncompressed binary blobs with a fixed length of 35,930 bytes. Data is stored primarily in little-endian format.

File Structure Overview
The file is partitioned into segments handling the global state, player-specific data, and map tiles.

Offset (Hex)	Size (Bytes)	Description
0x0000	2	Game Year (e.g., 0x05DE = 1502)
0x0002	2	Current Player Turn (0-4)
0x000C	2	Difficulty Level (0=Discoverer, 4=Viceregal)
0x002E	16	Player Flags (Aggression, AI state)
0x0372	2048	Map Tile Data (Terrain types, features)
0x0B72	2048	Map Visibility/Fog of War
0x2500	Variable	Colony Records (Buildings, stock, population)
0x6000	Variable	Unit Records (Coordinates, type, status, cargo)
Key Data Structures
1. Map Tiles (0x0372)
The map is typically 58×72 tiles. Each tile is represented by a single byte encoding the terrain type. High bits often store "forested" or "plowed" flags.

2. Colony Data
Each colony entry occupies a fixed-length block.

Production: Offsets within the colony block track accumulated hammers/bells.

Warehouse: 16-bit integers for each of the 16 primary goods (Food, Sugar, Tobacco, Cotton, Fur, Wood, Ore, Silver, Horses, Rum, Cigars, Cloth, Coats, Tools, Muskets, Trade Goods).

3. Unit Data
Units are stored in a contiguous array. Each unit entry (approx. 28 bytes) includes:

X/Y Coordinates: 1 byte each.

Unit Type: ID corresponding to the internal lookup table (e.g., 0x00 = Colonist, 0x12 = Man-o-War).

Status: Bitmask for Sentry, Fortify, or Active.

Cargo: 3 slots, each with a Type ID and Quantity byte.

Technical Caveats
Checksums: The DOS executable does not perform a CRC check on load; however, malformed coordinate data in unit blocks will trigger a "Divide by Zero" or "General Protection Fault" in the original engine.

Endianness: Standard x86 little-endian. 16-bit integers are swapped (LSB first).

Technical Specification: Sid Meier's Colonization (DOS, 1994) Save Game Format
The save game files (COLONY00.SAV through COLONY09.SAV) are uncompressed, fixed-length binary blobs of 35,930 bytes. The engine uses standard x86 little-endian (Intel) byte ordering for all 16-bit and 32-bit integers.

1. File Header and Global State
The first section of the file contains global game parameters and player-specific status flags.

Offset (Hex)	Size (Bytes)	Data Type	Description
0x0000	2	UINT16	Game Year (e.g., 1492 = 0x05D4)
0x0002	2	UINT16	Current Player Turn (0-4; 0=England, 1=France, etc.)
0x000C	2	UINT16	Difficulty Level (0=Discoverer, 4=Viceregal)
0x002E	16	BYTE[16]	AI Aggression/Diplomacy Flags
0x0080	512	BYTE[512]	Founding Fathers (Bitmask of obtained fathers)
2. Map Data Structures
The map is a grid of 58×72 tiles. Data is stored in row-major order.

Terrain Types (0x0372): 2,048 bytes. Each byte represents a tile.

Low 4 bits: Base terrain (0=Ocean, 1=Tundra, 2=Desert, 3=Plains, 4=Prairie, 5=Grassland, 6=Savannah, 7=Swamp, 8=Marsh).

High 4 bits: Modifiers (Forest/Wetland flag, Hills, Mountains).

Map Features (0x0B72): 2,048 bytes. Tracks "Plowed," "Road," and "Irrigated" status via bitmask.

Visibility (0x1372): 2,048 bytes per player. Tracks Fog of War.

3. Colony Records
Colony data begins near 0x2500. Each colony is a fixed-size structure. If a colony is destroyed, its record remains but is flagged as inactive.

Warehouse/Stockpile (Inside Colony Block):
Goods are stored as 16-bit integers in a specific sequence:

Food

Sugar

Tobacco

Cotton

Fur

Wood

Ore

Silver

Horses

Rum

Cigars

Cloth

Coats

Tools

Muskets

Trade Goods

Building Status:
Buildings are tracked via a bitmask or level indicator (0=None, 1=Basic, 2=Improved, 3=Advanced).

4. Unit Records
Unit data starts at 0x6000. Each unit occupies a 28-byte block.

Offset (Relative)	Size (Bytes)	Description
+0x00	1	X-Coordinate
+0x01	1	Y-Coordinate
+0x02	1	Unit Type ID (e.g., 0x00 Colonist, 0x07 Scout, 0x12 Man-o-War)
+0x03	1	Owner ID (0-4 for Europeans, 5+ for Natives)
+0x04	1	Status Flag (0=Active, 1=Sentry, 2=Fortifying, 3=Fortified)
+0x05	1	Specialty/Profession ID
+0x0A	6	Cargo Slots (3 slots: 1 byte for Type ID, 1 byte for Quantity)
5. Native Settlements
Located after the European colony blocks. These records track the "Alarm" level of specific tribes toward each European power and the primary resource produced by the settlement for trade.

Implementation Notes for Linux/C
To parse this in a modern environment:

Memory Mapping: Use mmap() to map the 35,930-byte file into a struct-aligned buffer.

Struct Alignment: Use __attribute__((packed)) on your C structs to prevent the compiler from inserting padding that would misalign the offsets.

Coordinate Validation: The engine is highly sensitive to OOB (Out-of-Bounds) coordinates. Moving a unit to X > 57 or Y > 71 will usually cause an immediate CTD (Crash to Desktop) or heap corruption when the executable attempts to calculate the memory offset for the map sprite.

