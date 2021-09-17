use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::prelude::*;
use toml;

mod scenario;
mod globals;
mod object;
mod physics;

pub use scenario::*;
pub use globals::*;
pub use object::*;
pub use physics::*;

type TagString = String; //todo
pub type TagId = TagString;

mod prelude {
    pub use serde::{Serialize, Deserialize};
    pub use super::TagId;

    #[macro_export]
    macro_rules! tag {
        ($s:item) => {
            #[repr(C)]
            #[derive(Clone, Serialize, Deserialize)]
            $s
        };
    }

    pub use tag;
}

use prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub globals: globals::Globals,
    pub scenario: scenario::Scenario,
    pub object: HashMap<TagId, object::Object>,
    pub physics: HashMap<TagId, physics::Physics>,
}

impl Map {
    pub fn load(path: &str) -> Map {
        let mut file = File::open(path).expect("Failed to open map file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read map file");
        toml::from_slice(contents.as_bytes()).expect("Failed to parse map file")
    }
}
