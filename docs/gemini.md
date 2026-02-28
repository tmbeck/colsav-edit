# Colonization Save Game File Format Specification

The save game files for the 1994 DOS version of *Sid Meier's Colonization* (COLONY00.SAV through COLONY09.SAV) are uncompressed binary blobs with a fixed length of **35,930 bytes**. Data is stored primarily in little-endian format.

### File Structure Overview

The file is partitioned into segments handling the global state, player-specific data, and map tiles.

| Offset (Hex) | Size (Bytes) | Description |
| --- | --- | --- |
| `0x0000` | 2 | Game Year (e.g., `0x05DE` = 1502) |
| `0x0002` | 2 | Current Player Turn (0-4) |
| `0x000C` | 2 | Difficulty Level (0=Discoverer, 4=Viceregal) |
| `0x002E` | 16 | Player Flags (Aggression, AI state) |
| `0x0372` | 2048 | Map Tile Data (Terrain types, features) |
| `0x0B72` | 2048 | Map Visibility/Fog of War |
| `0x2500` | Variable | Colony Records (Buildings, stock, population) |
| `0x6000` | Variable | Unit Records (Coordinates, type, status, cargo) |

---

### Key Data Structures

#### 1. Map Tiles (`0x0372`)

The map is typically $58 \times 72$ tiles. Each tile is represented by a single byte encoding the terrain type. High bits often store "forested" or "plowed" flags.

#### 2. Colony Data

Each colony entry occupies a fixed-length block.

* **Production:** Offsets within the colony block track accumulated hammers/bells.
* **Warehouse:** 16-bit integers for each of the 16 primary goods (Food, Sugar, Tobacco, Cotton, Fur, Wood, Ore, Silver, Horses, Rum, Cigars, Cloth, Coats, Tools, Muskets, Trade Goods).

#### 3. Unit Data

Units are stored in a contiguous array. Each unit entry (approx. 28 bytes) includes:

* **X/Y Coordinates:** 1 byte each.
* **Unit Type:** ID corresponding to the internal lookup table (e.g., `0x00` = Colonist, `0x12` = Man-o-War).
* **Status:** Bitmask for Sentry, Fortify, or Active.
* **Cargo:** 3 slots, each with a Type ID and Quantity byte.

### Technical Caveats

* **Checksums:** The DOS executable does not perform a CRC check on load; however, malformed coordinate data in unit blocks will trigger a "Divide by Zero" or "General Protection Fault" in the original engine.
* **Endianness:** Standard x86 little-endian. 16-bit integers are swapped (LSB first).

Would you like the specific hex offsets for the individual commodity types within a colony's warehouse block?




### Technical Specification: Sid Meier's Colonization (DOS, 1994) Save Game Format

The save game files (`COLONY00.SAV` through `COLONY09.SAV`) are uncompressed, fixed-length binary blobs of **35,930 bytes**. The engine uses standard x86 little-endian (Intel) byte ordering for all 16-bit and 32-bit integers.

---

#### 1. File Header and Global State

The first section of the file contains global game parameters and player-specific status flags.

| Offset (Hex) | Size (Bytes) | Data Type | Description |
| --- | --- | --- | --- |
| `0x0000` | 2 | UINT16 | Game Year (e.g., `1492` = `0x05D4`) |
| `0x0002` | 2 | UINT16 | Current Player Turn (0-4; 0=England, 1=France, etc.) |
| `0x000C` | 2 | UINT16 | Difficulty Level (0=Discoverer, 4=Viceregal) |
| `0x002E` | 16 | BYTE[16] | AI Aggression/Diplomacy Flags |
| `0x0080` | 512 | BYTE[512] | Founding Fathers (Bitmask of obtained fathers) |

---

#### 2. Map Data Structures

The map is a grid of $58 \times 72$ tiles. Data is stored in row-major order.

* **Terrain Types (`0x0372`):** 2,048 bytes. Each byte represents a tile.
* Low 4 bits: Base terrain (0=Ocean, 1=Tundra, 2=Desert, 3=Plains, 4=Prairie, 5=Grassland, 6=Savannah, 7=Swamp, 8=Marsh).
* High 4 bits: Modifiers (Forest/Wetland flag, Hills, Mountains).


* **Map Features (`0x0B72`):** 2,048 bytes. Tracks "Plowed," "Road," and "Irrigated" status via bitmask.
* **Visibility (`0x1372`):** 2,048 bytes per player. Tracks Fog of War.

---

#### 3. Colony Records

Colony data begins near `0x2500`. Each colony is a fixed-size structure. If a colony is destroyed, its record remains but is flagged as inactive.

