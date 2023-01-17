#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod game;
mod math;
mod util;
mod render;
mod config;

use game::{Game, actions::PlayerAction};
use config::Config;
use render::{Window, run_event_loop, Renderer};
use pollster;
use env_logger;

const WINDOW_TITLE: &str = "redrock";
const WINDOW_SIZE: [u32; 2] = [800, 600];

fn main() {
    env_logger::init();
    let mut game = Game::load_map("maps/example.toml");
    let config = Config::load("config.toml");

    let mut window = Window::new(WINDOW_TITLE, WINDOW_SIZE[0], WINDOW_SIZE[1]);
    let mut renderer = pollster::block_on(Renderer::new(&window));

    run_event_loop(window, move |mut inputs, resize| -> bool {
        if let Some((width, height)) = resize {
            renderer.resize(width, height);
        }

        let actions: Vec<PlayerAction> = inputs
            .drain(..)
            .filter_map(|input| config.map_to_action(input))
            .collect();

        let keep_running = game.update(&actions);
        renderer.render(&game);

        keep_running
    });
}
