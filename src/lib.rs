use kv3::kv3_serde::serde_kv3;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Text {}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Title<'a> {
    // Text = ""
    pub text: &'a str,
    // FontSize = 75
    pub font_size: u32,
    // FadeInDist = 300.0
    pub fade_in_dist: f32,
    // FadeOutDist = 40.0
    pub fade_out_dist: f32,
}

// Currently the same as Title
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Description<'a> {
    // Text = ""
    pub text: &'a str,
    // FontSize = 75
    pub font_size: u32,
    // FadeInDist = 300.0
    pub fade_in_dist: f32,
    // FadeOutDist = 40.0
    pub fade_out_dist: f32,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node<'a> {
    // Enabled = true
    pub enabled: bool,
    // Type = "grenade"
    #[serde(rename = "Type")]
    pub r#type: &'a str,
    // Id = "860dd8f2-cc23-432e-81a7-03ebd7956940"
    pub id: &'a str,
    // SubType = "main"
    pub sub_type: &'a str,
    // TODO: Wrap in Position struct
    // Position = [ -1332.026978, 491.96875, 8.03125 ]
    pub position: [f64; 3],
    // TODO: Wrap in Angle struct
    // Angles = [ 0.0, 87.922913, 0.0 ]
    pub angles: [f64; 3],
    // VisiblePfx = true
    pub visible_pfx: bool,
    // Color = [ 234, 191, 86 ]
    pub color: [u8; 3],
    // TextPositionOffset = [ 0.0, 0.0, 80.0 ]
    pub text_position_offset: [f32; 3],

    // TextFacePlayer = true
    pub text_face_player: bool,
    // TODO: Make text_horizontal_align an enum
    // TextHorizontalAlign = "center"
    pub text_horizontal_align: &'a str,

    // RevealOnSuccess = false
    pub reveal_on_success: bool,

    pub title: Title<'a>,
    #[serde(rename = "Desc")]
    pub description: Description<'a>,

    // StreakLimitGuidesOn = 2
    pub streak_limit_guides_on: u8,
    // StreakLimitGuidesOff = 2
    pub streak_limit_guides_off: u8,
    // JumpThrow = false
    pub jump_throw: bool,
    // MasterNodeId = "4f2f0b14-3d59-4c52-bccf-6530ec0a5af4"
    pub master_node_id: &'a str,
    // DistanceThreshold = 80.0
    pub distance_threshold: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct SillyMapAnnotation<'a> {
    map_name: &'a str,
    screen_text: Text,

    // Spiritually simply,
    // annotations: Vec<Node>,

    // WHY VALVE?? WHY? Didn't KeyValues3 already support arrays?!
    map_annotation_node0: Node<'a>,
    map_annotation_node1: Node<'a>,
    map_annotation_node2: Node<'a>,
    map_annotation_node3: Node<'a>,
    map_annotation_node4: Node<'a>,
    map_annotation_node5: Node<'a>,
    map_annotation_node6: Node<'a>,
    map_annotation_node7: Node<'a>,
    map_annotation_node8: Node<'a>,
    map_annotation_node9: Node<'a>,
    map_annotation_node10: Node<'a>,
    map_annotation_node11: Node<'a>,
    map_annotation_node12: Node<'a>,
    map_annotation_node13: Node<'a>,
    map_annotation_node14: Node<'a>,
    map_annotation_node15: Node<'a>,
    map_annotation_node16: Node<'a>,
    map_annotation_node17: Node<'a>,
    map_annotation_node18: Node<'a>,
    map_annotation_node19: Node<'a>,
    map_annotation_node20: Node<'a>,
    map_annotation_node21: Node<'a>,
    map_annotation_node22: Node<'a>,
    map_annotation_node23: Node<'a>,
    map_annotation_node24: Node<'a>,
    map_annotation_node25: Node<'a>,
    map_annotation_node26: Node<'a>,
    map_annotation_node27: Node<'a>,
    map_annotation_node28: Node<'a>,
    map_annotation_node29: Node<'a>,
    map_annotation_node30: Node<'a>,
    map_annotation_node31: Node<'a>,
    map_annotation_node32: Node<'a>,
    map_annotation_node33: Node<'a>,
    map_annotation_node34: Node<'a>,
    map_annotation_node35: Node<'a>,
    map_annotation_node36: Node<'a>,
    map_annotation_node37: Node<'a>,
    map_annotation_node38: Node<'a>,
    map_annotation_node39: Node<'a>,
    map_annotation_node40: Node<'a>,
    map_annotation_node41: Node<'a>,
    map_annotation_node42: Node<'a>,
    map_annotation_node43: Node<'a>,
    map_annotation_node44: Node<'a>,
    map_annotation_node45: Node<'a>,
    map_annotation_node46: Node<'a>,
    map_annotation_node47: Node<'a>,
    map_annotation_node48: Node<'a>,
    map_annotation_node49: Node<'a>,
    map_annotation_node50: Node<'a>,
    map_annotation_node51: Node<'a>,
    map_annotation_node52: Node<'a>,
    map_annotation_node53: Node<'a>,
    map_annotation_node54: Node<'a>,
    map_annotation_node55: Node<'a>,
    map_annotation_node56: Node<'a>,
    map_annotation_node57: Node<'a>,
    map_annotation_node58: Node<'a>,
    map_annotation_node59: Node<'a>,
    // XXX: There's up to 60 nodes
    // TODO: Fix code repetition
}

#[derive(Debug, Default)]
pub struct MapAnnotation<'a> {
    pub map_name: &'a str,
    pub screen_text: Text,

    pub nodes: Vec<Node<'a>>,
}

impl<'a> From<SillyMapAnnotation<'a>> for MapAnnotation<'a> {
    fn from(r: SillyMapAnnotation<'a>) -> Self {
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
        // XXX: There's up to 60 nodes

        MapAnnotation {
            map_name: r.map_name,
            screen_text: r.screen_text,
            nodes,
        }
    }
}

enum MapAnnotationError {
    ParseError,
}

impl MapAnnotation<'_> {
    fn try_from<'o>(map_annotations_str: &'o str) -> Result<MapAnnotation<'o>, MapAnnotationError> {
        // TODO: Fix build error
        let silly_map_annotation: SillyMapAnnotation =
            serde_kv3::<'o>(map_annotations_str).map_err(|_| MapAnnotationError::ParseError)?;
        Ok(MapAnnotation::from(silly_map_annotation))
    }
}
