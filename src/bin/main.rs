use cs2_annotations::args;
use cs2_annotations::logging;
use cs2_annotations::map::MapAnnotation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::get();
    logging::init(&args)?;

    let map_annotation: MapAnnotation = MapAnnotation::read_annotations(args.map)?;
    println!("{:#?}", map_annotation);

    Ok(())
}