**Warehouse/Stockpile (Inside Colony Block):**
Goods are stored as 16-bit integers in a specific sequence:

1. Food
2. Sugar
3. Tobacco
4. Cotton
5. Fur
6. Wood
7. Ore
8. Silver
9. Horses
10. Rum
11. Cigars
12. Cloth
13. Coats
14. Tools
15. Muskets
16. Trade Goods

**Building Status:**
Buildings are tracked via a bitmask or level indicator (0=None, 1=Basic, 2=Improved, 3=Advanced).

---

#### 4. Unit Records

Unit data starts at `0x6000`. Each unit occupies a 28-byte block.

| Offset (Relative) | Size (Bytes) | Description |
| --- | --- | --- |
| `+0x00` | 1 | X-Coordinate |
| `+0x01` | 1 | Y-Coordinate |
| `+0x02` | 1 | Unit Type ID (e.g., `0x00` Colonist, `0x07` Scout, `0x12` Man-o-War) |
| `+0x03` | 1 | Owner ID (0-4 for Europeans, 5+ for Natives) |
| `+0x04` | 1 | Status Flag (0=Active, 1=Sentry, 2=Fortifying, 3=Fortified) |
| `+0x05` | 1 | Specialty/Profession ID |
| `+0x0A` | 6 | Cargo Slots (3 slots: 1 byte for Type ID, 1 byte for Quantity) |

---

#### 5. Native Settlements

Located after the European colony blocks. These records track the "Alarm" level of specific tribes toward each European power and the primary resource produced by the settlement for trade.

### Implementation Notes for Linux/C

To parse this in a modern environment:

1. **Memory Mapping:** Use `mmap()` to map the 35,930-byte file into a struct-aligned buffer.
2. **Struct Alignment:** Use `__attribute__((packed))` on your C structs to prevent the compiler from inserting padding that would misalign the offsets.
3. **Coordinate Validation:** The engine is highly sensitive to OOB (Out-of-Bounds) coordinates. Moving a unit to `X > 57` or `Y > 71` will usually cause an immediate CTD (Crash to Desktop) or heap corruption when the executable attempts to calculate the memory offset for the map sprite.




### Header and Entity Counts (Offsets `0x0000` - `0x0371`)

The header stores global state and dynamically defines the active entity loops for the game engine.

| Offset | Size | Type | Description |
| --- | --- | --- | --- |
| `0x001A` | 2 | UINT16 | Total Active Colonies |
| `0x001C` | 2 | UINT16 | Total Active Units |
| `0x001E` | 2 | UINT16 | Total Active Native Villages |
| `0x0026` | 2 | UINT16 | Next Available Unit ID |

The game processes arrays sequentially up to these index limits. Orphaned data past these counts in the contiguous arrays is ignored by the engine.

### Map Architecture (`0x0372` - `0x2471`)

The 58x72 map grid requires precise synchronization across three separate arrays. Coordinates `(x, y)` map to an linear offset using the formula: `Offset = (y * 58) + x`.

#### Terrain Array (`0x0372`, 4176 Bytes)

Each byte encodes the base terrain and its modifier state using bitmasks.

**Low Nibble (Bits 0-3): Base Terrain**

* `0x00`: Tundra
* `0x01`: Desert
* `0x02`: Plains
* `0x03`: Prairie
* `0x04`: Grassland
* `0x05`: Savannah
* `0x06`: Marsh
* `0x07`: Swamp
* `0x08`: Ocean
* `0x09`: Sea Lane (Atlantic/Pacific routing depends on additional mask arrays)

**High Nibble (Bits 4-7): Modifiers**

* `Bit 4 (0x10)`: Forested (e.g., `0x14` = Forested Grassland = Conifer)
* `Bit 5 (0x20)`: Hills
* `Bit 6 (0x40)`: Mountains
* `Bit 7 (0x80)`: River

*Note: The map generator applies valid base types under mountains/hills, determining the yield when mined.*

#### Feature Array (`0x0B72`, 4176 Bytes)

Bitmask array handling tile improvements and Prime Resource (LCR) visibility.

* `Bit 0 (0x01)`: Plowed / Cleared
* `Bit 1 (0x02)`: Road built
* `Bit 2 (0x04)`: Prime Resource present (Algorithmically verified against the (17*n+c)%64 LCR pattern map)

#### Visibility Array (`0x1372`)

Multiple consecutive 4176-byte arrays handling Fog of War state per player index.

---

### Colony Structures (Start approx. `0x2500`)

