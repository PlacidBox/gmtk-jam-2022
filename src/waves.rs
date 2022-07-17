// enemy wave logic.
#[derive(Copy, Clone)]
pub struct Wave {
    // pair of min/max number to spawn
    pub lemons: (u8, u8),
    pub grapes: (u8, u8),
    pub breads: (u8, u8),
}

impl Wave {
    const fn lems(min: u8, max: u8) -> Self {
        Wave {
            lemons: (min, max),
            grapes: (0, 0),
            breads: (0, 0),
        }
    }
}

impl Default for Wave {
    fn default() -> Self {
        Self {
            lemons: (0, 0),
            grapes: (0, 0),
            breads: (0, 0),
        }
    }
}

static SET_WAVES: [Wave; 6] = [
    Wave::lems(1, 1),
    Wave::lems(1, 1),
    Wave::lems(4, 6),
    Wave::lems(4, 10),
    Wave {
        lemons: (4, 10),
        grapes: (1, 2),
        breads: (1, 1),
    },
    Wave {
        lemons: (1, 2),
        grapes: (3, 3),
        breads: (1, 1),
    },
];

static LATE_GAME_WAVES: [Wave; 2] = [
    Wave::lems(10, 20),
    Wave {
        lemons: (0, 3),
        grapes: (5, 10),
        breads: (0, 2),
    },
];

// set list of waves to introduce the player to the game

// list of end game waves to spawn
pub fn next_wave(wave_num: i32) -> Wave {
    let wave_num = wave_num as usize;

    if wave_num < SET_WAVES.len() {
        return SET_WAVES[wave_num];
    }

    // not totally convinced macroquad's giving up inclusive or exclusive bounds, here
    let wave_num = macroquad::rand::gen_range(0, LATE_GAME_WAVES.len());
    let wave_num = wave_num.clamp(0, LATE_GAME_WAVES.len() - 1);
    LATE_GAME_WAVES[wave_num]
}
