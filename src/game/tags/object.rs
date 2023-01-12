use super::prelude::*;

tag! {
    pub struct Object {
        pub physics: Option<TagId>,
        pub model: TagString,
        pub colour: [f32; 3],
    }
}
