#![windows_subsystem = "windows"]
use macroquad::camera::Camera2D;
use macroquad::prelude::*;

mod assets;

const TICKS_PER_SEC: f64 = 60.0;
const TICK_RATE: f64 = 1.0 / TICKS_PER_SEC;
const MAX_TIME_BEHIND: f64 = 0.200;

fn make_conf() -> Conf {
    Conf {
        window_title: "roll and dice".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

// circle game 3: roll and dice
//  you're a little circle with a sword poking out. stab enemy to kill
//  WASD to move. space to roll. invincible while rolling, but still kill enemies! needs a cooldown,
//  but should be short.
//
//  do we control sword direction, or not? maybe only goes in the direction we're facing/moving
//
// otherwise kinda like geometry wars?
// enemies:
//  - soldier: boring guy who runs towards you. dies on impact
//  - wizard: shoots 'lightning bolts', tries to move to the opposite side of the arena, slowly,
//      or perhaps stays around where it spawns. brownian motion?
//  - pikeman: charges to try and cut you off.

#[macroquad::main(make_conf)]
async fn main() {
    let ass = assets::load().await.unwrap();
    macroquad::audio::play_sound_once(ass.example);

    // store last time we ticked at, increment by 1/60th each time we tick up. if we're > 1 behind,
    // bound to max 1 second of catchup?
    //
    // max logic ticks _per_ render? 2?
    // this is just to help smooth over stuttering
    let mut tick_time = get_time();

    let mut st = GameState {tick: 0};

    loop {
        let now = get_time();
        if now > tick_time + MAX_TIME_BEHIND {
            tick_time = now - MAX_TIME_BEHIND;
        }

        while tick_time < now {
            tick_time += TICK_RATE;
            tick(&mut st);
        }

        clear_background(BLACK);
        render(&st);
        next_frame().await
    }
}

struct GameState {
    tick: i32,
}

fn tick(state: &mut GameState) {
    state.tick += 1;
}

fn render(state: &GameState) {
    let x = Camera2D::from_display_rect(Rect {
        x: 0.0,
        y: 0.0,
        w: 1280.0,
        h: 720.0,
    });
    // so set viewport to some set ratio, just to make a simple 2 game a bit easier?
    // this crops. an alternative is to let any aspect ratio, and not crop the sides, but
    // that may be a bit more complicated. hmmm. depends on the game?
    //
    // this is measured from the bottom left of the screen, as x,y,width,height
    // x.viewport = Some((0, 0, 800, 600));
    // for now take up everything. we can do the borders later if needed.
    macroquad::camera::set_camera(&x);

    draw_line(0.0, 0.0, 600.0, 600.0, 15.0, BLUE);
    // draw_text("Hello, world!", 20.0, 20.0, 20.0, WHITE);
    draw_text(&format!("{0}", state.tick), 20., 20., 20.0, WHITE)
}
