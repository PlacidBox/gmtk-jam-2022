use macroquad::camera::Camera2D;
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    for _ in 1..120 {
        clear_background(BLACK);
        draw_text("Loading", 20.0, 20.0, 20.0, WHITE);
        next_frame().await;
    }

    // ogg exported from audacity seems to work well.
    let s = macroquad::audio::load_sound("examples_sound.ogg")
        .await
        .unwrap();

    macroquad::audio::play_sound_once(s);

    loop {
        let mut _x = Camera2D::from_display_rect(Rect {
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
        _x.viewport = Some((500, 0, 800, 600));
        macroquad::camera::set_camera(&_x);

        clear_background(RED);

        draw_line(0.0, 0.0, 100.0, 100.0, 15.0, BLUE);
        draw_text("Hello, world!", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
