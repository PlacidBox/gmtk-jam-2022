use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::prelude::*;

pub struct Assets {
    pub bgm: Sound,
    pub roll: Sound,
    pub enemy_shoot: Sound,
    pub enemy_death: Sound,
    pub bread_attack: Sound,

    pub background: Texture2D,
    pub player: Texture2D,
    pub player_weapon: Texture2D,
    pub lemon: Texture2D,
    pub grape: Texture2D,
    pub bullet: Texture2D,
    pub bread: Texture2D,
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
        bread_attack: load_sound_from_bytes(include_bytes!("376860_6886013-lq.ogg")).await?,

        background: Texture2D::from_file_with_format(include_bytes!("background.png"), None),
        player: Texture2D::from_file_with_format(include_bytes!("cook.png"), None),
        player_weapon: Texture2D::from_file_with_format(include_bytes!("playerweapon.png"), None),
        lemon: Texture2D::from_file_with_format(include_bytes!("lemon.png"), None),
        grape: Texture2D::from_file_with_format(include_bytes!("grape.png"), None),
        bullet: Texture2D::from_file_with_format(include_bytes!("strawberry.png"), None),
        bread: Texture2D::from_file_with_format(include_bytes!("bread.png"), None),
    })
}
