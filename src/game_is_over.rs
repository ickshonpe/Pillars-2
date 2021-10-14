use ggez::{
    graphics::{self, Color, DrawMode, WHITE},
    GameResult,
};
use glam::{vec2, Vec2};
use graphics::{MeshBuilder, TextFragment};

use crate::input::*;
use crate::{
    assets::Assets,
    drawing::{draw_centered_text, draw_game_play, lerp_color, RED, YELLOW},
    game_loop::{CommonState, GameState},
    helpful_things::{time_delta, Center},
    scoring::{save_high_score_table, HighScore, HIGH_SCORE_PATH},
    timer::Timer,
    title_screen::TitleScreen,
};

pub struct GameIsOver {
    pub common: CommonState,
    pub timer: Timer,
    pub name: [usize; 3],
    pub cursor: usize,
    pub cursor_cooldown: f32,
    pub fade: f32,
    pub rank: usize,
}

impl GameIsOver {
    pub fn new(common: CommonState, assets: &Assets, score: u64) -> Self {
        let rank = 1 + assets.high_score_table.borrow().find_position(&HighScore {
            score,
            level: common.level,
            handle: "".to_owned(),
        });
        let cs = assets
            .high_score_table
            .borrow()
            .prev()
            .as_ref()
            .map_or([0; 3], |h| {
                let abet: Vec<char> = ('a'..='z').collect();
                let chars = h.handle.chars().take(3);
                let ons: Vec<_> = chars
                    .map(|c| abet.binary_search(&c).map_or(0, |i| i + 1))
                    .collect();
                [ons[0], ons[1], ons[2]]
            });

        Self {
            common,
            timer: Timer::new(0.0),
            name: cs,
            cursor: 0,
            cursor_cooldown: 0.25,
            fade: 0.0,
            rank,
        }
    }
}

impl GameState for GameIsOver {
    fn update(
        mut self: Box<Self>,
        ctx: &mut ggez::Context,
        assets: &Assets,
        input_state: &crate::input::InputState<PlayerInput, PlayerCommand>,
        commands: &mut crate::game_loop::Commands,
    ) -> Option<Box<dyn GameState>> {
        self.fade = (self.fade + 0.5 * time_delta(ctx)).min(0.9);
        if input_state.just_active(PlayerCommand::Start) {
            let [a, b, c] = self.name;
            if 0 < a && 0 < b && 0 < c {
                let chars: Vec<char> = ('a'..='z').collect();
                let name = [chars[a - 1], chars[b - 1], chars[c - 1]];
                let h = HighScore {
                    score: commands.get_score(),
                    level: self.common.level,
                    handle: name.iter().collect(),
                };
                let mut high_scores = assets.high_score_table.borrow_mut();
                high_scores.insert(h);
                save_high_score_table(HIGH_SCORE_PATH, &high_scores).unwrap();
                let next_state = TitleScreen::new();
                return Some(Box::new(next_state));
            }
        }

        if self.timer.update(time_delta(ctx)) {
            if input_state.active(PlayerCommand::MoveLeft) {
                self.cursor = self.cursor.saturating_sub(1);
                self.timer.set(self.cursor_cooldown);
            } else if input_state.active(PlayerCommand::MoveRight) {
                self.cursor += 1;
                self.cursor = self.cursor.min(2);
                self.timer.set(self.cursor_cooldown);
            } else if input_state.active(PlayerCommand::RotUp) {
                self.name[self.cursor] += 1;
                if 26 < self.name[self.cursor] {
                    self.name[self.cursor] = 1;
                }
                self.timer.set(self.cursor_cooldown);
            } else if input_state.active(PlayerCommand::RotDown) {
                self.name[self.cursor] = self.name[self.cursor].saturating_sub(1);
                if self.name[self.cursor] == 0 {
                    self.name[self.cursor] = 26;
                }
                self.timer.set(self.cursor_cooldown);
            }
        }
        Some(self)
    }

    fn draw(&self, ctx: &mut ggez::Context, assets: &Assets) -> GameResult {
        draw_game_play(ctx, assets, &self.common, None, None)?;
        let rect = graphics::screen_coordinates(ctx);
        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), rect, Color::new(0., 0., 0., self.fade))
            .build(ctx)?;
        graphics::draw(ctx, &mesh, (Vec2::zero(),))?;
        let game_over_message = graphics::Text::new(("GAME OVER", assets.font, 36.0));

        let y = graphics::screen_coordinates(ctx).center().y - 200.0;
        draw_centered_text(
            ctx,
            &game_over_message,
            y,
            lerp_color(
                WHITE,
                RED,
                ggez::timer::time_since_start(ctx).as_secs_f32().sin().abs(),
            ),
        )?;
        let y = y + 100.0;
        let rank_msg = graphics::Text::new((format!("rank #{}", self.rank), assets.font, 36.));
        draw_centered_text(ctx, &rank_msg, y, WHITE)?;

        let chars: Vec<char> = ('a'..='z').collect();
        let t = ggez::timer::time_since_start(ctx).as_secs_f32();
        let text_frags: Vec<_> = self
            .name
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let s = if c == 0 {
                    "_".to_owned()
                } else {
                    chars[c - 1].to_string()
                };
                let c = if i == self.cursor {
                    lerp_color(WHITE, YELLOW, 3.0 * t.sin().abs())
                } else {
                    WHITE
                };
                let f = TextFragment::from((s, assets.font, 36.0)).color(c);
                graphics::Text::new(f)
            })
            .collect();
        let gap = 50.0;
        let w: f32 = text_frags.iter().map(|f| f.width(ctx) as f32).sum::<f32>()
            + (text_frags.len() - 1) as f32 * 50.0;
        let x = graphics::screen_coordinates(ctx).center().x - 0.5 * w;
        let mut target = vec2(x, y + 100.0);
        for f in text_frags {
            graphics::draw(ctx, &f, (target,))?;
            target.x += gap + f.width(ctx) as f32;
        }
        Ok(())
    }
}
