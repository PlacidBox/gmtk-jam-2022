// #![windows_subsystem = "windows"]
use macroquad::audio::PlaySoundParams;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;

mod assets;

const TICKS_PER_SEC: i32 = 60;
const TICK_RATE: f64 = 1.0 / TICKS_PER_SEC as f64;
const MAX_TIME_BEHIND: f64 = 0.200;

const WORLD_WIDTH: f32 = 1280.0;
const WORLD_HEIGHT: f32 = 720.0;

// don't run faster when moving diagonally.
const DIAG_SPEED: f32 = 0.7071;
const PLAYER_RADIUS: f32 = 20.0;
const PLAYER_WALK_SPEED: f32 = 2.0;
const PLAYER_ROLL_SPEED: f32 = 6.0;
const PLAYER_ROLL_TICKS: i32 = 30;
const PLAYER_ROLL_RECOVERY_TICKS: i32 = 20;

// Knife hitbox, and how far away from the player it is.
const KNIFE_RADIUS: f32 = 20.0;
const KINFE_REACH: f32 = 30.0;

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
    macroquad::audio::play_sound(
        ass.bgm,
        PlaySoundParams {
            looped: true,
            volume: 0.125,
        },
    );

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

#[derive(PartialEq, Eq)]
enum PlayerState {
    Walk,
    Roll,
    Recover,
    Dead,
}

struct GameState {
    game_over: bool,
    tick: i32,

    player_pos: Vec2,
    player_dir: Vec2,
    // which tick the player ceases rolling, and starts recovering from the roll
    player_rolling_until: i32,

    // knife keeps its own dir, so that it doesn't get set back to 0,0 when hte player stops moving
    knife_pos: Vec2,
    knife_dir: Vec2,

    lemons: Vec<Lemon>,
}

impl GameState {
    fn player_state(&self) -> PlayerState {
        if self.game_over {
            PlayerState::Dead
        } else if self.player_rolling_until > self.tick {
            PlayerState::Roll
        } else if self.player_rolling_until + PLAYER_ROLL_RECOVERY_TICKS > self.tick {
            PlayerState::Recover
        } else {
            PlayerState::Walk
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        let world_centre = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
        Self {
            game_over: false,
            tick: 0,
            player_pos: world_centre,
            player_dir: vec2(0.0, 0.0),
            // dirty hack to start the player not in recovery mode
            player_rolling_until: -PLAYER_ROLL_RECOVERY_TICKS,

            knife_pos: world_centre,
            knife_dir: vec2(1.0, 0.0),

            lemons: vec![],
        }
    }
}

fn tick(state: &mut GameState) {
    if state.game_over {
        return;
    }
    state.tick += 1;

    tick_player(state);
    tick_knife(state);
    tick_check_enemy_death(state);
    tick_check_player_death(state);
    tick_spawner(state);
    tick_enemies(state);
}

fn tick_player(state: &mut GameState) {
    if state.player_state() == PlayerState::Walk || state.player_state() == PlayerState::Recover {
        let up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);

        state.player_dir = match (up, left, down, right) {
            (true, true, false, false) => vec2(-DIAG_SPEED, -DIAG_SPEED), // UL
            (false, true, true, false) => vec2(-DIAG_SPEED, DIAG_SPEED),  // DL
            (false, false, true, true) => vec2(DIAG_SPEED, DIAG_SPEED),   // DR
            (true, false, false, true) => vec2(DIAG_SPEED, -DIAG_SPEED),  // UR
            (true, _, false, _) => vec2(0.0, -1.0),                       // U
            (_, true, _, false) => vec2(-1.0, 0.0),                       // L
            (false, _, true, _) => vec2(0.0, 1.0),                        // D
            (_, false, _, true) => vec2(1.0, 0.0),                        // R
            _ => vec2(0.0, 0.0),
        };

        let start_roll =
            is_key_down(KeyCode::Space) && state.player_state() != PlayerState::Recover;
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
    state.player_pos += state.player_dir * speed_mul;

    ensure_in_bounds(&mut state.player_pos);
}

fn tick_knife(state: &mut GameState) {
    if state.player_dir != vec2(0.0, 0.0) {
        state.knife_dir = state.player_dir
    };
    state.knife_pos = state.player_pos + state.knife_dir * KINFE_REACH;
}

fn tick_check_enemy_death(state: &mut GameState) {
    const LEMON_KILL_DIST_SQ: f32 = KNIFE_RADIUS * KNIFE_RADIUS + LEMON_RADIUS * LEMON_RADIUS;

    let kill_zone = state.knife_pos;
    state.lemons.retain(|l|l.pos.distance_squared(kill_zone) > LEMON_KILL_DIST_SQ);
}

fn tick_check_player_death(state: &mut GameState) {
}

fn tick_spawner(state: &mut GameState) {
    if state.tick % 60 == 0 {
        let new_lemon = Lemon::new(rand_spawn_pos(state.player_pos));
        state.lemons.push(new_lemon);
    }
}

fn tick_enemies(state: &mut GameState) {
    for l in &mut state.lemons {
        l.tick(state.player_pos);
    }
}

// an enemy that starts as a lime, wanders for a bit, then begins to charge the player aggressively
// after turning in to a lemon
const LEMON_SPEED_WANDER: f32 = 0.5;
const LEMON_WANDER_CLOSE: f32 = 10.0;
const LEMON_WANDER_SQ: f32 = LEMON_WANDER_CLOSE * LEMON_WANDER_CLOSE;
const LEMON_SPEED_ATTACK: f32 = 2.5;
const LEMON_ATTACKS_AFTER: i32 = TICKS_PER_SEC * 10;
const LEMON_RADIUS: f32 = 10.0;
struct Lemon {
    pos: Vec2,
    wander_to: Vec2,
    attacks_in: i32,
}

impl Lemon {
    fn new(spawn_point: Vec2) -> Lemon {
        Lemon {
            pos: spawn_point,
            wander_to: spawn_point,
            attacks_in: LEMON_ATTACKS_AFTER,
        }
    }

