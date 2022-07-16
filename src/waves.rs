// enemy wave logic.
#[derive(Copy, Clone)]
pub struct Wave {
    // pair of min/max number to spawn
    pub lemons: (u8, u8),
}

impl Wave {
    const fn lems(min: u8, max: u8) -> Self {
        Wave {lemons: (min, max)}
    }
}

static SET_WAVES: [Wave; 5] = [
    Wave::lems(1, 1),
    Wave::lems(1, 1),
    Wave::lems(4, 6),
    Wave::lems(4, 10),
    Wave::lems(7, 10),
];

static LATE_GAME_WAVES: [Wave; 1] = [
    Wave::lems(10, 20),
];

// set list of waves to introduce the player to the game

// list of end game waves to spawn
pub fn next_wave(wave_num: i32) -> Wave {
    let wave_num = wave_num as usize;

    if wave_num < SET_WAVES.len() {
        return SET_WAVES[wave_num]
    }

    // not totally convinced macroquad's giving up inclusive or exclusive bounds, here
    let wave_num = macroquad::rand::gen_range(0, LATE_GAME_WAVES.len());
    let wave_num = wave_num.clamp(0, LATE_GAME_WAVES.len() - 1);
    LATE_GAME_WAVES[wave_num]
}
