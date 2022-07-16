use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::prelude::*;

pub struct Assets {
    pub bgm: Sound,
    pub roll: Sound,
    pub enemy_shoot: Sound,
    pub enemy_death: Sound,

    pub backgroud: Texture2D,
    pub player: Texture2D,
    pub lemon: Texture2D,
}

pub async fn load() -> Result<Assets, FileError> {
    // ogg exported from audacity seems to work well.
    // Texture2D::from_file_with_format(bytes, format)
    Ok(Assets {
        bgm: load_sound_from_bytes(include_bytes!("radmusic.ogg")).await?,
        roll: load_sound_from_bytes(include_bytes!("60013__qubodup__whoosh.ogg")).await?,
        enemy_shoot: load_sound_from_bytes(include_bytes!("245645__unfa__cartoon-pop-clean.ogg"))
            .await?,
        enemy_death: load_sound_from_bytes(include_bytes!("232135__yottasounds__splat-005.ogg"))
            .await?,

        backgroud: Texture2D::from_file_with_format(include_bytes!("background.png"), None),
        player: Texture2D::from_file_with_format(include_bytes!("cook.png"), None),
        lemon: Texture2D::from_file_with_format(include_bytes!("lemon.png"), None),
    })
}
