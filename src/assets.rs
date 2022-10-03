use std::cell::RefCell;

use ggez::{audio::SoundData, graphics, Context, GameResult};
use glam::{vec2, Vec2};
use rand::{prelude::StdRng, SeedableRng};

use crate::scoring::{load_high_scores_table, HighScore, HighScoresTable, HIGH_SCORE_PATH};

pub struct Assets {
    pub font: graphics::Font,
    pub block_image: graphics::Image,
    pub high_score_table: RefCell<HighScoresTable<HighScore>>,
    pub rng: RefCell<StdRng>,
    pub tile_sz: Vec2,
    pub game_over_sound: ggez::audio::SoundData,
    pub thud_sound: ggez::audio::SoundData,
    pub score_sound: ggez::audio::SoundData,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Self> {
        let high_scores =
            load_high_scores_table(HIGH_SCORE_PATH).unwrap_or_else(|_| HighScoresTable::new());
        Ok(Self {
            font: graphics::Font::new(ctx, "/ProFontWindows.ttf")?,
            block_image: graphics::Image::new(ctx, "/block_2.png")?,
            high_score_table: RefCell::new(high_scores),
            rng: RefCell::new(StdRng::from_entropy()),
            tile_sz: vec2(32.0, 32.0),
            game_over_sound: ggez::audio::SoundData::new(
                ctx,
                "/mixkit-player-losing-or-failing-2042.wav",
            )?,
            score_sound: SoundData::new(ctx, "/mixkit-winning-a-coin-video-game-2069.wav")?,
            thud_sound: SoundData::new(ctx, "/mixkit-electronic-retro-block-hit-2185.wav")?,
        })
    }
}
