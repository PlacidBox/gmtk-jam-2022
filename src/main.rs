#![windows_subsystem = "windows"]
mod assets;
mod waves;

// TODO
// - at least 2 new enemies (grapes, bread stick)
//      - bread sticks charge and make a sound, then run at you really quick without changing
//          direction.
// - better background
// - texture everything
// - animate the roll
// - better knife hitbox for the chef, so it's easir to kill things on a diagonals

use assets::Assets;

use macroquad::audio::{play_sound_once, PlaySoundParams};
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

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
const KNIFE_REACH: f32 = 30.0;

const TICKS_BETWEEN_WAVES_MAX: i32 = TICKS_PER_SEC * 3;
const TICKS_BETWEEN_WAVES_MIN: i32 = TICKS_PER_SEC * 5;

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

    let mut tick_time = get_time();

    let mut st = GameState::default();

    loop {
        // Fixed tick rate game loop, with some support for smoothing out the frame rate if there's
        // any hiccups. This is done by tracking how far 'behind' we are (clamped to a reasonable
        // value), and doing multiple logic steps if needed to catch back up.
        let now = get_time();
        if now > tick_time + MAX_TIME_BEHIND {
            tick_time = now - MAX_TIME_BEHIND;
        }

        while tick_time < now {
            tick_time += TICK_RATE;
            tick(&mut st, &ass);
        }

        clear_background(BLACK);
        render(&st, &ass);
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

    next_wave_num: i32,
    next_wave_at_tick: i32,

    player_pos: Vec2,
    player_dir: Vec2,
    // which tick the player ceases rolling, and starts recovering from the roll
    player_rolling_until: i32,

    // knife keeps its own dir, so that it doesn't get set back to 0,0 when hte player stops moving
    knife_pos: Vec2,
    knife_dir: Vec2,

    lemons: Vec<Lemon>,
    grapes: Vec<Grape>,
    bullets: Vec<Bullet>,
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

    fn spawn_lemon(&mut self) {
        if self.lemons.len() >= LEMONS_MAX {
            return;
        }

        let new_lemon = Lemon::new(rand_spawn_pos(self.player_pos));
        self.lemons.push(new_lemon);
    }

    fn spawn_grape(&mut self) {
        if self.grapes.len() >= GRAPES_MAX {
            return;
        }

        let new_grape = Grape::new(rand_spawn_pos(self.player_pos));
        self.grapes.push(new_grape);
    }
}

