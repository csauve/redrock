#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod game;
mod math;
mod util;
mod render;
mod tags;

use std::time::{Duration, SystemTime};
use tags::Map;
use game::{GameState, ObjectState};
use math::Vec3f;
use render::{Window, run_event_loop, InputEvent, Renderer};
use pollster;

const WINDOW_TITLE: &str = "redrock";
const TICK_RATE: u32 = 60;
const TICK_DURATION_NANOS: u32 = 1000000000 / TICK_RATE;
const TICK_DURATION_SEC: f32 = 1.0 / TICK_RATE as f32;
// const MAX_TICKS_PER_FRAME: u32 = 10; //todo: prevent spiral of death

fn main() {
    let map = Map::load("maps/example.toml");
    let mut game_state = GameState::from_map(&map);

    let mut prev_time = SystemTime::now();
    let mut accum_nanos: u128 = 0;

    let mut window = Window::new(WINDOW_TITLE);
    let mut renderer = pollster::block_on(Renderer::new(&window));

    run_event_loop(window, move |mut inputs, resize| {
        let curr_time = SystemTime::now();
        if let Ok(elapsed) = curr_time.duration_since(prev_time) {
            accum_nanos += elapsed.as_nanos();

            while accum_nanos >= TICK_DURATION_NANOS as u128 {
                do_tick(&mut game_state);
                accum_nanos -= TICK_DURATION_NANOS as u128;
            }

            if let Some((width, height)) = resize {
                renderer.resize(width, height);
            }

            renderer.render(&game_state);

            handle_inputs(&mut game_state, &mut inputs);
            prev_time = curr_time;
        }
    });
}

fn do_tick(game_state: &mut GameState) {
    // dbg!(game_state.tick);

    for (_id, _item) in game_state.objects.iter_mut() {
        // item.position.x += 0.001;
    }

    game_state.tick = game_state.tick.wrapping_add(1);
}

fn handle_inputs(game_state: &mut GameState, inputs: &mut Vec<InputEvent>) {
    for input in inputs.drain(..) {
        dbg!(&input);
        match input {
            //A
            InputEvent::Key {code: 30, pressed: true} => {
                if let Some(player) = game_state.objects.get_mut(game_state.player_object_id) {
                    player.position.x += 0.01;
                }
            },
            _ => ()
        }
    }
}
