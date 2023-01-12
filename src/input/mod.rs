use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use toml;
use serde::{Deserialize};

use crate::game::actions::PlayerAction;
use winit::event::{VirtualKeyCode, MouseButton};

#[derive(Debug)]
pub enum InputEvent {
    Key {code: u32, pressed: bool, key: Option<VirtualKeyCode>},
    Click {button: MouseButton, pressed: bool},
    Mouse {delta: (f64, f64)},
}

#[derive(Deserialize)]
pub struct InputMappings {
    pub controls: HashMap<String, String>,
}

impl Default for InputMappings {
    fn default() -> Self {
        let mut controls: HashMap<String, String> = HashMap::new();
        controls.insert("W".into(), "Forward".into());
        InputMappings {
            controls
        }
    }
}

impl InputMappings {
    pub fn load(path: &str) -> Self {
        if let Ok(mut file) = File::open(path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(mapping) = toml::from_slice::<InputMappings>(contents.as_bytes()) {
                    return mapping;
                } else {
                    println!("Failed to parse map file");
                }
            } else {
                println!("Failed to read map file");
            }
        } else {
            println!("Failed to open map file");
        }
        InputMappings::default()
    }

    pub fn map_to_action(&self, input: InputEvent) -> Option<PlayerAction> {
        match input {
            //Esc
            InputEvent::Key {code: _, pressed: false, key: Some(VirtualKeyCode::Escape)} => {
                Some(PlayerAction::Quit)
            },
            //W
            InputEvent::Key {code: _, pressed, key: Some(VirtualKeyCode::W)} => {
                Some(PlayerAction::Forward(pressed))
            },
            //S
            InputEvent::Key {code: _, pressed, key: Some(VirtualKeyCode::S)} => {
                Some(PlayerAction::Back(pressed))
            },
            //D
            InputEvent::Key {code: _, pressed, key: Some(VirtualKeyCode::D)} => {
                Some(PlayerAction::Right(pressed))
            },
            //A
            InputEvent::Key {code: _, pressed, key: Some(VirtualKeyCode::A)} => {
                Some(PlayerAction::Left(pressed))
            },
            //space
            InputEvent::Key {code: _, pressed, key: Some(VirtualKeyCode::Space)} => {
                Some(PlayerAction::Jump(pressed))
            },
            //ctrl
            InputEvent::Key {code: _, pressed, key: Some(VirtualKeyCode::LControl)} => {
                Some(PlayerAction::Crouch(pressed))
            },
            InputEvent::Mouse {delta: (dx, dy)} => {
                let dx = dx / 300.0;
                let dy = dy / 300.0;
                Some(PlayerAction::AimDelta(dx as f32, dy as f32 / 2.0))
            },
            _ => {
                dbg!(&input);
                None
            }
        }
    }
}
