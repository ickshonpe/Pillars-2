use ggez::{
    audio::{SoundSource, Source},
    graphics::Color,
    GameResult,
};
use glam::vec2;

use crate::input::*;
use crate::{
    assets::Assets,
    board::Block,
    drawing::{draw_game_play, DrawingPlans},
    exploding_blocks::ExplodingBlocks,
    game_loop::CommonState,
    game_loop::GameState,
    helpful_things::time_delta,
    magic_puffs,
    timer::Timer,
};

pub struct MatchingBlocks {
    pub common: CommonState,
    pub timer: Timer,
    pub h_s: f32,
    pub match_count: u64,
    pub current_matches: Vec<([usize; 2], Block)>,
}

impl GameState for MatchingBlocks {
    fn update(
        mut self: Box<Self>,
        ctx: &mut ggez::Context,
        assets: &Assets,
        _input_state: &crate::input::InputState<PlayerInput, PlayerCommand>,
        commands: &mut crate::game_loop::Commands,
    ) -> Option<Box<dyn GameState>> {
        let t = time_delta(ctx);
        if self.timer.update(t) {
            if !self.current_matches.is_empty() {
                let mut sound = Source::from_data(ctx, assets.score_sound.clone()).unwrap();
                sound.set_repeat(false);
                sound.play(&ctx).unwrap();
            }
            let tile_sz = vec2(
                assets.block_image.dimensions().w,
                assets.block_image.dimensions().h,
            );
            for &m in &self.current_matches {
                let ([x, y], b) = m;
                let target = vec2((x as f32 + 0.5) * tile_sz.x, (y as f32 + 0.5) * tile_sz.y);
                let color = Color {
                    a: 0.8,
                    ..b.color()
                };
                let mut puffer = |p| commands.add_puff(p);
                let plans = DrawingPlans::new(
                    ctx,
                    tile_sz,
                    self.common.board.sz(),
                    self.common.rules.pillar_spawn_pt.into(),
                    self.common.rules.pillar_sz,
                    vec2(0.0, 0.0),
                );
                magic_puffs::create_puff_plosion_puff(
                    &mut *assets.rng.borrow_mut(),
                    target + plans.board_pos,
                    color,
                    &mut puffer,
                );
            }
            for &(idx, _) in self.current_matches.iter() {
                self.common.board[idx] = None;
            }
            self.match_count += self.current_matches.len() as u64;
            let next_state = ExplodingBlocks {
                common: self.common,
                timer: Timer::new(0.2),
                match_count: self.match_count,
            };
            return Some(Box::new(next_state));
        } else {
            self.h_s += 3.0 * t;
        }
        Some(self)
    }

    fn draw(&self, ctx: &mut ggez::Context, assets: &Assets) -> GameResult {
        let ms: Vec<_> = self.current_matches.iter().map(|&(a, _)| a).collect();
        draw_game_play(ctx, assets, &self.common, None, Some((&ms, self.h_s)))
    }
}
