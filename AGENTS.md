# AGENTS.md — Colonization SAV File Editor

## Project Overview

Rust workspace for reading, editing, and analyzing Sid Meier's Colonization (1994 DOS) save game
files (.SAV). Two crates: `colonization-sav` (library — parsing, serialization, round-trip) and
`colsav` (binary — CLI subcommands + retro DOS-style TUI editor using ratatui).

**Rust edition:** 2024  
**MSRV:** latest stable (uses edition 2024 features).  
**Dependencies:** thiserror 2, clap 4, anyhow 1, ratatui 0.29, crossterm 0.28.  
**License:** Unlicense (public domain).

---

## Repository Structure

```
Cargo.toml                      # Workspace root — members, lint config
Makefile                        # Build/test/lint/install targets

colonization-sav/               # Library crate — SAV format parser
  Cargo.toml
  src/
    lib.rs                      # Re-exports: SaveFile, SaveError, Goods16, enums::*
    error.rs                    # SaveError enum (InvalidMagic, UnexpectedEof, InvalidSize, Io, Other)
    savefile.rs                 # SaveFile — from_path/from_bytes/to_bytes/save, section layout
    enums.rs                    # sav_enum! macro + ~20 enums (TerrainType, UnitType, etc.)
    goods.rs                    # Goods16<T> — generic 16-element array for cargo/prices
    bits.rs                     # BitReader/BitWriter — MSB-first bit-level I/O
    display.rs                  # Display impls for Header, Unit, Colony, Nation, Player
    raw/
      mod.rs                    # Re-exports all raw types
      header.rs                 # Header (390 bytes), GameOptions, ColonyReportOptions, EventFlags
      player.rs                 # Player (52 bytes)
      colony.rs                 # Colony (202 bytes), Buildings bit-struct
      unit.rs                   # Unit (28 bytes)
      nation.rs                 # Nation (316 bytes), FoundingFathers, Relation, NationTrade
      tribe.rs                  # Tribe (18 bytes), TribeBLCS, TribeMission
      indian.rs                 # Indian (78 bytes), TribeFlags
      trade_route.rs            # TradeRoute (74 bytes), TradeRouteStop
      stuff.rs                  # Stuff (727 bytes), ForeignAffairsReport, NationUnitCounts, TribeDataBlock
      maps.rs                   # MapLayer, Connectivity, TileMap/MaskMap/PathMap/SeenMap type aliases

colsav/                         # Binary crate — CLI + TUI
  Cargo.toml
  src/
    main.rs                     # CLI with clap: info, dump-units, dump-colonies, dump-nations,
                                #   dump-map, edit, tui
    tui/
      mod.rs                    # TUI module root, run_tui entry point
      app.rs                    # App struct, event loop, tab/editing state machine
      theme.rs                  # DOS retro blue/cyan theme colors
      tabs.rs                   # Tab enum (Header, Colonies, Units, Nations, TradeRoutes, Tribes, Map)
      header_tab.rs             # Header tab renderer
      colonies_tab.rs           # Colonies tab renderer
      units_tab.rs              # Units tab renderer
      nations_tab.rs            # Nations tab renderer
      map_tab.rs                # Map tab renderer (ASCII terrain)
      trade_routes_tab.rs       # Trade Routes tab renderer
      tribes_tab.rs             # Tribes & Indians tab renderer

colonization/                   # Legacy Python package (original codebase, kept for reference)
docs/
  gemini.md                     # User-added docs (INACCURATE — see Key Warnings)
saves/                          # 10 test SAV files (COLONY01.SAV – COLONY10.SAV)
Format.md                       # Original binary format documentation
ALLTERRA.MP                     # Sample map file for testing
*.py                            # Legacy Python CLI scripts (kept for reference)
colsav/
  tests/
    cli.rs                      # CLI integration tests (10 tests)
```

---

## Build / Run / Test Commands

### Prerequisites

```bash
# Ensure Rust toolchain is available
. "$HOME/.cargo/env"
```

### Makefile targets (preferred)

