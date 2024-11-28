use clap::Parser;

use crate::map::CSMap;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum, default_value_t = CSMap::Dust2)]
    pub map: CSMap,

    // Logging
    #[command(flatten)]
    pub color: concolor_clap::Color,
    #[arg(short, long)]
    pub verbosity: Option<log::LevelFilter>,
    #[arg(long)]
    pub log_verbosity: Option<log::LevelFilter>,
}

pub fn get() -> Args {
    Args::parse()
}
