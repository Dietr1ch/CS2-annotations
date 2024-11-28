use cs2_annotations::MapAnnotation;
use cs2_annotations::CSMap;

use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value_t = CSMap::Dust2)]
    pub map: CSMap,
}

fn setup_logger() -> Result<(), fern::InitError> {
    use std::time::SystemTime;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    setup_logger()?;

    let map_annotation: MapAnnotation = MapAnnotation::read_annotations(args.map)?;
    log::debug!("{:#?}", map_annotation);

    Ok(())
}