```bash
make all              # fmt-check + clippy + test (full CI check)
make build            # cargo build --workspace (debug)
make check            # cargo check --workspace (type-check only, faster)
make test             # cargo test --workspace (53 tests)
make test-one T=name  # run a single test by name
make test-lib         # tests for colonization-sav only
make test-bin         # tests for colsav only
make fmt              # cargo fmt --all (apply formatting)
make fmt-check        # cargo fmt --all -- --check (CI mode)
make clippy           # cargo clippy --workspace (uses workspace lint config)
make clippy-strict    # cargo clippy --workspace -- -D warnings
make clean            # cargo clean
make release          # cargo build --release -p colsav
make install          # cargo install --path colsav
make tui              # run TUI with saves/COLONY01.SAV
make info             # dump info from saves/COLONY01.SAV
```

### Direct cargo commands

```bash
cargo test --workspace                     # all 53 tests
cargo test --workspace -- round_trip       # filter by test name
cargo test -p colonization-sav             # library tests only
cargo clippy --workspace                   # lint (0 warnings expected)
cargo fmt --all -- --check                 # formatting check
```

### CLI usage

```bash
# All subcommands take -f <FILE>
colsav info -f saves/COLONY01.SAV
colsav dump-units -f saves/COLONY01.SAV
colsav dump-colonies -f saves/COLONY01.SAV
colsav dump-nations -f saves/COLONY01.SAV
colsav dump-map -f saves/COLONY01.SAV
colsav edit -f input.SAV -o output.SAV -p 0 -g 500000    # set gold for power 0
colsav edit -f input.SAV -o output.SAV -p 0 -t 10         # set tax for power 0
colsav tui -f saves/COLONY01.SAV                           # launch TUI editor

# During development (without installing):
cargo run -p colsav -- info -f saves/COLONY01.SAV
cargo run -p colsav -- tui -f saves/COLONY01.SAV
```

### Using the library

```rust
use colonization_sav::SaveFile;

let save = SaveFile::from_path("saves/COLONY01.SAV")?;
println!("Colonies: {}", save.header.colony_count);
for unit in &save.units {
    println!("{unit}");
}
// Round-trip: re-serialize to bytes
let bytes = save.to_bytes();
save.save("output.SAV")?;
```

---

## Code Style Guidelines

### Workspace Lint Configuration

Lints are configured at the workspace level in the root `Cargo.toml`:

- `clippy::all` + `clippy::pedantic` at warn level
- `unsafe_code` = forbid
- Specific pedantic lints allowed (cast truncation/sign/wrap/lossless, module_name_repetitions,
  missing_errors_doc, missing_panics_doc, must_use_candidate, struct_excessive_bools,
  too_many_lines, wildcard_imports, similar_names, doc_markdown, match_same_arms,
  needless_pass_by_value, unnested_or_patterns, unreadable_literal, return_self_not_must_use,
  items_after_statements)

Both crates inherit with `[lints] workspace = true`.

### Module Organization

- Library crate (`colonization-sav`) re-exports key types from `lib.rs`:
  `SaveFile`, `SaveError`, `Goods16`, and all enums via `pub use enums::*`.
- Raw binary structs live under `src/raw/` — one module per SAV section.
- `raw/mod.rs` re-exports all raw types for convenience.
- Binary crate (`colsav`) imports as `colonization_sav::...` and has its own `tui/` module tree.

### Naming Conventions

- **Types:** PascalCase — `SaveFile`, `TradeRoute`, `Colony`, `Unit`, `Nation`, `Goods16`.
- **Functions/methods:** snake_case — `from_path`, `from_bytes`, `to_bytes`, `read_bits`.
- **Constants:** SCREAMING_SNAKE_CASE — `HEADER_SIZE`, `COLONY_SIZE`, `UNIT_SIZE`, `FOOD`.
- **Enums:** PascalCase type + PascalCase variants — `TerrainType::Prairie`, `UnitType::Colonist`.
- **Modules:** snake_case, singular — `colony.rs`, `unit.rs`, `trade_route.rs`.
- **Type aliases:** PascalCase — `TileMap`, `MaskMap`, `PathMap`, `SeenMap`.

