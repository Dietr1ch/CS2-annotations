use std::collections::HashSet;

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

    #[serde(default)]
    map_annotation_node100: Node,
    #[serde(default)]
    map_annotation_node101: Node,
    #[serde(default)]
    map_annotation_node102: Node,
    #[serde(default)]
    map_annotation_node103: Node,
    #[serde(default)]
    map_annotation_node104: Node,
    #[serde(default)]
    map_annotation_node105: Node,
    #[serde(default)]
    map_annotation_node106: Node,
    #[serde(default)]
    map_annotation_node107: Node,
    #[serde(default)]
    map_annotation_node108: Node,
    #[serde(default)]
    map_annotation_node109: Node,
    #[serde(default)]
    map_annotation_node110: Node,
    #[serde(default)]
    map_annotation_node111: Node,
    #[serde(default)]
    map_annotation_node112: Node,
    #[serde(default)]
    map_annotation_node113: Node,
    #[serde(default)]
    map_annotation_node114: Node,
    #[serde(default)]
    map_annotation_node115: Node,
    #[serde(default)]
    map_annotation_node116: Node,
    #[serde(default)]
    map_annotation_node117: Node,
    #[serde(default)]
    map_annotation_node118: Node,
    #[serde(default)]
    map_annotation_node119: Node,
    #[serde(default)]
    map_annotation_node120: Node,
    #[serde(default)]
    map_annotation_node121: Node,
    #[serde(default)]
    map_annotation_node122: Node,
    #[serde(default)]
    map_annotation_node123: Node,
    #[serde(default)]
    map_annotation_node124: Node,
    #[serde(default)]
    map_annotation_node125: Node,
    #[serde(default)]
    map_annotation_node126: Node,
    #[serde(default)]
    map_annotation_node127: Node,
    #[serde(default)]
    map_annotation_node128: Node,
    #[serde(default)]
    map_annotation_node129: Node,
    #[serde(default)]
    map_annotation_node130: Node,
    #[serde(default)]
    map_annotation_node131: Node,
    #[serde(default)]
    map_annotation_node132: Node,
    #[serde(default)]
    map_annotation_node133: Node,
    #[serde(default)]
    map_annotation_node134: Node,
    #[serde(default)]
    map_annotation_node135: Node,
    #[serde(default)]
    map_annotation_node136: Node,
    #[serde(default)]
    map_annotation_node137: Node,
    #[serde(default)]
    map_annotation_node138: Node,
    #[serde(default)]
    map_annotation_node139: Node,
    #[serde(default)]
    map_annotation_node140: Node,
    #[serde(default)]
    map_annotation_node141: Node,
    #[serde(default)]
    map_annotation_node142: Node,
    #[serde(default)]
    map_annotation_node143: Node,
    #[serde(default)]
    map_annotation_node144: Node,
    #[serde(default)]
    map_annotation_node145: Node,
    #[serde(default)]
    map_annotation_node146: Node,
    #[serde(default)]
    map_annotation_node147: Node,
    #[serde(default)]
    map_annotation_node148: Node,
    #[serde(default)]
    map_annotation_node149: Node,
    #[serde(default)]
    map_annotation_node150: Node,
    #[serde(default)]
    map_annotation_node151: Node,
    #[serde(default)]
    map_annotation_node152: Node,
    #[serde(default)]
    map_annotation_node153: Node,
    #[serde(default)]
    map_annotation_node154: Node,
    #[serde(default)]
    map_annotation_node155: Node,
    #[serde(default)]
    map_annotation_node156: Node,
    #[serde(default)]
    map_annotation_node157: Node,
    #[serde(default)]
    map_annotation_node158: Node,
    #[serde(default)]
    map_annotation_node159: Node,
    #[serde(default)]
    map_annotation_node160: Node,
    #[serde(default)]
    map_annotation_node161: Node,
    #[serde(default)]
    map_annotation_node162: Node,
    #[serde(default)]
    map_annotation_node163: Node,
    #[serde(default)]
    map_annotation_node164: Node,
    #[serde(default)]
    map_annotation_node165: Node,
    #[serde(default)]
    map_annotation_node166: Node,
    #[serde(default)]
    map_annotation_node167: Node,
    #[serde(default)]
    map_annotation_node168: Node,
    #[serde(default)]
    map_annotation_node169: Node,
    #[serde(default)]
    map_annotation_node170: Node,
    #[serde(default)]
    map_annotation_node171: Node,
    #[serde(default)]
    map_annotation_node172: Node,
    #[serde(default)]
    map_annotation_node173: Node,
    #[serde(default)]
    map_annotation_node174: Node,
    #[serde(default)]
    map_annotation_node175: Node,
    #[serde(default)]
    map_annotation_node176: Node,
    #[serde(default)]
    map_annotation_node177: Node,
    #[serde(default)]
    map_annotation_node178: Node,
    #[serde(default)]
    map_annotation_node179: Node,
    #[serde(default)]
    map_annotation_node180: Node,
    #[serde(default)]
    map_annotation_node181: Node,
    #[serde(default)]
    map_annotation_node182: Node,
    #[serde(default)]
    map_annotation_node183: Node,
    #[serde(default)]
    map_annotation_node184: Node,
    #[serde(default)]
    map_annotation_node185: Node,
    #[serde(default)]
    map_annotation_node186: Node,
    #[serde(default)]
    map_annotation_node187: Node,
    #[serde(default)]
    map_annotation_node188: Node,
    #[serde(default)]
    map_annotation_node189: Node,
    #[serde(default)]
    map_annotation_node190: Node,
    #[serde(default)]
    map_annotation_node191: Node,
    #[serde(default)]
    map_annotation_node192: Node,
    #[serde(default)]
    map_annotation_node193: Node,
    #[serde(default)]
    map_annotation_node194: Node,
    #[serde(default)]
    map_annotation_node195: Node,
    #[serde(default)]
    map_annotation_node196: Node,
    #[serde(default)]
    map_annotation_node197: Node,
    #[serde(default)]
    map_annotation_node198: Node,
    #[serde(default)]
    map_annotation_node199: Node,

    #[serde(default)]
    map_annotation_node200: Node,
    #[serde(default)]
    map_annotation_node201: Node,
    #[serde(default)]
    map_annotation_node202: Node,
    #[serde(default)]
    map_annotation_node203: Node,
    #[serde(default)]
    map_annotation_node204: Node,
    #[serde(default)]
    map_annotation_node205: Node,
    #[serde(default)]
    map_annotation_node206: Node,
    #[serde(default)]
    map_annotation_node207: Node,
    #[serde(default)]
    map_annotation_node208: Node,
    #[serde(default)]
    map_annotation_node209: Node,
    #[serde(default)]
    map_annotation_node210: Node,
    #[serde(default)]
    map_annotation_node211: Node,
    #[serde(default)]
    map_annotation_node212: Node,
    #[serde(default)]
    map_annotation_node213: Node,
    #[serde(default)]
    map_annotation_node214: Node,
    #[serde(default)]
    map_annotation_node215: Node,
    #[serde(default)]
    map_annotation_node216: Node,
    #[serde(default)]
    map_annotation_node217: Node,
    #[serde(default)]
    map_annotation_node218: Node,
    #[serde(default)]
    map_annotation_node219: Node,
    #[serde(default)]
    map_annotation_node220: Node,
    #[serde(default)]
    map_annotation_node221: Node,
    #[serde(default)]
    map_annotation_node222: Node,
    #[serde(default)]
    map_annotation_node223: Node,
    #[serde(default)]
    map_annotation_node224: Node,
    #[serde(default)]
    map_annotation_node225: Node,
    #[serde(default)]
    map_annotation_node226: Node,
    #[serde(default)]
    map_annotation_node227: Node,
    #[serde(default)]
    map_annotation_node228: Node,
    #[serde(default)]
    map_annotation_node229: Node,
    #[serde(default)]
    map_annotation_node230: Node,
    #[serde(default)]
    map_annotation_node231: Node,
    #[serde(default)]
    map_annotation_node232: Node,
    #[serde(default)]
    map_annotation_node233: Node,
    #[serde(default)]
    map_annotation_node234: Node,
    #[serde(default)]
    map_annotation_node235: Node,
    #[serde(default)]
    map_annotation_node236: Node,
    #[serde(default)]
    map_annotation_node237: Node,
    #[serde(default)]
    map_annotation_node238: Node,
    #[serde(default)]
    map_annotation_node239: Node,
    #[serde(default)]
    map_annotation_node240: Node,
    #[serde(default)]
    map_annotation_node241: Node,
    #[serde(default)]
    map_annotation_node242: Node,
    #[serde(default)]
    map_annotation_node243: Node,
    #[serde(default)]
    map_annotation_node244: Node,
    #[serde(default)]
    map_annotation_node245: Node,
    #[serde(default)]
    map_annotation_node246: Node,
    #[serde(default)]
    map_annotation_node247: Node,
    #[serde(default)]
    map_annotation_node248: Node,
    #[serde(default)]
    map_annotation_node249: Node,
    #[serde(default)]
    map_annotation_node250: Node,
    #[serde(default)]
    map_annotation_node251: Node,
    #[serde(default)]
    map_annotation_node252: Node,
    #[serde(default)]
    map_annotation_node253: Node,
    #[serde(default)]
    map_annotation_node254: Node,
    #[serde(default)]
    map_annotation_node255: Node,
    #[serde(default)]
    map_annotation_node256: Node,
    #[serde(default)]
    map_annotation_node257: Node,
    #[serde(default)]
    map_annotation_node258: Node,
    #[serde(default)]
    map_annotation_node259: Node,
    #[serde(default)]
    map_annotation_node260: Node,
    #[serde(default)]
    map_annotation_node261: Node,
    #[serde(default)]
    map_annotation_node262: Node,
    #[serde(default)]
    map_annotation_node263: Node,
    #[serde(default)]
    map_annotation_node264: Node,
    #[serde(default)]
    map_annotation_node265: Node,
    #[serde(default)]
    map_annotation_node266: Node,
    #[serde(default)]
    map_annotation_node267: Node,
    #[serde(default)]
    map_annotation_node268: Node,
    #[serde(default)]
    map_annotation_node269: Node,
    #[serde(default)]
    map_annotation_node270: Node,
    #[serde(default)]
    map_annotation_node271: Node,
    #[serde(default)]
    map_annotation_node272: Node,
    #[serde(default)]
    map_annotation_node273: Node,
    #[serde(default)]
    map_annotation_node274: Node,
    #[serde(default)]
    map_annotation_node275: Node,
    #[serde(default)]
    map_annotation_node276: Node,
    #[serde(default)]
    map_annotation_node277: Node,
    #[serde(default)]
    map_annotation_node278: Node,
    #[serde(default)]
    map_annotation_node279: Node,
    #[serde(default)]
    map_annotation_node280: Node,
    #[serde(default)]
    map_annotation_node281: Node,
    #[serde(default)]
    map_annotation_node282: Node,
    #[serde(default)]
    map_annotation_node283: Node,
    #[serde(default)]
    map_annotation_node284: Node,
    #[serde(default)]
    map_annotation_node285: Node,
    #[serde(default)]
    map_annotation_node286: Node,
    #[serde(default)]
    map_annotation_node287: Node,
    #[serde(default)]
    map_annotation_node288: Node,
    #[serde(default)]
    map_annotation_node289: Node,
    #[serde(default)]
    map_annotation_node290: Node,
    #[serde(default)]
    map_annotation_node291: Node,
    #[serde(default)]
    map_annotation_node292: Node,
    #[serde(default)]
    map_annotation_node293: Node,
    #[serde(default)]
    map_annotation_node294: Node,
    #[serde(default)]
    map_annotation_node295: Node,
    #[serde(default)]
    map_annotation_node296: Node,
    #[serde(default)]
    map_annotation_node297: Node,
    #[serde(default)]
    map_annotation_node298: Node,
    #[serde(default)]
    map_annotation_node299: Node,
    // XXX: There's up to 300 nodes
    // https://store.steampowered.com/news/app/730/view/4472733036435210471
    // TODO: Fix code repetition
}

