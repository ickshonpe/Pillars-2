use std::time::Duration;

use ggez::{
    audio::{SoundSource, Source},
    GameResult,
};

use crate::input::*;
use crate::{
    assets::Assets,
    board::{fall_down, find_matches},
    drawing::draw_game_play,
    game_loop::*,
    matching_blocks::MatchingBlocks,
    scoring,
    the_pillar_descending::ThePillarIsFalling,
    timer::Timer,
};
use crate::{board::is_game_over, game_is_over::GameIsOver, helpful_things::*};

pub struct ExplodingBlocks {
    pub common: CommonState,
    pub timer: Timer,
    pub match_count: u64,
}

impl GameState for ExplodingBlocks {
    fn update(
        mut self: Box<Self>,
        ctx: &mut ggez::Context,
        assets: &Assets,
        _input_state: &crate::input::InputState<PlayerInput, PlayerCommand>,
        commands: &mut crate::game_loop::Commands,
    ) -> Option<Box<dyn GameState>> {
        let t = time_delta(ctx);
        if self.timer.update(t) {
            if fall_down(&mut self.common.board) {
                let current_matches: Vec<_> =
                    find_matches(&self.common.board, self.common.rules.matches_required)
                        .into_iter()
                        .collect();
                if !current_matches.is_empty() {
                    let next_state = MatchingBlocks {
                        common: self.common,
                        timer: Timer::new(4f32.recip()),
                        h_s: 0.0,
                        match_count: self.match_count,
                        current_matches,
                    };

                    return Some(Box::new(next_state));
                }
            } else {
                // all blocks have already fallen, no new matches
                if is_game_over(&self.common.board, self.common.rules.pillar_sz) {
                    let mut sound = Source::from_data(ctx, assets.game_over_sound.clone()).unwrap();
                    sound.set_fade_in(Duration::from_millis(400));
                    sound.play_detached(&ctx).unwrap();
                    let next_state = GameIsOver::new(self.common, assets, commands.get_score());
                    return Some(Box::new(next_state));
                } else {
                    let points = scoring::calculate_points(self.match_count, self.common.level);
                    commands.add_to_score(points);
                    let next_state =
                        ThePillarIsFalling::new(self.common, &mut *assets.rng.borrow_mut());

                    return Some(Box::new(next_state));
                }
            }
        }
        Some(self)
    }

    fn draw(&self, ctx: &mut ggez::Context, assets: &Assets) -> GameResult {
        draw_game_play(ctx, assets, &self.common, None, None)
    }
}
