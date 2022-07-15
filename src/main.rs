#![windows_subsystem = "windows"]

use macroquad::camera::Camera2D;
use macroquad::prelude::*;

static SOUND: &[u8; 17937] = include_bytes!("examples_sound.ogg");

// https://crates.io/crates/collider ?
// https://rapier.rs/docs/ ?

// https://opengameart.org/

#[macroquad::main("Game")]
async fn main() {
    // ogg exported from audacity seems to work well.
    let s = macroquad::audio::load_sound_from_bytes(SOUND)
        .await
        .unwrap();

    macroquad::audio::play_sound_once(s);

    loop {
        let mut x = Camera2D::from_display_rect(Rect {
            x: 0.,
            y: 0.,
            w: 100.,
            h: 100.,
        });
        // so set viewport to some set ratio, just to make a simple 2 game a bit easier?
        // this crops. an alternative is to let any aspect ratio, and not crop the sides, but
        // that may be a bit more complicated. hmmm. depends on the game?
        //
        // this is measured from the bottom left of the screen, as x,y,width,height
        x.viewport = Some((500, 0, 800, 600));
        macroquad::camera::set_camera(&x);

        clear_background(RED);

        draw_line(0.0, 0.0, 100.0, 100.0, 15.0, BLUE);
        draw_text("Hello, world!", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
