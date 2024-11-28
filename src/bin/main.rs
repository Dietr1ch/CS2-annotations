use strum::IntoEnumIterator;

use cs2_annotations::args;
use cs2_annotations::logging;
use cs2_annotations::map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::get();
    logging::init(&args)?;

    if args.check_all_maps {
        for m in map::Name::iter() {
            println!("Checking {:?}", m);
            let map_annotation = map::Annotation::read_annotations(m)?;
            println!("Checking {:#?}", map_annotation);
        }
        println!("All maps passed validation!");
    } else if let Some(map) = args.map {
        let map_annotation: map::Annotation = map::Annotation::read_annotations(map)?;
        println!("{:#?}", map_annotation);
    }

    Ok(())
}
