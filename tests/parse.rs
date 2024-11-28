use cs2_annotations::map;

// TODO: Consider using https://docs.rs/assert_ok
macro_rules! assert_ok {
    ($expression:expr) => {
        match $expression {
            Ok(_) => (),
            ref e => panic!("Expected Ok(_) but got `{:?}`", e),
        }
    };
}

#[test]
fn parse_ancient() {
    assert_ok!(map::Annotation::read(map::Name::Ancient));
}

#[test]
fn parse_anubis() {
    assert_ok!(map::Annotation::read(map::Name::Anubis));
}

#[test]
fn parse_dust2() {
    assert_ok!(map::Annotation::read(map::Name::Dust2));
}

#[test]
fn parse_inferno() {
    assert_ok!(map::Annotation::read(map::Name::Inferno));
}

#[test]
fn parse_mirage() {
    assert_ok!(map::Annotation::read(map::Name::Mirage));
}

#[test]
fn parse_nuke() {
    assert_ok!(map::Annotation::read(map::Name::Nuke));
}

#[test]
fn parse_overpass() {
    assert_ok!(map::Annotation::read(map::Name::Overpass));
}

#[test]
fn parse_train() {
    assert_ok!(map::Annotation::read(map::Name::Train));
}

#[test]
fn parse_vertigo() {
    assert_ok!(map::Annotation::read(map::Name::Vertigo));
}