#[derive(Debug, strum_macros::Display, thiserror::Error)]
pub enum MapAnnotationError {
    ParseError,
    ParseErrorOnFile(&'static str),
    DuplicateId,
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

        match MapAnnotation::try_from(text.as_ref()) {
            Err(MapAnnotationError::ParseError) => Err(MapAnnotationError::ParseErrorOnFile(file_name)),
            annotation => annotation,
        }
    }

    fn verify(&self) -> Result<(), MapAnnotationError> {
        let mut node_ids = HashSet::<String>::new();
        let mut missing_ids = 0usize;
        for n in self.nodes.iter() {
            if n.id.is_empty() {
                missing_ids += 1;
                continue;
            }
            if !node_ids.insert(n.id.clone()) {
                // Id already existed
                return Err(MapAnnotationError::DuplicateId);
            }
        }

        assert_eq!(node_ids.len() + missing_ids, self.nodes.len());

        Ok(())
    }
}

impl TryFrom<&str> for MapAnnotation {
    type Error = MapAnnotationError;

    fn try_from(s: &str) -> Result<MapAnnotation, MapAnnotationError> {
        let s: SillyMapAnnotation = serde_kv3(s).map_err(|_| MapAnnotationError::ParseError)?;
        let map_annotation = MapAnnotation::from(s);
        map_annotation.verify()?;
        Ok(map_annotation)
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

        let node = r.map_annotation_node100;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node101;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node102;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node103;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node104;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node105;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node106;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node107;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node108;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node109;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node110;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node111;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node112;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node113;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node114;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node115;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node116;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node117;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node118;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node119;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node120;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node121;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node122;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node123;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node124;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node125;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node126;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node127;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node128;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node129;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node130;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node131;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node132;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node133;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node134;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node135;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node136;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node137;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node138;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node139;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node140;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node141;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node142;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node143;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node144;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node145;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node146;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node147;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node148;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node149;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node150;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node151;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node152;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node153;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node154;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node155;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node156;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node157;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node158;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node159;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node160;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node161;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node162;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node163;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node164;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node165;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node166;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node167;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node168;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node169;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node170;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node171;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node172;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node173;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node174;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node175;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node176;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node177;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node178;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node179;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node180;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node181;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node182;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node183;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node184;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node185;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node186;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node187;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node188;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node189;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node190;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node191;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node192;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node193;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node194;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node195;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node196;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node197;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node198;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node199;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node200;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node201;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node202;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node203;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node204;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node205;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node206;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node207;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node208;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node209;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node210;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node211;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node212;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node213;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node214;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node215;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node216;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node217;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node218;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node219;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node220;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node221;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node222;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node223;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node224;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node225;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node226;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node227;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node228;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node229;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node230;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node231;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node232;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node233;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node234;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node235;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node236;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node237;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node238;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node239;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node240;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node241;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node242;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node243;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node244;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node245;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node246;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node247;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node248;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node249;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node250;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node251;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node252;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node253;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node254;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node255;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node256;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node257;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node258;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node259;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node260;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node261;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node262;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node263;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node264;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node265;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node266;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node267;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node268;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node269;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node270;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node271;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node272;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node273;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node274;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node275;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node276;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node277;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node278;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node279;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node280;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node281;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node282;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node283;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node284;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node285;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node286;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node287;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node288;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node289;
        if node != empty_node {
            nodes.push(node);
        }

        let node = r.map_annotation_node290;
        if node == empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node291;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node292;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node293;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node294;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node295;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node296;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node297;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node298;
        if node != empty_node {
            nodes.push(node);
        }
        let node = r.map_annotation_node299;
        if node != empty_node {
            nodes.push(node);
        }
        // XXX: There's up to 300 nodes

        MapAnnotation {
            map_name: r.map_name,
            screen_text: r.screen_text,
            nodes,
        }
    }
}
