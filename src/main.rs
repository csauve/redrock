#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod game;
mod math;
mod util;
mod render;

use std::time::{Duration, SystemTime};
use game::{Game, actions::PlayerAction};
use render::{Window, run_event_loop, InputEvent, Renderer};
use pollster;

const WINDOW_TITLE: &str = "redrock";
const TICK_RATE: u32 = 120;
const TICK_DURATION_NANOS: u32 = 1000000000 / TICK_RATE;
const TICK_DURATION_SEC: f32 = 1.0 / TICK_RATE as f32;
// const MAX_TICKS_PER_FRAME: u32 = 10; //todo: prevent spiral of death

fn main() {
    let mut game = Game::load_map("maps/example.toml");

    let mut prev_time = SystemTime::now();
    let mut accum_nanos: u128 = 0;

    let mut window = Window::new(WINDOW_TITLE);
    let mut renderer = pollster::block_on(Renderer::new(&window));

    run_event_loop(window, move |mut inputs, resize| {
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

            handle_inputs(&mut game, &mut inputs);
            prev_time = curr_time;
        }
    });
}

fn handle_inputs(game: &mut Game, inputs: &mut Vec<InputEvent>) {
    for input in inputs.drain(..) {
        // dbg!(&input);
        match input {
            //W
            InputEvent::Key {code: 17, pressed} => {
                game.apply_action(PlayerAction::Forward(pressed));
            },
            //S
            InputEvent::Key {code: 31, pressed} => {
                game.apply_action(PlayerAction::Back(pressed));
            },
            //D
            InputEvent::Key {code: 32, pressed} => {
                game.apply_action(PlayerAction::Right(pressed));
            },
            //A
            InputEvent::Key {code: 30, pressed} => {
                game.apply_action(PlayerAction::Left(pressed));
            },
            //space
            InputEvent::Key {code: 57, pressed} => {
                game.apply_action(PlayerAction::Jump(pressed));
            },
            //ctrl
            InputEvent::Key {code: 29, pressed} => {
                game.apply_action(PlayerAction::Crouch(pressed));
            },
            InputEvent::Mouse {delta: (dx, dy)} => {
                let dx = dx / 100.0;
                let dy = dy / 100.0;
                game.apply_action(PlayerAction::AimDelta(dx as f32, dy as f32 / 2.0))
            },
            _ => ()
        }
    }
}
