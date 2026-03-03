mod audio;
mod game;
mod physics;

use game::SCREEN_H;
use game::SCREEN_W;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Rust".to_string(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let audio = audio::AudioBank::load().await;
    let mut game = game::Game::new(audio);

    loop {
        let dt = get_frame_time();
        game.update(dt);
        game.draw();
        next_frame().await;
    }
}