impl Default for GameState {
    fn default() -> Self {
        let world_centre = vec2(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
        Self {
            game_over: false,
            tick: 0,

            next_wave_num: 0,
            next_wave_at_tick: 0,

            player_pos: world_centre,
            player_dir: vec2(0.0, 0.0),
            // dirty hack to start the player not in recovery mode
            player_rolling_until: -PLAYER_ROLL_RECOVERY_TICKS,

            knife_pos: world_centre,
            knife_dir: vec2(1.0, 0.0),

            lemons: Vec::with_capacity(LEMONS_MAX),
            grapes: Vec::with_capacity(GRAPES_MAX),
            bullets: Vec::new(),
        }
    }
}

fn tick(state: &mut GameState, ass: &Assets) {
    if state.game_over {
        if is_key_down(KeyCode::R) {
            *state = GameState::default();
        }
        return;
    }

    state.tick += 1;

    tick_player(state, ass);
    tick_knife(state);
    tick_check_enemy_death(state, ass);
    tick_spawner(state);
    tick_enemies(state, ass);

    if state.player_state() != PlayerState::Roll && check_player_death(state) {
        state.game_over = true;
    }
}

fn tick_player(state: &mut GameState, ass: &Assets) {
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
            play_sound_once(ass.roll);
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
    state.knife_pos = state.player_pos + state.knife_dir * KNIFE_REACH;
}

fn tick_check_enemy_death(state: &mut GameState, ass: &Assets) {
    const LEMON_KILL_DIST_SQ: f32 = KNIFE_RADIUS * KNIFE_RADIUS + LEMON_RADIUS * LEMON_RADIUS;
    const GRAPE_KILL_DIST_SQ: f32 = KNIFE_RADIUS * KNIFE_RADIUS + GRAPE_RADIUS * GRAPE_RADIUS;

    let kill_zone = state.knife_pos;

    let initial_lem_len = state.lemons.len();
    let initial_grape_len = state.grapes.len();

    state
        .lemons
        .retain(|l| l.pos.distance_squared(kill_zone) > LEMON_KILL_DIST_SQ);

    state
        .grapes
        .retain(|g| g.pos.distance_squared(kill_zone) > GRAPE_KILL_DIST_SQ);

    // bullets aren't killed by knife, but when going out of bounds
    state
        .bullets
        .retain(|b| b.pos >= vec2(0.0, 0.0) && b.pos <= vec2(WORLD_WIDTH, WORLD_HEIGHT));

    let any_lemons_died = state.lemons.len() != initial_lem_len;
    let any_grapes_died = state.grapes.len() != initial_grape_len;

    if any_lemons_died || any_grapes_died {
        play_sound_once(ass.enemy_death);
    }
}

fn tick_spawner(state: &mut GameState) {
    let spawn_wave = state.tick >= state.next_wave_at_tick;
    if !spawn_wave {
        return;
    }

    let nw = waves::next_wave(state.next_wave_num);
    state.next_wave_num += 1;
    state.next_wave_at_tick =
        state.tick + gen_range(TICKS_BETWEEN_WAVES_MIN, TICKS_BETWEEN_WAVES_MAX);

    let num_lemons = gen_range(nw.lemons.0, nw.lemons.1);
    let num_grapes = gen_range(nw.grapes.0, nw.grapes.1);

    for _ in 0..num_lemons {
        state.spawn_lemon();
    }

    for _ in 0..num_grapes {
        state.spawn_grape();
    }
}

fn tick_enemies(state: &mut GameState, ass: &Assets) {
    for l in &mut state.lemons {
        l.tick(state.player_pos);
    }

    for g in &mut state.grapes {
        g.tick(ass, state.player_pos, &mut state.bullets);
    }

    for b in &mut state.bullets {
        b.tick();
    }
}

fn check_player_death(state: &GameState) -> bool {
    const LEMON_KILL_DIST_SQ: f32 = PLAYER_RADIUS * PLAYER_RADIUS + LEMON_RADIUS * LEMON_RADIUS;
    const BULLET_KILL_DIST_SQ: f32 = PLAYER_RADIUS * PLAYER_RADIUS + BULLET_RADIUS * BULLET_RADIUS;

    let kill_zone = state.player_pos;

    for l in &state.lemons {
        if l.pos.distance_squared(kill_zone) < LEMON_KILL_DIST_SQ {
            return true;
        }
    }

    for b in &state.bullets {
        if b.pos.distance_squared(kill_zone) < BULLET_KILL_DIST_SQ {
            return true;
        }
    }

    false
}

// an enemy that starts as a lime, wanders for a bit, then begins to charge the player aggressively
// after turning in to a lemon
const LEMONS_MAX: usize = 64;
const LEMON_SPEED_WANDER: f32 = 0.5;
const LEMON_WANDER_CLOSE: f32 = 10.0;
const LEMON_WANDER_SQ: f32 = LEMON_WANDER_CLOSE * LEMON_WANDER_CLOSE;
const LEMON_SPEED_ATTACK: f32 = 2.0;
const LEMON_ATTACKS_AFTER_MIN: i32 = TICKS_PER_SEC * 3;
const LEMON_ATTACKS_AFTER_MAX: i32 = TICKS_PER_SEC * 20;
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
            attacks_in: gen_range(LEMON_ATTACKS_AFTER_MIN, LEMON_ATTACKS_AFTER_MAX),
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
    }

    fn is_attacking(&self) -> bool {
        self.attacks_in == 0
    }
}

const GRAPES_MAX: usize = 32;
const GRAPE_ATTACKS_AFTER_MIN: i32 = 5 * TICKS_PER_SEC;
const GRAPE_ATTACKS_AFTER_MAX: i32 = 15 * TICKS_PER_SEC;
const GRAPE_RADIUS: f32 = 25.0;
const GRAPE_NO_SHOOT_WITHIN: f32 = 100.0;
const GRAPE_NO_SHOOT_WITHIN_SQ: f32 = GRAPE_NO_SHOOT_WITHIN * GRAPE_NO_SHOOT_WITHIN;
struct Grape {
    pos: Vec2,
    attacks_in: i32,
}

