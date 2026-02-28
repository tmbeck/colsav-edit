# AGENTS.md — Colonization SAV File Tools

## Project Overview

Python toolkit for reading, editing, and analyzing Colonization 3.0 save game files (.SAV).
The `colonization` package parses binary SAV file structures (header, colonies, units, powers,
villages, maps, trade routes) into Python objects. CLI scripts dump or modify game data.

**Python version:** 3.6+ (uses f-strings, `int.from_bytes`; no type hints, no async).
**No external dependencies.** Standard library only (`os`, `sys`, `argparse`, `binascii`, `string`).
**No package manager config** — no `setup.py`, `pyproject.toml`, `requirements.txt`, or `Pipfile`.
**License:** Unlicense (public domain).

---

## Repository Structure

```
colonization/           # Core library package
  __init__.py           # Re-exports: Map, Tile, Unit, Colonist, TradeRoute, Destination,
                        #   Village, Colony, Power, SaveFileWriter, SaveFile, Header
  header.py             # SaveFile, SaveFileWriter, Header — file I/O and offset computation
  buildings.py          # Colony, Village, OldColony (legacy) — colony/village parsing
  units.py              # Unit, Colonist — unit parsing and cargo decoding
  powers.py             # Power — gold, taxes, serialization
  map.py                # Map, Tile — terrain map parsing and ASCII rendering
  trade.py              # TradeRoute, Destination — trade route parsing
colmapplotter.py        # CLI: display map views from a SAV file
dump_colonies.py        # CLI: dump colony data
dump_powers.py          # CLI: dump power data
dump_units.py           # CLI: dump unit data
hex_compare.py          # CLI: compare hex diffs between two SAV files
edit.py                 # CLI: modify power gold/taxes and write new SAV file
ALLTERRA.MP             # Sample map file for testing
Format.md               # Binary format documentation for the SAV file structure
```

---

## Build / Run / Test Commands

There is **no build system, no test suite, no linter, and no formatter** configured.

### Running scripts

All CLI scripts accept `-f FILE` or `-d DIRECTORY -s SLOT`:

```bash
# Dump units from a specific file
python3 dump_units.py -f /path/to/COLONY00.SAV

# Dump colonies from slot 0 in a directory
python3 dump_colonies.py -d /path/to/saves/ -s 0

# Dump powers
python3 dump_powers.py -f /path/to/COLONY00.SAV

# Display map
python3 colmapplotter.py -f /path/to/COLONY00.SAV

# Compare two save files (by slot number, requires -d)
python3 hex_compare.py -d /path/to/saves/ 0 1

# Edit a power's gold (requires -o output, -p power index 0-3)
python3 edit.py -f /path/to/COLONY00.SAV -o output.SAV -p 0 -g 500000
```

### Using the library

```python
import colonization as col

save = col.SaveFile("/path/to/COLONY00.SAV")
print(save.header.colony_count)
for unit in save.units:
    print(unit)
for colony in save.colonies:
    print(colony)
```

### Quick validation (no formal tests)

```bash
# Syntax check all Python files
python3 -m py_compile colonization/__init__.py
python3 -m py_compile colonization/header.py
python3 -m py_compile colonization/buildings.py
python3 -m py_compile colonization/units.py
python3 -m py_compile colonization/powers.py
python3 -m py_compile colonization/map.py
python3 -m py_compile colonization/trade.py

# Or check all at once
python3 -m py_compile edit.py dump_units.py dump_colonies.py dump_powers.py colmapplotter.py hex_compare.py
```

---

## Code Style Guidelines

### Imports

- Standard library first (`os`, `sys`, `argparse`), then blank line, then `import colonization as col`.
- Inside the `colonization` package, use relative imports: `from .units import Colonist, Unit`.
- The `__init__.py` re-exports all public classes — external code uses `import colonization as col`
  and accesses `col.SaveFile`, `col.Unit`, etc.
- `colonization` aliased as `col` in scripts and internally in `header.py`.

### Naming Conventions

- **Classes:** PascalCase — `SaveFile`, `TradeRoute`, `Colony`, `Unit`, `Power`.
- **Functions/methods:** snake_case — `check_args`, `dump_units`, `display_map`.
- **Private methods:** double-underscore prefix (name mangling) — `__parse`, `__reader`.
- **Constants:** ALL_CAPS for module-level — `TERRAIN = 0`.
- **Class attributes:** snake_case — `byte_length`, `base_offset`, `gold_min`.
- **Lookup dicts:** named by category — `orders`, `powers`, `supplies`, `forms`, `buildings`.
- **Files:** snake_case for scripts, singular nouns for modules (`units.py`, not `unit.py`).

### Formatting

- 4-space indentation, no tabs.
- No formatter or linter configured — follow existing file style.
- Hex constants: lowercase `0x` prefix for addresses (`0x186`), uppercase `0X` sometimes for
  building/constructable offsets (inconsistent — prefer lowercase `0x`).
- f-strings for all string formatting (no `.format()` except in `__str__` methods where
  `.ljust()` / `.rjust()` alignment is needed).
- Trailing whitespace exists in some files — not enforced.

### Type Handling

- **No type hints** anywhere. This is a Python 3.6-era codebase.
- Do not add type hints unless explicitly asked.
- Binary data handled as `bytes` (read) and `bytearray` (write/modify).
- Integer parsing: `int.from_bytes(data[start:end], 'little')` — all values are little-endian.
- Reverse lookups: `{val: key for key, val in SomeClass.dict.items()}` pattern used heavily.

### Error Handling

- `ValueError` for invalid data lengths, out-of-range values, invalid arguments.
- `FileNotFoundError` for missing files/directories.
- `FileExistsError` for refusing to overwrite (in `SaveFile.save_data`).
- `Exception` with descriptive message for unrecognized file types.
- CLI scripts: wrap `check_args()` in try/except, print error, `sys.exit(1)`.
- Some classes raise bare `ValueError` (no message) on length mismatch — this is existing style.

### Class Patterns

- Game data classes use class-level `byte_length` for struct size.
- Lookup dicts are class-level constants: `powers`, `supplies`, `orders`, `forms`, `buildings`.
- Unknown byte ranges tracked as `unknowns` list of `(start, end)` tuples.
- `__str__` methods produce multi-line human-readable dumps.
- `unpack(data)` / `__init__(data)` for deserialization; `serialize()` / `pack()` for writing back.
- `OldColony` in `buildings.py` is a legacy version of `Colony` — uses `unpack()` instead of
  `__init__` for parsing. New code should follow `Colony`'s pattern (parse in `__init__`).

### Binary Format Notes

- SAV files start with `b'COLONIZE\0'` magic marker.
- All multi-byte integers are little-endian.
- Strings are 24-byte null-terminated ASCII.
- Section offsets are computed dynamically from object counts in the header.
- See `Format.md` for full binary structure documentation.

---

## Key Warnings

- **No tests exist.** Validate changes manually against a real SAV file.
- **Debug prints** are scattered through production code (`print()` in `SaveFile.__parse`,
  `Power.serialize`). These are intentional for reverse-engineering work.
- The `supplies` dict is duplicated across `Unit`, `Village`, `Colony`, and `TradeRoute`.
  Keep them in sync if modifying supply mappings.
- `header.py` has a bug: `self.villages_start_address` uses `Village.byte_length` instead of
  `Power.byte_length` in the powers offset calculation. Be aware when working with addresses.
