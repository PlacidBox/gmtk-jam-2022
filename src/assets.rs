use macroquad::audio::Sound;
use macroquad::prelude::*;

static BGM_DATA: &[u8; 1268437] = include_bytes!("radmusic.ogg");

pub struct Assets {
    pub bgm: Sound,
}

pub async fn load() -> Result<Assets, FileError> {
    // ogg exported from audacity seems to work well.
    Ok(Assets { bgm:  macroquad::audio::load_sound_from_bytes(BGM_DATA).await? })
}
