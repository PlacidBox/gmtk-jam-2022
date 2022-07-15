use macroquad::audio::Sound;
use macroquad::prelude::*;

static SOUND: &[u8; 17937] = include_bytes!("examples_sound.ogg");

pub struct Assets {
    pub example: Sound,
}

pub async fn load() -> Result<Assets, FileError> {
    // ogg exported from audacity seems to work well.
    let s = macroquad::audio::load_sound_from_bytes(SOUND).await?;

    Ok(Assets { example: s })
}
