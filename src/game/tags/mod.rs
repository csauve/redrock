use std::collections::hash_map::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::io::prelude::*;
use toml;
use serde::{Deserializer, Deserialize};

mod scenario;
mod globals;
mod object;
mod physics;

pub use scenario::*;
pub use globals::*;
pub use object::*;
pub use physics::*;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default)]
pub struct TagString([u8; 32]);
pub type TagId = TagString;

impl Into<String> for TagString {
    fn into(self) -> String {
        String::from_utf8((self.0).iter().map(|c| *c).collect()).expect("msg: &str")
    }
}

impl<'de> Deserialize<'de> for TagString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        let string_val: String = Deserialize::deserialize(deserializer)?;
        // let string_bytes = string_val.as_bytes();
        // if !string_val.is_ascii() || string_bytes.len() > 32 {
        //     return Err(D::Error::invalid_value(format!("{} is not an ASCII string at most 32 chars long", string_val)));
        // }
        let mut buffer = [0u8; 32];
        for (i, c) in string_val.bytes().take(32).enumerate() {
            buffer[i] = c;
        }
        Ok(TagString(buffer))
    }
}

#[derive(Deserialize)]
pub struct Map {
    pub globals: globals::Globals,
    pub scenario: scenario::Scenario,
    pub object: HashMap<TagId, Object>,
    pub physics: HashMap<TagId, Physics>,
}

macro_rules! get_tag {
    ($name:ident, $hashmap:ident, $type:ty) => {
        pub fn $name(&self, tag_id: &TagId) -> Option<&$type> {
            self.$hashmap.get(tag_id)
        }
    };
}

impl Map {
    pub fn load(path: &str) -> Map {
        let mut file = File::open(path).expect("Failed to open map file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read map file");
        toml::from_slice(contents.as_bytes()).expect("Failed to parse map file")
    }

    get_tag!(get_object, object, Object);
    get_tag!(get_physics, physics, Physics);
}

mod prelude {
    pub use super::{TagId, TagString};
    pub use serde::{Deserialize};

    #[macro_export]
    macro_rules! tag {
        ($s:item) => {
            #[repr(C)]
            #[derive(Clone, Deserialize)]
            $s
        };
    }

    pub use tag;
}