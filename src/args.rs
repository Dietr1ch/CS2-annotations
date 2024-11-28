use clap::Parser;

use crate::map::CSMap;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub map: Option<CSMap>,

    #[arg(long, default_value_t = false)]
    pub check_all_maps: bool,

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
