use macroquad::camera::Camera2D;
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    for _ in 1..120 {
        clear_background(BLACK);
        draw_text("Loading", 20.0, 20.0, 20.0, WHITE);
        next_frame().await;
    }

    let s = macroquad::audio::load_sound("examples_sound.wav")
        .await
        .unwrap();

    macroquad::audio::play_sound_once(s);

    loop {
        let _x = Camera2D::from_display_rect(Rect {
            x: 0.,
            y: 0.,
            w: 100.,
            h: 100.,
        });
        macroquad::camera::set_camera(&_x);

        clear_background(RED);

        draw_line(0.0, 0.0, 100.0, 100.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("Hello, world!", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
