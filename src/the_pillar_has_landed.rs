use crate::{
    board::{find_matches, is_game_over, write_pillar},
    drawing::draw_game_play,
    game_is_over::GameIsOver,
    game_loop::{increase_level, CommonState, GameState, PilPos},
    helpful_things::time_delta,
    input::PlayerCommand,
    matching_blocks::MatchingBlocks,
    pillar::Pillar,
    the_pillar_descending::ThePillarIsFalling,
    timer::Timer,
};
use ggez::{
    audio::{SoundSource, Source},
    GameResult,
};

pub struct ThePillarHasLanded {
    pub common: CommonState,
    pub current_pillar: Pillar,
    pub pillar_pos: PilPos,
    pub rot_cooldown: f32,
    pub rot_timer: Timer,
    /// time left until pillar is fixed
    pub grace_period: Timer,
}

impl GameState for ThePillarHasLanded {
    fn update(
        mut self: Box<Self>,
        ctx: &mut ggez::Context,
        assets: &crate::assets::Assets,
        input_state: &crate::input::InputState<
            crate::input::PlayerInput,
            crate::input::PlayerCommand,
        >,
        commands: &mut crate::game_loop::Commands,
    ) -> Option<Box<dyn GameState>> {
        let time_delta = time_delta(ctx);
        if self.rot_timer.update(time_delta) {
            if input_state.just_active(PlayerCommand::RotUp) {
                self.current_pillar.rot_up();
                self.rot_timer.set(self.rot_cooldown);
            } else if input_state.just_active(PlayerCommand::RotDown) {
                self.current_pillar.rot_down();
                self.rot_timer.set(self.rot_cooldown);
            }
        }

        if self.grace_period.update(time_delta) {
            write_pillar(
                &mut self.common.board,
                &self.current_pillar,
                self.pillar_pos,
            );

            self.common.drop_count += 1;
            if self.common.drop_count % self.common.rules.drops_per_level == 0 {
                increase_level(&mut self.common);
            }

            let current_matches: Vec<_> =
                find_matches(&self.common.board, self.common.rules.matches_required)
                    .into_iter()
                    .collect();

            if !current_matches.is_empty() {
                let next_state = MatchingBlocks {
                    common: self.common,
                    match_count: 0,
                    timer: Timer::new(4f32.recip()),
                    h_s: 0.0,
                    current_matches,
                };
                return Some(Box::new(next_state));
            } else if is_game_over(&self.common.board, self.common.rules.pillar_sz) {
                let mut sound = Source::from_data(ctx, assets.game_over_sound.clone()).unwrap();
                sound.set_repeat(false);
                sound.play(&ctx).unwrap();
                let next_state = GameIsOver::new(self.common, assets, commands.get_score());
                return Some(Box::new(next_state));
            } else {
                let next_state =
                    ThePillarIsFalling::new(self.common, &mut *assets.rng.borrow_mut());
                return Some(Box::new(next_state));
            }
        }

        Some(self)
    }

    fn draw(&self, ctx: &mut ggez::Context, assets: &crate::assets::Assets) -> GameResult {
        draw_game_play(
            ctx,
            assets,
            &self.common,
            Some((&self.current_pillar, self.pillar_pos.x, self.pillar_pos.y)),
            None,
        )
    }
}
