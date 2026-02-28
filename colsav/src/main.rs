use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use colonization_sav::{HillsRiver, SaveFile, TerrainType};

const NATION_NAMES: [&str; 4] = ["England", "France", "Spain", "Netherlands"];

#[derive(Parser)]
#[command(name = "colsav", about = "Colonization SAV file editor")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Info {
        #[arg(short, long)]
        file: String,
    },
    DumpUnits {
        #[arg(short, long)]
        file: String,
    },
    DumpColonies {
        #[arg(short, long)]
        file: String,
    },
    DumpNations {
        #[arg(short, long)]
        file: String,
    },
    DumpMap {
        #[arg(short, long)]
        file: String,
    },
    Edit {
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        output: String,
        #[arg(short, long)]
        power: usize,
        #[arg(short, long)]
        gold: Option<i32>,
        #[arg(short, long)]
        tax: Option<u8>,
    },
}

fn terrain_base_char(terrain: TerrainType) -> char {
    match terrain {
        TerrainType::Ocean | TerrainType::SeaLane => ' ',
        TerrainType::Arctic => '#',
        TerrainType::Tundra => 't',
        TerrainType::Desert => 'd',
        TerrainType::Plains => 'p',
        TerrainType::Prairie => 'r',
        TerrainType::Grassland => 'g',
        TerrainType::Savannah => 's',
        TerrainType::Marsh => 'm',
        TerrainType::Swamp => 'w',
        TerrainType::TundraForest | TerrainType::TundraForestW => 'T',
        TerrainType::DesertForest | TerrainType::DesertForestW => 'D',
        TerrainType::PlainsForest | TerrainType::PlainsForestW => 'P',
        TerrainType::PrairieForest | TerrainType::PrairieForestW => 'R',
        TerrainType::GrasslandForest | TerrainType::GrasslandForestW => 'G',
        TerrainType::SavannahForest | TerrainType::SavannahForestW => 'S',
        TerrainType::MarshForest | TerrainType::MarshForestW => 'M',
        TerrainType::SwampForest | TerrainType::SwampForestW => 'W',
    }
}

fn terrain_char(tile: u8) -> char {
    let terrain_raw = (tile >> 3) & 0x1F;
    let hills_river_raw = tile & 0x07;

    let terrain = match TerrainType::try_from(terrain_raw) {
        Ok(v) => v,
        Err(_) => return '?',
    };

    if matches!(terrain, TerrainType::Ocean | TerrainType::SeaLane) {
        return ' ';
    }

    match HillsRiver::try_from(hills_river_raw) {
        Ok(HillsRiver::River | HillsRiver::MajorRiver | HillsRiver::RiverHills) => '~',
        Ok(HillsRiver::Mountains) => '^',
        Ok(HillsRiver::Hills) => 'h',
        _ => terrain_base_char(terrain),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { file } => {
            let save = SaveFile::from_path(file)?;
            print!("{}", save.header);
        }
        Commands::DumpUnits { file } => {
            let save = SaveFile::from_path(file)?;
            for (idx, unit) in save.units.iter().enumerate() {
                println!("Unit {}:", idx + 1);
                println!("{}", unit);
            }
        }
        Commands::DumpColonies { file } => {
            let save = SaveFile::from_path(file)?;
            for colony in &save.colonies {
                println!("{}", colony);
            }
        }
        Commands::DumpNations { file } => {
            let save = SaveFile::from_path(file)?;
            for (idx, nation) in save.nations.iter().enumerate() {
                let name = NATION_NAMES.get(idx).copied().unwrap_or("Unknown");
                println!("Nation: {name}");
                println!("{}", nation);
            }
        }
        Commands::DumpMap { file } => {
            let save = SaveFile::from_path(file)?;
            for row in 0..save.tile_map.rows {
                let mut line = String::with_capacity(save.tile_map.cols);
                for col in 0..save.tile_map.cols {
                    line.push(terrain_char(save.tile_map.get(row, col)));
                }
                println!("{line}");
            }
        }
        Commands::Edit {
            file,
            output,
            power,
            gold,
            tax,
        } => {
            if power >= NATION_NAMES.len() {
                bail!("Invalid power index {power}; expected 0..=3");
            }

            let mut save = SaveFile::from_path(file)?;
            let nation = &mut save.nations[power];

            if let Some(gold_value) = gold {
                nation.gold = gold_value;
            }
            if let Some(tax_value) = tax {
                nation.tax_rate = tax_value;
            }

            save.save(output)?;
        }
    }

    Ok(())
}
