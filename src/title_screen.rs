use ggez::graphics::Color;
use ggez::{event, graphics, GameResult};

use crate::{assets::Assets, the_rules::GameRules};
use crate::{
    drawing::{draw_centered_text, lerp_color, YELLOW},
    game_loop::GameState,
    input::{PlayerCommand, PlayerInput},
    the_pillar_descending::ThePillarIsFalling,
};

pub struct TitleScreen {}

impl TitleScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameState for TitleScreen {
    fn update(
        self: Box<Self>,
        ctx: &mut ggez::Context,
        assets: &Assets,
        input_state: &crate::input::InputState<PlayerInput, PlayerCommand>,
        commands: &mut crate::game_loop::Commands,
    ) -> Option<Box<dyn GameState>> {
        if input_state.just_active(PlayerCommand::Quit) {
            event::quit(ctx);
        }
        if input_state.just_active(PlayerCommand::Start) {
            commands.reset_score();
            let new_state =
                ThePillarIsFalling::new_game(GameRules::default(), &mut *assets.rng.borrow_mut());
            return Some(Box::new(new_state));
        }

        Some(self)
    }

    fn draw(&self, ctx: &mut ggez::Context, assets: &Assets) -> GameResult {
        let mut cursor_y = 100.0;
        let title_message = graphics::Text::new(("Pillars", assets.font, 40.0));
        draw_centered_text(ctx, &title_message, cursor_y, Color::WHITE)?;

        cursor_y += 60.0;
        let scores = assets.high_score_table.borrow();

        for score in scores.list().iter().take(10) {
            let score_text = graphics::Text::new((
                format!("{:>2}. {} {:>8}", score.level, score.handle, score.score),
                assets.font,
                25.0,
            ));

            draw_centered_text(ctx, &score_text, cursor_y, Color::WHITE)?;
            cursor_y += 20.0;
        }

        let begin_text = graphics::Text::new(("press start", assets.font, 30.0));
        let y_begin = graphics::screen_coordinates(ctx).bottom() - 60.0;
        let begin_color = lerp_color(
            Color::WHITE,
            YELLOW,
            ggez::timer::time_since_start(ctx).as_secs_f32().sin().abs(),
        );
        draw_centered_text(ctx, &begin_text, y_begin, begin_color)?;

        Ok(())
    }
}
