use strum_macros::EnumIter;

use kv3::kv3_serde::serde_kv3;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, EnumIter, clap::ValueEnum)]
pub enum CSMap {
    Ancient,
    Anubis,
    #[default]
    Dust2,
    Inferno,
    Mirage,
    Nuke,
    Overpass,
    Train,
    Vertigo,
}

impl CSMap {
    pub fn file_name(&self) -> &'static str {
        match &self {
            CSMap::Ancient => "de_ancient.txt",
            CSMap::Anubis => "de_anubis.txt",
            CSMap::Dust2 => "de_dust2.txt",
            CSMap::Inferno => "de_inferno.txt",
            CSMap::Mirage => "de_mirage.txt",
            CSMap::Nuke => "de_nuke.txt",
            CSMap::Overpass => "de_overpass.txt",
            CSMap::Train => "de_train.txt",
            CSMap::Vertigo => "de_vertigo.txt",
        }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Text {}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Title {
    // Text = ""
    #[serde(default)]
    pub text: String,
    // FontSize = 75
    #[serde(default)]
    pub font_size: u32,
    // FadeInDist = 300.0
    #[serde(default)]
    pub fade_in_dist: f32,
    // FadeOutDist = 40.0
    #[serde(default)]
    pub fade_out_dist: f32,
}

// Currently the same as Title
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Description {
    // Text = ""
    #[serde(default)]
    pub text: String,
    // FontSize = 75
    #[serde(default)]
    pub font_size: u32,
    // FadeInDist = 300.0
    #[serde(default)]
    pub fade_in_dist: f32,
    // FadeOutDist = 40.0
    #[serde(default)]
    pub fade_out_dist: f32,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    // Enabled = true
    #[serde(default)]
    pub enabled: bool,
    // Type = "grenade"
    #[serde(default, rename = "Type")]
    pub r#type: String,
    // Id = "860dd8f2-cc23-432e-81a7-03ebd7956940"
    #[serde(default)]
    pub id: String,
    // SubType = "main"
    #[serde(default)]
    pub sub_type: String,
    // TODO: Wrap in Position struct
    // Position = [ -1332.026978, 491.96875, 8.03125 ]
    #[serde(default)]
    pub position: [f64; 3],
    // TODO: Wrap in Angle struct
    // Angles = [ 0.0, 87.922913, 0.0 ]
    #[serde(default)]
    pub angles: [f64; 3],
    // VisiblePfx = true
    #[serde(default)]
    pub visible_pfx: bool,
    // Color = [ 234, 191, 86 ]
    #[serde(default)]
    pub color: [u8; 3],
    // TextPositionOffset = [ 0.0, 0.0, 80.0 ]
    #[serde(default)]
    pub text_position_offset: [f32; 3],

    // TextFacePlayer = true
    #[serde(default)]
    pub text_face_player: bool,
    // TODO: Make text_horizontal_align an enum
    // TextHorizontalAlign = "center"
    #[serde(default)]
    pub text_horizontal_align: String,

    // RevealOnSuccess = false
    #[serde(default)]
    pub reveal_on_success: bool,

    #[serde(default)]
    pub title: Title,
    #[serde(default, rename = "Desc")]
    pub description: Description,

    // StreakLimitGuidesOn = 2
    #[serde(default)]
    pub streak_limit_guides_on: u8,
    // StreakLimitGuidesOff = 2
    #[serde(default)]
    pub streak_limit_guides_off: u8,
    // JumpThrow = false
    #[serde(default)]
    pub jump_throw: bool,
    // MasterNodeId = "4f2f0b14-3d59-4c52-bccf-6530ec0a5af4"
    #[serde(default)]
    pub master_node_id: String,
    // DistanceThreshold = 80.0
    #[serde(default)]
    pub distance_threshold: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct SillyMapAnnotation {
    #[serde(default)]
    map_name: String,
    #[serde(default)]
    screen_text: Text,

    // Spiritually simply,
    // annotations: Vec<Node>,

    // WHY VALVE?? WHY? Didn't KeyValues3 already support arrays?!
    #[serde(default)]
    map_annotation_node0: Node,
    #[serde(default)]
    map_annotation_node1: Node,
    #[serde(default)]
    map_annotation_node2: Node,
    #[serde(default)]
    map_annotation_node3: Node,
    #[serde(default)]
    map_annotation_node4: Node,
    #[serde(default)]
    map_annotation_node5: Node,
    #[serde(default)]
    map_annotation_node6: Node,
    #[serde(default)]
    map_annotation_node7: Node,
    #[serde(default)]
    map_annotation_node8: Node,
    #[serde(default)]
    map_annotation_node9: Node,
    #[serde(default)]
    map_annotation_node10: Node,
    #[serde(default)]
    map_annotation_node11: Node,
    #[serde(default)]
    map_annotation_node12: Node,
    #[serde(default)]
    map_annotation_node13: Node,
    #[serde(default)]
    map_annotation_node14: Node,
    #[serde(default)]
    map_annotation_node15: Node,
    #[serde(default)]
    map_annotation_node16: Node,
    #[serde(default)]
    map_annotation_node17: Node,
    #[serde(default)]
    map_annotation_node18: Node,
    #[serde(default)]
    map_annotation_node19: Node,
    #[serde(default)]
    map_annotation_node20: Node,
    #[serde(default)]
    map_annotation_node21: Node,
    #[serde(default)]
    map_annotation_node22: Node,
    #[serde(default)]
    map_annotation_node23: Node,
    #[serde(default)]
    map_annotation_node24: Node,
    #[serde(default)]
    map_annotation_node25: Node,
    #[serde(default)]
    map_annotation_node26: Node,
    #[serde(default)]
    map_annotation_node27: Node,
    #[serde(default)]
    map_annotation_node28: Node,
    #[serde(default)]
    map_annotation_node29: Node,
    #[serde(default)]
    map_annotation_node30: Node,
    #[serde(default)]
    map_annotation_node31: Node,
    #[serde(default)]
    map_annotation_node32: Node,
    #[serde(default)]
    map_annotation_node33: Node,
    #[serde(default)]
    map_annotation_node34: Node,
    #[serde(default)]
    map_annotation_node35: Node,
    #[serde(default)]
    map_annotation_node36: Node,
    #[serde(default)]
    map_annotation_node37: Node,
    #[serde(default)]
    map_annotation_node38: Node,
    #[serde(default)]
    map_annotation_node39: Node,
    #[serde(default)]
    map_annotation_node40: Node,
    #[serde(default)]
    map_annotation_node41: Node,
    #[serde(default)]
    map_annotation_node42: Node,
    #[serde(default)]
    map_annotation_node43: Node,
    #[serde(default)]
    map_annotation_node44: Node,
    #[serde(default)]
    map_annotation_node45: Node,
    #[serde(default)]
    map_annotation_node46: Node,
    #[serde(default)]
    map_annotation_node47: Node,
    #[serde(default)]
    map_annotation_node48: Node,
    #[serde(default)]
    map_annotation_node49: Node,
    #[serde(default)]
    map_annotation_node50: Node,
    #[serde(default)]
    map_annotation_node51: Node,
    #[serde(default)]
    map_annotation_node52: Node,
    #[serde(default)]
    map_annotation_node53: Node,
    #[serde(default)]
    map_annotation_node54: Node,
    #[serde(default)]
    map_annotation_node55: Node,
    #[serde(default)]
    map_annotation_node56: Node,
    #[serde(default)]
    map_annotation_node57: Node,
    #[serde(default)]
    map_annotation_node58: Node,
    #[serde(default)]
    map_annotation_node59: Node,
    #[serde(default)]
    map_annotation_node60: Node,
    #[serde(default)]
    map_annotation_node61: Node,
    #[serde(default)]
    map_annotation_node62: Node,
    #[serde(default)]
    map_annotation_node63: Node,
    #[serde(default)]
    map_annotation_node64: Node,
    #[serde(default)]
    map_annotation_node65: Node,
    #[serde(default)]
    map_annotation_node66: Node,
    #[serde(default)]
    map_annotation_node67: Node,
    #[serde(default)]
    map_annotation_node68: Node,
    #[serde(default)]
    map_annotation_node69: Node,
    #[serde(default)]
    map_annotation_node70: Node,
    #[serde(default)]
    map_annotation_node71: Node,
    #[serde(default)]
    map_annotation_node72: Node,
    #[serde(default)]
    map_annotation_node73: Node,
    #[serde(default)]
    map_annotation_node74: Node,
    #[serde(default)]
    map_annotation_node75: Node,
    #[serde(default)]
    map_annotation_node76: Node,
    #[serde(default)]
    map_annotation_node77: Node,
    #[serde(default)]
    map_annotation_node78: Node,
    #[serde(default)]
    map_annotation_node79: Node,
    #[serde(default)]
    map_annotation_node80: Node,
    #[serde(default)]
    map_annotation_node81: Node,
    #[serde(default)]
    map_annotation_node82: Node,
    #[serde(default)]
    map_annotation_node83: Node,
    #[serde(default)]
    map_annotation_node84: Node,
    #[serde(default)]
    map_annotation_node85: Node,
    #[serde(default)]
    map_annotation_node86: Node,
    #[serde(default)]
    map_annotation_node87: Node,
    #[serde(default)]
    map_annotation_node88: Node,
    #[serde(default)]
    map_annotation_node89: Node,
    #[serde(default)]
    map_annotation_node90: Node,
    #[serde(default)]
    map_annotation_node91: Node,
    #[serde(default)]
    map_annotation_node92: Node,
    #[serde(default)]
    map_annotation_node93: Node,
    #[serde(default)]
    map_annotation_node94: Node,
    #[serde(default)]
    map_annotation_node95: Node,
    #[serde(default)]
    map_annotation_node96: Node,
    #[serde(default)]
    map_annotation_node97: Node,
    #[serde(default)]
    map_annotation_node98: Node,
    #[serde(default)]
    map_annotation_node99: Node,
    // XXX: There's up to 100 nodes
    // TODO: Fix code repetition
}

#[derive(Debug, strum_macros::Display, thiserror::Error)]
pub enum MapAnnotationError {
    ParseError,
    ParseErrorOnFile(&'static str),
}

#[derive(Debug, Default)]
pub struct MapAnnotation {
    pub map_name: String,
    pub screen_text: Text,

    pub nodes: Vec<Node>,
}

impl MapAnnotation {
    pub fn read_annotations(map: CSMap) -> Result<Self, MapAnnotationError> {
        let file_name: &str = map.file_name();
        let text = std::fs::read_to_string(file_name)
            .expect("Failed to read map annotations file")
            .replace("\r\n", "\n");

        MapAnnotation::try_from(text.as_ref())
            .map_err(|_| MapAnnotationError::ParseErrorOnFile(file_name))
    }
}

impl TryFrom<&str> for MapAnnotation {
    type Error = MapAnnotationError;

    fn try_from(s: &str) -> Result<MapAnnotation, MapAnnotationError> {
        let s: SillyMapAnnotation =
            serde_kv3(s).map_err(|_| MapAnnotationError::ParseError)?;
        Ok(MapAnnotation::from(s))
    }
}

impl From<SillyMapAnnotation> for MapAnnotation {
    fn from(r: SillyMapAnnotation) -> Self {
        let empty_node = Node::default();
        let mut nodes = Vec::<Node>::new();

        let node = r.map_annotation_node0;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node1;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node2;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node3;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node4;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node5;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node6;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node7;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node8;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node9;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node10;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node11;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node12;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node13;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node14;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node15;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node16;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node17;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node18;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node19;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node20;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node21;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node22;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node23;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node24;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node25;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node26;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node27;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node28;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node29;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node30;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node31;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node32;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node33;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node34;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node35;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node36;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node37;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node38;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node39;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node40;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node41;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node42;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node43;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node44;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node45;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node46;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node47;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node48;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node49;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node50;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node51;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node52;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node53;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node54;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node55;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node56;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node57;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node58;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node59;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node60;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node61;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node62;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node63;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node64;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node65;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node66;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node67;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node68;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node69;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node70;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node71;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node72;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node73;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node74;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node75;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node76;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node77;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node78;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node79;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node80;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node81;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node82;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node83;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node84;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node85;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node86;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node87;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node88;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node89;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node90;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node91;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node92;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node93;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node94;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node95;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node96;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node97;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node98;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node99;
        if node != empty_node {
            nodes.push(node);
        }
        // XXX: There's up to 100 nodes

        MapAnnotation {
            map_name: r.map_name,
            screen_text: r.screen_text,
            nodes,
        }
    }
}
