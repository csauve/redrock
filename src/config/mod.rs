use std::{collections::HashMap, iter::FromIterator};
use std::fs::File;
use std::io::prelude::*;
use toml;
use serde::Deserialize;

use crate::game::actions::PlayerAction;
use crate::render::InputEvent;
use winit::event::{VirtualKeyCode, MouseButton};

macro_rules! map {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut hash = HashMap::new();
            $(
                hash.insert($k, $v);
            )*
            hash
        }
    };
}

const MAPPABLE_KEYCODES: &[(VirtualKeyCode, &'static str)] = &[
    (VirtualKeyCode::Key1, "Key1"),
    (VirtualKeyCode::Key2, "Key2"),
    (VirtualKeyCode::Key3, "Key3"),
    (VirtualKeyCode::Key4, "Key4"),
    (VirtualKeyCode::Key5, "Key5"),
    (VirtualKeyCode::Key6, "Key6"),
    (VirtualKeyCode::Key7, "Key7"),
    (VirtualKeyCode::Key8, "Key8"),
    (VirtualKeyCode::Key9, "Key9"),
    (VirtualKeyCode::Key0, "Key0"),
    (VirtualKeyCode::A, "A"),
    (VirtualKeyCode::B, "B"),
    (VirtualKeyCode::C, "C"),
    (VirtualKeyCode::D, "D"),
    (VirtualKeyCode::E, "E"),
    (VirtualKeyCode::F, "F"),
    (VirtualKeyCode::G, "G"),
    (VirtualKeyCode::H, "H"),
    (VirtualKeyCode::I, "I"),
    (VirtualKeyCode::J, "J"),
    (VirtualKeyCode::K, "K"),
    (VirtualKeyCode::L, "L"),
    (VirtualKeyCode::M, "M"),
    (VirtualKeyCode::N, "N"),
    (VirtualKeyCode::O, "O"),
    (VirtualKeyCode::P, "P"),
    (VirtualKeyCode::Q, "Q"),
    (VirtualKeyCode::R, "R"),
    (VirtualKeyCode::S, "S"),
    (VirtualKeyCode::T, "T"),
    (VirtualKeyCode::U, "U"),
    (VirtualKeyCode::V, "V"),
    (VirtualKeyCode::W, "W"),
    (VirtualKeyCode::X, "X"),
    (VirtualKeyCode::Y, "Y"),
    (VirtualKeyCode::Z, "Z"),
    (VirtualKeyCode::F1, "F1"),
    (VirtualKeyCode::F2, "F2"),
    (VirtualKeyCode::F3, "F3"),
    (VirtualKeyCode::F4, "F4"),
    (VirtualKeyCode::F5, "F5"),
    (VirtualKeyCode::F6, "F6"),
    (VirtualKeyCode::F7, "F7"),
    (VirtualKeyCode::F8, "F8"),
    (VirtualKeyCode::F9, "F9"),
    (VirtualKeyCode::F10, "F10"),
    (VirtualKeyCode::F11, "F11"),
    (VirtualKeyCode::F12, "F12"),
    (VirtualKeyCode::F13, "F13"),
    (VirtualKeyCode::F14, "F14"),
    (VirtualKeyCode::F15, "F15"),
    (VirtualKeyCode::F16, "F16"),
    (VirtualKeyCode::F17, "F17"),
    (VirtualKeyCode::F18, "F18"),
    (VirtualKeyCode::F19, "F19"),
    (VirtualKeyCode::F20, "F20"),
    (VirtualKeyCode::F21, "F21"),
    (VirtualKeyCode::F22, "F22"),
    (VirtualKeyCode::F23, "F23"),
    (VirtualKeyCode::F24, "F24"),
    (VirtualKeyCode::Insert, "Insert"),
    (VirtualKeyCode::Home, "Home"),
    (VirtualKeyCode::Delete, "Delete"),
    (VirtualKeyCode::End, "End"),
    (VirtualKeyCode::PageDown, "PageDown"),
    (VirtualKeyCode::PageUp, "PageUp"),
    (VirtualKeyCode::Left, "Left"),
    (VirtualKeyCode::Up, "Up"),
    (VirtualKeyCode::Right, "Right"),
    (VirtualKeyCode::Down, "Down"),
    (VirtualKeyCode::Return, "Return"),
    (VirtualKeyCode::Space, "Space"),
    (VirtualKeyCode::Numlock, "Numlock"),
    (VirtualKeyCode::Numpad0, "Numpad0"),
    (VirtualKeyCode::Numpad1, "Numpad1"),
    (VirtualKeyCode::Numpad2, "Numpad2"),
    (VirtualKeyCode::Numpad3, "Numpad3"),
    (VirtualKeyCode::Numpad4, "Numpad4"),
    (VirtualKeyCode::Numpad5, "Numpad5"),
    (VirtualKeyCode::Numpad6, "Numpad6"),
    (VirtualKeyCode::Numpad7, "Numpad7"),
    (VirtualKeyCode::Numpad8, "Numpad8"),
    (VirtualKeyCode::Numpad9, "Numpad9"),
    (VirtualKeyCode::NumpadAdd, "NumpadAdd"),
    (VirtualKeyCode::NumpadDivide, "NumpadDivide"),
    (VirtualKeyCode::NumpadDecimal, "NumpadDecimal"),
    (VirtualKeyCode::NumpadComma, "NumpadComma"),
    (VirtualKeyCode::NumpadEnter, "NumpadEnter"),
    (VirtualKeyCode::NumpadEquals, "NumpadEquals"),
    (VirtualKeyCode::NumpadMultiply, "NumpadMultiply"),
    (VirtualKeyCode::NumpadSubtract, "NumpadSubtract"),
    (VirtualKeyCode::Backslash, "Backslash"),
    (VirtualKeyCode::Comma, "Comma"),
    (VirtualKeyCode::Equals, "Equals"),
    (VirtualKeyCode::Grave, "Grave"),
    (VirtualKeyCode::LAlt, "LAlt"),
    (VirtualKeyCode::LBracket, "LBracket"),
    (VirtualKeyCode::LControl, "LControl"),
    (VirtualKeyCode::LShift, "LShift"),
    (VirtualKeyCode::Minus, "Minus"),
    (VirtualKeyCode::Period, "Period"),
    (VirtualKeyCode::RAlt, "RAlt"),
    (VirtualKeyCode::RBracket, "RBracket"),
    (VirtualKeyCode::RControl, "RControl"),
    (VirtualKeyCode::RShift, "RShift"),
    (VirtualKeyCode::Semicolon, "Semicolon"),
    (VirtualKeyCode::Slash, "Slash"),
    (VirtualKeyCode::Tab, "Tab"),
];

#[derive(Deserialize)]
pub struct Config {
    pub controls: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Config {
        let mut controls: HashMap<String, String> = map!(
            "W".into() => "Forward".into(),
            "S".into() => "Back".into(),
            "A".into() => "Left".into(),
            "D".into() => "Right".into(),
            "Space".into() => "Jump".into(),
            "LControl".into() => "Crouch".into()
        );
        Config {
            controls
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Config {
        if let Ok(mut file) = File::open(path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(mapping) = toml::from_slice::<Config>(contents.as_bytes()) {
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
        Config::default()
    }

    pub fn map_to_action(&self, input: InputEvent) -> Option<PlayerAction> {
        match input {
            //Esc
            InputEvent::Key {code: _, pressed: false, key: Some(VirtualKeyCode::Escape)} => {
                Some(PlayerAction::Quit)
            },
            //Bindable keys
            InputEvent::Key {code: _, pressed, key: Some(key)} => {
                if let Some(&(_, key_config_name)) = MAPPABLE_KEYCODES.iter().find(|kv| kv.0 == key) {
                    return match self.controls.get(key_config_name).map(String::as_str) {
                        Some("Forward") => Some(PlayerAction::Forward(pressed)),
                        Some("Back") => Some(PlayerAction::Back(pressed)),
                        Some("Left") => Some(PlayerAction::Left(pressed)),
                        Some("Right") => Some(PlayerAction::Right(pressed)),
                        Some("Crouch") => Some(PlayerAction::Crouch(pressed)),
                        Some("Jump") => Some(PlayerAction::Jump(pressed)),
                        Some("Boost") => Some(PlayerAction::Boost(pressed)),
                        _ => None,
                    }
                }
                None
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