### Formatting

- `rustfmt` with default settings — run `cargo fmt --all` before committing.
- 4-space indentation (rustfmt default).
- No custom `rustfmt.toml` — standard Rust formatting.

### Type Handling & Binary Patterns

- **No type hints era — this is fully typed Rust.** All structs have explicit types.
- Binary data: `&[u8]` for input, `Vec<u8>` for output.
- Integer parsing: `u16::from_le_bytes`, `i32::from_le_bytes`, etc. — all SAV values are little-endian.
- Bit-level parsing: `BitReader`/`BitWriter` in `bits.rs` — MSB-first within each byte.
- Unknown/reserved fields: stored as raw `[u8; N]` arrays or `Vec<u8>` for round-trip fidelity.
- Name fields: stored as `[u8; N]` (not `String`) with `name() -> String` helper methods,
  because names can contain embedded nulls.
- Enum conversions: `sav_enum!` macro generates `TryFrom<u8>` (returns `Err(u8)` for unknown)
  and `From<Enum> for u8`.
- Generic goods arrays: `Goods16<T>` wraps `[T; 16]` with typed index access by goods constant.

### Error Handling

- Library crate: `SaveError` enum via `thiserror` — `InvalidMagic`, `UnexpectedEof`,
  `InvalidSize`, `Io`, `Other`.
- Library returns `crate::error::Result<T>` (type alias for `std::result::Result<T, SaveError>`).
- Binary crate: `anyhow::Result` for CLI error handling.
- Panics: only in `BitReader`/`BitWriter` assertions and `sav_enum!` TryFrom for truly
  impossible states. Production parsing uses `Result`.

### Struct Patterns

- Raw structs use `from_bytes(&[u8]) -> Self` for deserialization, `to_bytes() -> Vec<u8>`
  for serialization.
- Each raw struct has a size constant: `HEADER_SIZE`, `COLONY_SIZE`, `UNIT_SIZE`, etc.
  Some use `byte_size()` const fn instead.
- Bit-structs (GameOptions, Buildings, FoundingFathers, etc.) use `BitReader`/`BitWriter`
  for sub-byte field packing.
- All unused/unknown bit-fields are stored as raw values (not skipped) for round-trip fidelity.
- `Display` implementations produce multi-line human-readable dumps.
- `SaveFile` is the top-level container — holds all parsed sections plus raw bytes for
  unrecognized sections (`other`, `tail_fixed`, `trailing`).

---

## Testing

### Test Coverage (53 tests)

Tests are in-module `#[cfg(test)] mod tests` blocks within the library crate:

- **Round-trip tests**: Parse real SAV files → serialize → compare bytes (all 10 save files).
- **Raw parser round-trips**: Every module (header, colony, unit, nation, tribe, indian,
  trade_route, stuff, maps, player) has parse→serialize→compare tests.
- **Bit-struct round-trips**: GameOptions, ColonyReportOptions, EventFlags, Buildings,
  FoundingFathers, Relation, TribeBLCS, TribeMission, TribeFlags.
- **Enum conversions**: TryFrom valid/invalid values, NationId None handling, TerrainType
  all values, HillsRiver all values.
- **Goods module**: u8, u16, i16, i32, bool bitmap round-trips, indexing by constant.
- **Error cases**: bad magic detection, short data handling.

CLI integration tests are in `colsav/tests/cli.rs`:

- **Subcommand success**: Each of info, dump-units, dump-colonies, dump-nations, dump-map
  verified with exit code 0 and expected output fragments.
- **Edit round-trips**: Gold edit, tax edit verified by loading output file and checking values.
- **No-op edit**: Edit without -g/-t produces byte-identical output.
- **Error cases**: Missing input file and invalid power index return non-zero exit.

### Writing Tests

- Place tests in `#[cfg(test)] mod tests { ... }` at the bottom of each module.
- Use `use super::*;` to import the module's items.
- For round-trip tests: read a real SAV file from `../saves/`, parse, serialize, assert byte equality.
- Test names: `test_<thing>_<aspect>` — e.g., `test_unit_cargo_nibble_packing`.