Colonies are stored as a contiguous array of fixed-length structs (typically 200+ bytes each, exact size varies by executable version patching).

**Key Colony Struct Offsets (Relative to Colony Base Address):**
| Relative Offset | Size | Description |
| :--- | :--- | :--- |
| `+0x00` | 24 | Null-terminated ASCII String (Colony Name) |
| `+0x18` | 1 | X-Coordinate |
| `+0x19` | 1 | Y-Coordinate |
| `+0x1A` | 1 | Player Owner ID (0-3 European, 4=Royal Expeditionary Force) |
| `+0x20` | 32 | Warehouse Goods Array (16 x UINT16). Order: Food, Sugar, Tobacco, Cotton, Fur, Wood, Ore, Silver, Horses, Rum, Cigars, Cloth, Coats, Tools, Muskets, Trade Goods. |
| `+0x40` | 16 | Building Levels (1 byte per building type. 0=None, 1=Level 1, etc.) |
| `+0x50` | 32 | Unit Work assignments (Internal Unit IDs mapped to building slots or terrain tiles) |

---

### Unit Structures (`0x6000` area)

Units are stored in a 28-byte packed struct array.

**Unit Type Lookup (`+0x02` in Unit Struct):**
| Hex Value | Unit Type |
| :--- | :--- |
| `0x00` | Free Colonist |
| `0x01` | Armed Colonist (Soldier/Dragoon depending on horses) |
| `0x02` | Pioneer |
| `0x03` | Missionary |
| `0x05` | Scout |
| `0x08` | Artillery |
| `0x09` | Wagon Train |
| `0x0A` | Caravel |
| `0x0B` | Merchantman |
| `0x0C` | Galleon |
| `0x0D` | Privateer |
| `0x0E` | Frigate |
| `0x0F` | Man-o-War |

**Unit Status / Orders (`+0x04` in Unit Struct):**
Tracks the immediate loop instruction for the AI/Player unit state machine.

* `0x00`: Active (Awaiting input)
* `0x01`: Fortified
* `0x02`: Sentry
* `0x03`: Go-To (Coordinates stored in `+0x0B` and `+0x0C`)
* `0x04`: Plow/Clear
* `0x05`: Build Road

**Profession/Specialty (`+0x05` in Unit Struct):**
Modifies base colonist behavior. Overrides the graphical sprite.

* `0x00`: None (Free Colonist)
* `0x01`: Indentured Servant
* `0x02`: Petty Criminal
* `0x03`: Native Convert
* `0x04`: Expert Farmer
* `0x05`: Expert Fisherman
* `0x0A`: Master Carpenter
* `0x0B`: Master Blacksmith
* `0x0E`: Veteran Soldier

**Cargo Slots (`+0x10` to `+0x15` in Unit Struct):**
6 bytes handling 3 slots (2 bytes per slot: Type, Quantity).

* **Type byte:** `0x00`-`0x0F` correspond to warehouse goods. `0x10`+ map to Unit IDs (indicating a unit is loaded onto a ship/wagon).
* **Quantity byte:** 0-100 for goods. Ignored if the type is a unit.

### Native Village Structures

Located immediately following the European colony arrays.

* Tracks X/Y coordinates.
* 1 byte for Tribe ID (Inca, Aztec, Arawak, etc. determines base aggressiveness and sprite).
* 4 bytes storing the "Tension/Alarm" level toward the 4 European powers (0-255 scale).
* 1 byte for the specialized trade good offered by the village.
* 1 byte tracking missionary presence (maps to a European player ID).


### Player State and Economy Block

Data pertaining to European powers is stored sequentially by Player ID (0=England, 1=France, 2=Spain, 3=Netherlands, 4=REF).

* **Treasury:** 32-bit signed integer (`INT32`). Modification beyond standard limits (e.g., setting to `0x7FFFFFFF`) will not crash the engine but can trigger arithmetic overflows in trade calculations.
* **Tax Rate:** 16-bit integer (`INT16`) representing the current tax percentage imposed by the Crown. The maximum legitimate value is 100.
* **Crosses:** Two `INT16` fields. One tracks accumulated crosses; the other tracks the dynamically increasing threshold required to trigger the next immigrant on the European docks.
* **Diplomatic Stance:** A matrix of byte values tracking relations between European powers and Native tribes (evaluating states such as Peace, War, Alliance, and Ceasefire).

### European Market Dynamics

The internal commodity market in Europe dictates the buy/sell prices of the 16 standard goods. The data is partitioned into parallel arrays.

