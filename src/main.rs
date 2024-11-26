use cs2_annotations::MapAnnotation;

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

use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = "de_dust_2.txt")]
    pub file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    setup_logger()?;

    let map_annotations_str: String =
        std::fs::read_to_string(path).expect("Should have been able to read the file");

    match MapAnnotation::try_from(map_annotations_str) {
        Ok(map_annotation) => {
            log::info!("Yay!");
            log::debug!("{:#?}", map_annotation);
        }
        Err(e) => {
            log::error!("error {:?}", e);
            panic!("expected to pass the test")
        }
    }

    Ok(())
}
