use macroquad::audio::Sound;
use macroquad::prelude::*;

static BGM: &[u8; 1268437] = include_bytes!("radmusic.ogg");
static ROLL: &[u8; 12263] = include_bytes!("60013__qubodup__whoosh.ogg");
static ENEMY_SHOOT: &[u8; 7665] = include_bytes!("245645__unfa__cartoon-pop-clean.ogg");
static ENEMY_DEATH: &[u8; 8638] = include_bytes!("232135__yottasounds__splat-005.ogg");

pub struct Assets {
    pub bgm: Sound,
    pub roll: Sound,
    pub enemy_shoot: Sound,
    pub enemy_death: Sound,
}

pub async fn load() -> Result<Assets, FileError> {
    // ogg exported from audacity seems to work well.
    Ok(Assets {
        bgm:  macroquad::audio::load_sound_from_bytes(BGM).await?,
        roll:  macroquad::audio::load_sound_from_bytes(ROLL).await?,
        enemy_shoot:  macroquad::audio::load_sound_from_bytes(ENEMY_SHOOT).await?,
        enemy_death:  macroquad::audio::load_sound_from_bytes(ENEMY_DEATH).await?,
     })
}