* **Current Prices:** Two arrays (Buy and Sell) storing the active price per 100 units.
* **Volume Accumulators:** `INT16` values tracking the net volume of each good sold to or bought from Europe. When an accumulator crosses an internal threshold (which decreases as the game progresses), the engine triggers a price drop or hike.
* **Boycott Status:** A bitmask tracking which specific goods are currently blockaded by the Crown due to refused tax hikes. A bit value of `1` prevents the player from selling that commodity in the European port until arrears are paid via the customs house or diplomacy.

### Royal Expeditionary Force (REF) Build-Up

The REF is not spawned on the map until the Declaration of Independence. Prior to that, it exists purely as a set of `INT16` counters tracking the King's reserves.

* **Composition Counters:** Four distinct values tracking the exact number of:
1. Regular Infantry
2. Dragoons
3. Artillery
4. Man-o-Wars


* **Foreign Intervention:** A parallel set of counters tracks the military aid pledged by a rival European monarch. These units spawn only if the player generates sufficient Liberty Bells after declaring independence.

### Continental Congress (Founding Fathers)

The state of the Continental Congress is managed via an array corresponding to the 25 historical figures.

* **Recruitment State:** Each index contains a byte indicating status: `0x00` (Unassigned/Available), `0x01` (Currently generating bells toward recruitment), or `0x02` (Recruited).
* **Bell Accumulator:** An `INT16` value tracking total Liberty Bells generated specifically toward the currently targeted Founding Father.

### Debug and State Flags

The primary header (first 256 bytes) includes binary flags controlling engine behavior.

* **Cheat Mode Flag:** Toggling the debug byte activates the internal `ALT-W-I-N` routine on load. This forces the engine to bypass the Fog of War arrays (`0x1372`) and exposes the memory-mapped unit editor directly within the UI.
* **Difficulty/Handicap:** The value at `0x000C` directly modifies the divisor used in the volume-to-price-drop market calculation. Higher difficulties result in faster market saturation and more aggressive price depreciation.



# Sid Meier's Colonization (DOS, 1994) `.SAV` Format Specification

## 1. File Architecture Overview

The `.SAV` files (`COLONY00.SAV` through `COLONY09.SAV`) are uncompressed, fixed-length binary binaries of exactly **35,930 bytes**.

* **Endianness:** Little-endian (x86 standard).
* **Alignment:** 1-byte packed (no padding inserted by the original compiler).
* **Checksums:** None. Modifying data directly does not require recalculating a CRC. OOB (Out-of-Bounds) coordinates will cause General Protection Faults or division-by-zero crashes in the engine.

## 2. Global Header & Entity Counts (`0x0000` - `0x0371`)

The header maintains global state variables and defines the loop limits for the engine's entity processing.

| Offset | Size | Type | Description |
| --- | --- | --- | --- |
| `0x0000` | 2 | `UINT16` | Game Year (e.g., `0x05D4` = 1492) |
| `0x0002` | 2 | `UINT16` | Current Player Turn (0=England, 1=France, 2=Spain, 3=Netherlands, 4=REF) |
| `0x000C` | 2 | `UINT16` | Difficulty Level (0=Discoverer, 4=Viceregal). Scales market saturation divisors. |
| `0x001A` | 2 | `UINT16` | Total Active Colonies. Defines the loop limit for the colony array. |
| `0x001C` | 2 | `UINT16` | Total Active Units. |
| `0x001E` | 2 | `UINT16` | Total Active Native Villages. |
| `0x0026` | 2 | `UINT16` | Next Available Unit ID. |
| `0x002E` | 16 | `BYTE[16]` | AI Diplomacy / Aggression state matrix. |
| `0x0080` | 512 | `BYTE[512]` | Continental Congress State. Tracks 25 Founding Fathers (`0x00`=Available, `0x01`=Recruiting, `0x02`=Recruited). Includes `INT16` bell accumulators per father. |
| `0x0200` | 1 | `BYTE` | Cheat Mode Flag (`0x00`=Off, `0x01`=On). Enables `ALT-W-I-N` menu and disables Fog of War checks. |

## 3. Map Data Arrays (`0x0372` + Sequence)

The game map is a fixed $58 \times 72$ grid. Each map layer is exactly 4,176 bytes. Coordinate mapping follows `Offset = (y * 58) + x`.

### Layer 1: Base Terrain & Modifiers

Each byte defines the tile topography.

* **Low Nibble (Base):** `0x0` Tundra, `0x1` Desert, `0x2` Plains, `0x3` Prairie, `0x4` Grassland, `0x5` Savannah, `0x6` Marsh, `0x7` Swamp, `0x8` Ocean, `0x9` Sea Lane.
* **High Nibble (Modifiers):** `0x10` Forested, `0x20` Hills, `0x40` Mountains, `0x80` River.
*(Example: `0x14` = Forested Grassland = Conifer Forest)*

