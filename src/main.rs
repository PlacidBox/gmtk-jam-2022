#![windows_subsystem = "windows"]
use macroquad::camera::Camera2D;
use macroquad::prelude::*;

mod assets;

const TICKS_PER_SEC: f64 = 60.0;
const TICK_RATE: f64 = 1.0 / TICKS_PER_SEC;
const MAX_TIME_BEHIND: f64 = 0.200;

const WORLD_WIDTH: f32 = 1280.0;
const WORLD_HEIGHT: f32 = 720.0;

// don't run faster when moving diagonally.
const DIAG_SPEED: f32 = 0.7071;
const PLAYER_WALK_SPEED: f32 = 2.0;
const PLAYER_ROLL_SPEED: f32 = 6.0;
const PLAYER_ROLL_TICKS: i32 = 30;
const PLAYER_ROLL_RECOVERY_TICKS: i32 = 5;

const DEBUG_VIEW: bool = true;

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
    player_pos: (f32, f32),
    player_dir: (f32, f32),
    // which tick the player ceases rolling
    player_rolling_until: i32,
    knife_pos: (f32, f32),
}

#[derive(PartialEq, Eq)]
enum PlayerState {
    Walk,
    Roll,
    Recover,
    Dead,
}

impl GameState {
    fn player_state(&self) -> PlayerState {
        if self.player_rolling_until > self.tick {
            PlayerState::Roll
        } else {
            PlayerState::Walk
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        let world_centre = (WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
        Self {
            tick: 0,
            player_pos: world_centre,
            player_dir: (0.0, 0.0),
            player_rolling_until: 0,
            knife_pos: world_centre,
        }
    }
}

fn tick(state: &mut GameState) {
    state.tick += 1;

    if state.player_state() == PlayerState::Walk {
        let up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let start_roll = is_key_down(KeyCode::Space);

        state.player_dir = match (up, left, down, right) {
            (true, true, false, false) => (-DIAG_SPEED, -DIAG_SPEED), // UL
            (false, true, true, false) => (-DIAG_SPEED, DIAG_SPEED),  // DL
            (false, false, true, true) => (DIAG_SPEED, DIAG_SPEED),   // DR
            (true, false, false, true) => (DIAG_SPEED, -DIAG_SPEED),  // UR
            (true, _, false, _) => (0.0, -1.0),                       // U
            (_, true, _, false) => (-1.0, 0.0),                       // L
            (false, _, true, _) => (0.0, 1.0),                        // D
            (_, false, _, true) => (1.0, 0.0),                        // R
            _ => (0.0, 0.0),
        };

        if start_roll {
            state.player_rolling_until = state.tick + PLAYER_ROLL_TICKS;
        }
    }

    let speed_mul = match state.player_state() {
        PlayerState::Dead => 0.0,
        PlayerState::Walk => PLAYER_WALK_SPEED,
        PlayerState::Roll => PLAYER_ROLL_SPEED,
        PlayerState::Recover => 0.0,
    };

    state.player_pos.0 += state.player_dir.0 * speed_mul;
    state.player_pos.1 += state.player_dir.1 * speed_mul;
    ensure_in_bounds(&mut state.player_pos);

    // player dir, for holding out kinfe to kill enemies with. shouldn't ever be set to 0, stays
    // resting if not.

    // check for begin of roll animation. can't change dir while rolling?
}

fn ensure_in_bounds(pos: &mut (f32, f32)) {
    pos.0 = pos.0.clamp(0.0, WORLD_WIDTH);
    pos.1 = pos.1.clamp(0.0, WORLD_HEIGHT);
}

fn render(state: &GameState) {
    let x = Camera2D::from_display_rect(Rect {
        x: 0.0,
        y: 0.0,
        w: WORLD_WIDTH,
        h: WORLD_HEIGHT,
    });

    // FUTURE: set viewport to maintain a constant aspect ratio, rather than stretching.
    macroquad::camera::set_camera(&x);

    draw_text("WASD to move. Space to roll", 20., 40., 20.0, WHITE);
    draw_text(
        "Dice up the evil food with your knife",
        20.,
        60.,
        20.0,
        WHITE,
    );
    draw_text("Also you have food allergies", 20., 80., 20.0, WHITE);
    draw_text("Unless you're rolling. Of course", 20., 100., 20.0, WHITE);

    if DEBUG_VIEW {
        draw_circle_lines(state.player_pos.0, state.player_pos.1, 20.0, 1.0, RED);
        draw_circle_lines(state.knife_pos.0, state.knife_pos.1, 20.0, 1.0, GREEN);
    }
}
