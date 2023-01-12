#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod game;
mod math;
mod util;
mod render;
mod input;

use std::time::{SystemTime};
use game::{Game, actions::PlayerAction};
use input::InputMappings;
use render::{Window, run_event_loop, Renderer};
use pollster;

const WINDOW_TITLE: &str = "redrock";
const TICK_RATE: u32 = 120;
const TICK_DURATION_NANOS: u32 = 1000000000 / TICK_RATE;
const TICK_DURATION_SEC: f32 = 1.0 / TICK_RATE as f32;
// const MAX_TICKS_PER_FRAME: u32 = 10; //todo: prevent spiral of death

fn main() {
    let mut game = Game::load_map("maps/example.toml");
    let input_mappings = InputMappings::load("controls.toml");

    let mut prev_time = SystemTime::now();
    let mut accum_nanos: u128 = 0;

    let mut window = Window::new(WINDOW_TITLE);
    let mut renderer = pollster::block_on(Renderer::new(&window));

    run_event_loop(window, move |mut inputs, resize| -> bool {
        let curr_time = SystemTime::now();
        if let Ok(elapsed) = curr_time.duration_since(prev_time) {
            accum_nanos += elapsed.as_nanos();

            while accum_nanos >= TICK_DURATION_NANOS as u128 {
                game.update();
                accum_nanos -= TICK_DURATION_NANOS as u128;
            }

            if let Some((width, height)) = resize {
                renderer.resize(width, height);
            }

            renderer.render(&game);

            for input in inputs.drain(..) {
                match input_mappings.map_to_action(input) {
                    Some(PlayerAction::Quit) => {
                        return false;
                    },
                    Some(action) => {
                        game.apply_action(action);
                    },
                    None => {}
                }
            }
            prev_time = curr_time;
            return true;
        }
        true
    });
}