### Layer 2: Tile Features & LCRs

A 4,176-byte bitmask array overlaying the terrain.

* `Bit 0 (0x01)`: Plowed / Cleared.
* `Bit 1 (0x02)`: Road built.
* `Bit 2 (0x04)`: Prime Resource present. (Algorithmically verified against the internal `(17*n+c)%64` LCR pattern map upon initialization).
* `Bit 3 (0x08)`: Lost City Rumor present.

### Layer 3: Visibility / Fog of War

Multiple sequential 4,176-byte arrays handling visibility per European power.

### Layer 4: Ocean Routing Mask

Determines if an ocean tile routes to the Atlantic or Pacific when a ship sails to Europe. `0x00`/`0x04` routes Atlantic; `0x20`/`0x24` routes Pacific.

## 4. Economy & Player State

European power data is stored sequentially by Player ID (0 to 4).

* **Treasury:** `INT32`. Maximum safe limit is `0x7FFFFFFF` to prevent arithmetic overflow in customs house routines.
* **Tax Rate:** `INT16`. Current percentage (0-100).
* **Crosses:** Two `INT16` fields (Accumulated Crosses, Current Threshold for next immigrant).
* **Market Arrays:**
* **Buy/Sell Prices:** Parallel arrays mapping the 16 commodities.
* **Volume Accumulators:** `INT16` nets of trade volume. Hitting limits triggers price volatility.
* **Boycott Bitmask:** A single `UINT16` where active bits blockade the respective good ID in the European port.



## 5. Entity Arrays

### Colony Struct Array

Contiguous block of fixed-length structs. Orphaned records persist if a colony is destroyed but are ignored based on the Active Colony count.

| Relative Offset | Size | Description |
| --- | --- | --- |
| `+0x00` | 24 | Null-terminated ASCII string (Colony Name). |
| `+0x18` | 1 | X-Coordinate. |
| `+0x19` | 1 | Y-Coordinate. |
| `+0x1A` | 1 | Owner ID (0-3 European, 4 REF). |
| `+0x20` | 32 | Warehouse Goods. Array of 16 `UINT16` values. Order: Food, Sugar, Tobacco, Cotton, Fur, Wood, Ore, Silver, Horses, Rum, Cigars, Cloth, Coats, Tools, Muskets, Trade Goods. |
| `+0x40` | 16 | Building Levels. Array of 16 bytes. `0x00`=None, `0x01`=Level 1, `0x02`=Level 2, etc. (e.g., Stockade=`0x41`, Fort=`0x43`, Fortress=`0x47`). |
| `+0x50` | 32 | Unit Work Assignments. Internal Unit IDs mapped to building/terrain slots. |

### Unit Struct Array (Start approx `0x6000`)

Packed 28-byte structures.

| Relative Offset | Size | Description |
| --- | --- | --- |
| `+0x00` | 1 | X-Coordinate. |
| `+0x01` | 1 | Y-Coordinate. |
| `+0x02` | 1 | Unit Type ID. `0x00` Colonist, `0x02` Pioneer, `0x08` Artillery, `0x09` Wagon, `0x0C` Galleon, `0x0F` Man-o-War. |
| `+0x03` | 1 | Owner ID. |
| `+0x04` | 1 | State Machine Instruction. `0x00` Active, `0x01` Fortified, `0x02` Sentry, `0x03` Go-To, `0x04` Plow, `0x05` Road. |
| `+0x05` | 1 | Profession ID. Overrides sprite. `0x00` None, `0x01` Indentured, `0x02` Criminal, `0x04` Expert Farmer, `0x0E` Veteran. |
| `+0x0A` | 6 | Cargo Slots. 3 slots total. 2 bytes per slot (Byte 1: Type ID, Byte 2: Quantity). For Type ID: `0x00`-`0x0F` = Goods. `0x10+` = Internal Unit ID (loaded passenger). |
| `+0x10` | 2 | Go-To Coordinates (Active if State == `0x03`). |

### Native Village Struct Array

Located after European arrays.

* **Location:** X/Y coordinates (2 bytes).
* **Tribe ID:** 1 byte (Determines AI aggression profile and sprite).
* **Alarm/Tension Matrix:** 4 bytes. `UINT8` values tracking hostility toward the 4 European powers.
* **Trade Capability:** 1 byte defining the primary local resource.
* **Missionary State:** 1 byte tracking the active European mission (matches European Player ID, or null).