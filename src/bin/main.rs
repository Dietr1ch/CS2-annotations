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
        let org = a.to_org().map_err(|e| {
            println!("{:?}", e);
            map::ParseError::Error
        })?;

        let mut writer = Vec::new();
        org.write_org(&mut writer).unwrap();
        let s = String::from_utf8(writer)?;
        println!("{}", s);
    }

    Ok(())
}
