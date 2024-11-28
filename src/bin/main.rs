use strum::IntoEnumIterator;

use cs2_annotations::args;
use cs2_annotations::logging;
use cs2_annotations::map::CSMap;
use cs2_annotations::map::MapAnnotation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::get();
    logging::init(&args)?;

    if args.check_all_maps {
        for m in CSMap::iter() {
            println!("Checking {:?}", m);
            let _map_annotation = MapAnnotation::read_annotations(m)?;
        }
        println!("All maps passed validation!");
    } else if let Some(map) = args.map {
        let map_annotation: MapAnnotation = MapAnnotation::read_annotations(map)?;
        println!("{:#?}", map_annotation);
    }

    Ok(())
}