impl Grape {
    fn new(spawn_point: Vec2) -> Self {
        Self {
            pos: spawn_point,
            attacks_in: gen_range(GRAPE_ATTACKS_AFTER_MIN, GRAPE_ATTACKS_AFTER_MAX),
        }
    }

    fn tick(&mut self, ass: &Assets, player_pos: Vec2, bullets: &mut Vec<Bullet>) {
        self.attacks_in -= 1;
        if self.attacks_in > 0 {
            return;
        }

        self.attacks_in = gen_range(GRAPE_ATTACKS_AFTER_MIN, GRAPE_ATTACKS_AFTER_MAX);

        // if the player is fairly close to the grape, don't shoot. avoids 'instantly' killing the
        // player.
        if self.pos.distance_squared(player_pos) < GRAPE_NO_SHOOT_WITHIN_SQ {
            return;
        }

        let target_off = vec2(
            gen_range(-BULLET_RNG, BULLET_RNG),
            gen_range(-BULLET_RNG, BULLET_RNG),
        );
        let target_pos = player_pos + target_off;
        let bull_dir = (target_pos - self.pos).normalize();

        bullets.push(Bullet {
            pos: self.pos,
            dir: bull_dir * BULLET_SPEED,
        });

        play_sound_once(ass.enemy_shoot);
    }
}

// inaccuracy when shooting bullets from grapes
const BULLET_RNG: f32 = 60.0;
const BULLET_RADIUS: f32 = 13.0;
const BULLET_SPEED: f32 = 5.0;
struct Bullet {
    pos: Vec2,
    dir: Vec2,
}

impl Bullet {
    fn tick(&mut self) {
        self.pos += self.dir;
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
        let v = vec2(gen_range(0.0, WORLD_WIDTH), gen_range(0.0, WORLD_HEIGHT));

        if v.distance_squared(avoid_pos) > TOO_CLOSE_SQ {
            return v;
        }
    }
}

fn render(state: &GameState, ass: &Assets) {
    draw_texture(ass.background, 0.0, 0.0, WHITE);

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

    if state.game_over {
        let score_text = format!("You lasted {0} seconds", state.tick / TICKS_PER_SEC);
        draw_text("Game over. Press R to restart", 300.0, 300.0, 30.0, WHITE);
        draw_text(&score_text, 400.0, 330.0, 30.0, WHITE);
    }

    let player_params = DrawTextureParams {
        dest_size: Some(vec2(PLAYER_RADIUS, PLAYER_RADIUS) * 2.0),
        ..Default::default()
    };
    let player_col = match state.player_state() {
        PlayerState::Walk => WHITE,
        PlayerState::Roll => WHITE,
        PlayerState::Recover => GOLD,
        PlayerState::Dead => MAROON,
    };
    draw_texture_ex(
        ass.player,
        state.player_pos.x - PLAYER_RADIUS,
        state.player_pos.y - PLAYER_RADIUS,
        player_col,
        player_params,
    );

    if DEBUG_VIEW {
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
    }

    let lem_params = DrawTextureParams {
        dest_size: Some(vec2(LEMON_RADIUS, LEMON_RADIUS) * 2.0),
        ..Default::default()
    };
    for l in &state.lemons {
        draw_texture_ex(
            ass.lemon,
            l.pos.x - LEMON_RADIUS,
            l.pos.y - LEMON_RADIUS,
            WHITE,
            lem_params.clone(),
        );
    }

    let grape_params = DrawTextureParams {
        dest_size: Some(vec2(GRAPE_RADIUS, GRAPE_RADIUS) * 2.0),
        ..Default::default()
    };
    for g in &state.grapes {
        draw_texture_ex(
            ass.grape,
            g.pos.x - GRAPE_RADIUS,
            g.pos.y - GRAPE_RADIUS,
            WHITE,
            grape_params.clone(),
        );
    }

    let bull_params = DrawTextureParams {
        dest_size: Some(vec2(BULLET_RADIUS, BULLET_RADIUS) * 2.0),
        ..Default::default()
    };
    for b in &state.bullets {
        draw_texture_ex(
            ass.bullet,
            b.pos.x - BULLET_RADIUS,
            b.pos.y - BULLET_RADIUS,
            WHITE,
            bull_params.clone(),
        );
    }
}