    fn tick(&mut self, player_pos: Vec2) {
        if self.is_attacking() {
            // move towards player at attack rate
            let dir = (player_pos - self.pos).normalize_or_zero();
            self.pos += dir * LEMON_SPEED_ATTACK;
            return;
        }

        self.attacks_in -= 1;
        if self.pos.distance_squared(self.wander_to) < LEMON_WANDER_SQ {
            // try to avoid the player when wandering.
            self.wander_to = rand_spawn_pos(player_pos);
        }

        let dir = (self.wander_to - self.pos).normalize();
        self.pos += dir * LEMON_SPEED_WANDER;

        ensure_in_bounds(&mut self.pos);
    }

    fn is_attacking(&self) -> bool {
        self.attacks_in == 0
    }
}

fn ensure_in_bounds(pos: &mut Vec2) {
    pos.x = pos.x.clamp(0.0, WORLD_WIDTH);
    pos.y = pos.y.clamp(0.0, WORLD_HEIGHT);
}

fn rand_spawn_pos(avoid_pos: Vec2) -> Vec2 {
    const TOO_CLOSE: f32 = 250.0;
    const TOO_CLOSE_SQ: f32 = TOO_CLOSE * TOO_CLOSE;

    loop {
        let v = vec2(
            rand::gen_range(0.0, WORLD_WIDTH),
            rand::gen_range(0.0, WORLD_HEIGHT),
        );

        if v.distance_squared(avoid_pos) > TOO_CLOSE_SQ {
            return v;
        }
    }
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
        let player_col = match state.player_state() {
            PlayerState::Walk => RED,
            PlayerState::Roll => GOLD,
            PlayerState::Recover => ORANGE,
            PlayerState::Dead => MAROON,
        };

        draw_circle_lines(
            state.player_pos.x,
            state.player_pos.y,
            PLAYER_RADIUS,
            1.0,
            player_col,
        );

        draw_circle_lines(
            state.knife_pos.x,
            state.knife_pos.y,
            KNIFE_RADIUS,
            1.0,
            GREEN,
        );

        for l in &state.lemons {
            let col = if l.is_attacking() { YELLOW } else { GREEN };
            draw_circle(l.pos.x, l.pos.y, LEMON_RADIUS, col);
        }
    }
}
