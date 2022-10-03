use crate::{
    board::{Board, BoardCell},
    drawing::draw_game_play,
    input::PlayerCommand,
    pillar::Pillar,
    the_pillar_has_landed::ThePillarHasLanded,
    the_rules::GameRules,
    timer::Timer,
};
use ggez::{
    audio::{SoundSource, Source},
    Context, GameResult,
};

use crate::assets::Assets;
use crate::game_loop::*;
use crate::helpful_things::*;
use crate::input::*;
use rand::Rng;

pub struct ThePillarIsFalling {
    common: CommonState,
    pillar_pos: PilPos,
    current_pillar: Pillar,
    held_down_flag: bool,
    rot_cooldown: f32,
    rot_timer: Timer,
    horizontal_move_timer: Timer,
    horizontal_move_cooldown: f32,
}

impl ThePillarIsFalling {
    pub fn new_game<R: Rng>(rules: GameRules, rng: &mut R) -> Self {
        let board = Board::new([rules.board_sz[0], rules.board_sz[1] + rules.pillar_sz + 1]);
        let current_pillar = Pillar::new_random(rng, &rules.initial_palette, rules.pillar_sz);
        let next_pillar = Pillar::new_random(rng, &rules.initial_palette, rules.pillar_sz);
        let pillar_pos = rules.pillar_spawn_pt;
        let palette = rules.initial_palette.clone();
        let pillar_fall_rate = rules.initial_fall_rate;
        let common = CommonState {
            rules,
            board,
            next_pillar,
            palette,
            level: 1,
            puffs: vec![],
            drop_count: 0,
            pillar_fall_rate,
        };

        Self {
            common,
            pillar_pos,
            current_pillar,
            held_down_flag: true,
            rot_cooldown: 0.2,
            rot_timer: Timer::new(0.0),
            horizontal_move_timer: Timer::new(0.0),
            horizontal_move_cooldown: 0.1,
        }
    }

    pub fn new<R: Rng>(mut common: CommonState, rng: &mut R) -> Self {
        let current_pillar = common.next_pillar.clone();

        common.next_pillar = Pillar::new_random(rng, &common.palette, common.rules.pillar_sz);
        let pillar_pos = common.rules.pillar_spawn_pt;
        Self {
            common,
            pillar_pos,
            current_pillar,
            held_down_flag: true,
            rot_cooldown: 0.2,
            rot_timer: Timer::new(0.0),
            horizontal_move_timer: Timer::new(0.0),
            horizontal_move_cooldown: 0.1,
        }
    }
}

impl GameState for ThePillarIsFalling {
    fn update(
        mut self: Box<Self>,
        ctx: &mut Context,
        assets: &Assets,
        input_state: &InputState<PlayerInput, PlayerCommand>,
        _commands: &mut Commands,
    ) -> Option<Box<dyn GameState>> {
        let time_delta = time_delta(ctx);
        if self.held_down_flag && input_state.inactive(PlayerCommand::MoveDown) {
            self.held_down_flag = false;
        }

        if self.rot_timer.update(time_delta) {
            if input_state.just_active(PlayerCommand::RotUp) {
                self.current_pillar.rot_up();
                self.rot_timer.set(self.rot_cooldown);
            } else if input_state.just_active(PlayerCommand::RotDown) {
                self.current_pillar.rot_down();
                self.rot_timer.set(self.rot_cooldown);
            }
        }

        if self.horizontal_move_timer.update(time_delta) {
            let new_pillar_x = if input_state.active(PlayerCommand::MoveLeft)
                && !input_state.active(PlayerCommand::MoveRight)
            {
                self.pillar_pos.x.saturating_sub(1)
            } else if !input_state.active(PlayerCommand::MoveLeft)
                && input_state.active(PlayerCommand::MoveRight)
            {
                self.pillar_pos.x + 1
            } else {
                self.pillar_pos.x
            };
            if is_horizontal_move_valid(new_pillar_x, self.pillar_pos.y, &self.common.board) {
                if new_pillar_x != self.pillar_pos.x {
                    self.horizontal_move_timer
                        .set(self.horizontal_move_cooldown);
                }
                self.pillar_pos.x = new_pillar_x;
            }
        } else {
            let new_pillar_x = if input_state.just_active(PlayerCommand::MoveLeft)
                && !input_state.active(PlayerCommand::MoveRight)
            {
                self.pillar_pos.x.saturating_sub(1)
            } else if !input_state.active(PlayerCommand::MoveLeft)
                && input_state.just_active(PlayerCommand::MoveRight)
            {
                self.pillar_pos.x + 1
            } else {
                self.pillar_pos.x
            };
            if is_horizontal_move_valid(new_pillar_x, self.pillar_pos.y, &self.common.board) {
                if new_pillar_x != self.pillar_pos.x {
                    self.horizontal_move_timer
                        .set(self.horizontal_move_cooldown)
                }
                self.pillar_pos.x = new_pillar_x;
            }
        }

        let next_pillar_y = self.pillar_pos.y
            + if input_state.active(PlayerCommand::MoveDown) && !self.held_down_flag {
                time_delta * self.common.rules.max_fall_rate
            } else {
                time_delta
                    * self
                        .common
                        .pillar_fall_rate
                        .min(self.common.rules.max_fall_rate)
            };

        if let BoardCell::Empty = self.common.board.get_pp(PilPos {
            x: self.pillar_pos.x,
            y: next_pillar_y,
        }) {
            self.pillar_pos.y = next_pillar_y;
        } else {
            let mut sound = Source::from_data(ctx, assets.thud_sound.clone()).unwrap();
            sound.set_repeat(false);
            sound.play(&ctx).unwrap();
            self.pillar_pos.y = self.pillar_pos.y.ceil() - 0.0001;
            let next = ThePillarHasLanded {
                common: self.common,
                current_pillar: self.current_pillar,
                pillar_pos: self.pillar_pos,
                rot_cooldown: self.rot_cooldown,
                rot_timer: self.rot_timer,
                grace_period: Timer::new(0.3),
            };
            return Some(Box::new(next));
        }
        Some(self)
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult {
        draw_game_play(
            ctx,
            assets,
            &self.common,
            Some((&self.current_pillar, self.pillar_pos.x, self.pillar_pos.y)),
            None,
        )
    }
}

fn is_horizontal_move_valid(new_pillar_x: usize, pillar_y: f32, board: &Board) -> bool {
    new_pillar_x < board.x_len()
        && (pillar_y < 0.0 || {
            let y_idx = pillar_y as usize;
            board[[new_pillar_x, y_idx]].is_none()
        })
}
