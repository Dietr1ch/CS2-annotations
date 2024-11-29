use strum::IntoEnumIterator;

use cs2_annotations::args;
use cs2_annotations::logging;
use cs2_annotations::map;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::get();
    logging::init(&args)?;

    if args.check_all_maps {
        for map in map::Name::iter() {
            println!("Checking {:?}", map);
            println!("{:#?}", map::Annotation::read(map)?);
        }
        println!("All maps passed validation!");
    } else if let Some(map) = args.map {
        let a = map::Annotation::read(map)?;
        let org_str = a.to_org_string().map_err(|e| {
            println!("{:?}", e);
            map::ParseError::Error
        })?;


        println!("Writing '{}'", a.name.org_name());
        std::fs::write(a.name.org_name(), org_str)?;
    }

    Ok(())
}
