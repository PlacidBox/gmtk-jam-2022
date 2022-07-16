#![windows_subsystem = "windows"]
use macroquad::camera::Camera2D;
use macroquad::prelude::*;

mod assets;

const TICKS_PER_SEC: f64 = 60.0;
const TICK_RATE: f64 = 1.0 / TICKS_PER_SEC;
const MAX_TIME_BEHIND: f64 = 0.200;

const WORLD_WIDTH: f32 = 1280.0;
const WORLD_HEIGHT: f32 = 720.0;

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
//
// fruit theming?
//  player -> chef
//  soldier? -> pineapple
//  wizard -> grape bunch. shoots grapes
//  pikeman? some long pointy fruit? baugette?
//      maybe use emojis?

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

    let mut st = GameState::default();

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
    player: (f32, f32),
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tick: 0,
            player: (WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0),
        }
    }
}

fn tick(state: &mut GameState) {
    state.tick += 1;

    const PLAYER_SPEED: f32 = 3.0;
    const DIAG_MUL: f32 = 0.7071;

    // ,aoe
    if is_key_down(KeyCode::W) {
        state.player.1 -= PLAYER_SPEED;
    }

    if is_key_down(KeyCode::A) {
        state.player.0 -= PLAYER_SPEED;
    }

    if is_key_down(KeyCode::S) {
        state.player.1 += PLAYER_SPEED;
    }

    if is_key_down(KeyCode::D) {
        state.player.0 += PLAYER_SPEED;

    }

    // player dir, for holding out kinfe to kill enemies with. shouldn't ever be set to 0, stays
    // resting if not.

    // check for begin of roll animation. can't change dir while rolling?
}

fn render(state: &GameState) {
    let x = Camera2D::from_display_rect(Rect {
        x: 0.0,
        y: 0.0,
        w: WORLD_WIDTH,
        h: WORLD_HEIGHT,
    });
    // so set viewport to some set ratio, just to make a simple 2 game a bit easier?
    // this crops. an alternative is to let any aspect ratio, and not crop the sides, but
    // that may be a bit more complicated. hmmm. depends on the game?
    //
    // this is measured from the bottom left of the screen, as x,y,width,height
    // x.viewport = Some((0, 0, 800, 600));
    // for now take up everything. we can do the borders later if needed.
    macroquad::camera::set_camera(&x);

    // draw_text("Hello, world!", 20.0, 20.0, 20.0, WHITE);
    draw_text(&format!("{0}", state.tick), 20., 20., 20.0, WHITE);
    draw_text("WASD to move. Space to roll", 20., 40., 20.0, WHITE);
    draw_text("Dice up the evil food with your knife", 20., 60., 20.0, WHITE);
    draw_text("Also you have food allergies", 20., 80., 20.0, WHITE);
    draw_text("Unless you're rolling. Of course", 20., 100., 20.0, WHITE);

    draw_circle_lines(state.player.0, state.player.1, 20.0, 3.0, RED);
    draw_circle_lines(state.player.0 + 12.0, state.player.1 + 12.0, 20.0, 3.0, GREEN);
}