---

## Key Architectural Decisions

### Two-Layer Model

Raw structs (`src/raw/`) hold the exact on-disk binary layout. No interpretation beyond
byte extraction. This ensures byte-exact round-trip: `from_bytes(data).to_bytes() == data`.

### Round-Trip Fidelity

The core design constraint. Every unknown byte, every reserved bit-field, every trailing
byte is preserved. The `SaveFile` struct stores `other`, `tail_fixed`, and `trailing` as
raw `Vec<u8>` for sections not yet fully decoded.

### Section Sizes (Verified Against Real Files)

| Section       | Bytes | Count Source                    |
|---------------|-------|--------------------------------|
| Header block  | 390   | Fixed (header 158 + players 208 + other 24) |
| Colony        | 202   | `header.colony_count`          |
| Unit          | 28    | `header.unit_count`            |
| Nation        | 316   | Fixed: 4                       |
| Tribe         | 18    | `header.tribe_count`           |
| Indian        | 78    | Fixed: 8                       |
| Stuff         | 727   | Fixed: 1                       |
| MapLayer      | W×H   | `header.map_width × header.map_height` |
| Connectivity  | W     | `header.map_width`             |
| Tail fixed    | 74    | Fixed                          |
| TradeRoute    | 74    | Fixed: 12                      |

### External Format References

- **Primary**: `/tmp/smcol_saves_utility/smcol_sav_struct.json` (pavelbel's detailed JSON schema)
- **Supplemental**: `/tmp/smcol_saves_utility/supplemental-info.md`
- **In-repo**: `Format.md` (original binary format notes)

---

## Key Warnings

- **`docs/gemini.md` is inaccurate.** It claims files are fixed 35,930 bytes (they're
  variable-length), gives wrong offsets, and misidentifies section sizes. Use pavelbel's
  JSON as the authoritative reference.
- **Name fields contain embedded nulls** (e.g., `Vlad\x00el De Ruyter`). Always use
  `[u8; N]` storage with `name()` helpers, never raw `String`.
- **Bit order is MSB-first** within each byte (big-endian bit order), matching pavelbel's
  JSON `bit_struct` convention. The `BitReader`/`BitWriter` in `bits.rs` handles this.
- **Header block is 390 bytes total**: header fields (158) + players (4×52=208) + other (24).
  These are packed contiguously, not separate sections.
- **The `supplies` / goods mapping** (16 goods types) appears in `goods.rs`, `enums.rs`,
  and across colony/unit/trade_route parsers. Keep them in sync.
- **Legacy Python code** (`colonization/` package, `*.py` scripts) is kept for reference
  but is not maintained. The Rust codebase is the active project.

---

## TUI Architecture

The TUI uses ratatui 0.29 with crossterm 0.28 backend:

- **Theme**: Authentic Norton Commander — VGA dark blue (`#0000AA`) background, cyan (`#00AAAA`) borders, light gray (`#AAAAAA`) text, yellow (`#FFFF55`) highlights. All colors use exact VGA 4-bit palette RGB values for consistent rendering across terminals.
- **Tabs**: Header, Colonies, Units, Nations, Trade Routes, Tribes, Map (7 tabs,
  switchable with Tab/Shift-Tab or number keys 1-7).
- **Editing**: Inline editing for gold and tax fields (Enter to edit, type value, Enter to confirm).
- **Saving**: `s` or `Ctrl+S` writes modified SAV file back to disk.
- **Navigation**: Arrow keys/hjkl for list scrolling, `q`/`Esc` to quit.
- **Help popup**: `?` key shows keybinding reference (dismiss with `?` or `Esc`).
- **State machine**: `InputMode` enum (Normal, Editing) + `show_help` flag controls input handling.
- **Unsaved changes**: Quit prompts for confirmation when dirty.

### Current TUI Limitations

- Colony detail view doesn't scroll for large colonies
- Map viewport doesn't support panning/zooming
- Trade routes and tribes are read-only (no inline editing yet)
