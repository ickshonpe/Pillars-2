use crate::input::*;
use crate::{assets::Assets, the_rules::GameRules};
use crate::{
    board::*, helpful_things::time_delta, magic_puffs::Puff, pillar::Pillar, title_screen,
};
use ggez::{
    event::{self, Button, EventHandler, KeyCode},
    graphics::{self, DrawMode, MeshBuilder},
    Context, GameResult,
};
use glam::{vec2, Vec2};
pub trait GameState {
    fn update(
        self: Box<Self>,
        ctx: &mut Context,
        assets: &Assets,
        input_state: &InputState<PlayerInput, PlayerCommand>,
        commands: &mut Commands,
    ) -> Option<Box<dyn GameState>>;
    fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult;
}

pub struct Commands {
    points_accum: u64,
    puffer: Vec<Puff>,
    current_score: u64,
    reset_score: bool,
}

impl Commands {
    pub fn add_to_score(&mut self, points: u64) {
        self.points_accum += points;
    }

    pub fn add_puff(&mut self, puff: Puff) {
        self.puffer.push(puff);
    }

    pub fn get_score(&self) -> u64 {
        self.current_score + self.points_accum
    }

    pub fn reset_score(&mut self) {
        self.reset_score = true;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PilPos {
    pub x: usize,
    pub y: f32,
}

impl From<Vec2> for PilPos {
    fn from(v: Vec2) -> Self {
        Self {
            x: v.x as usize,
            y: v.y,
        }
    }
}

impl From<PilPos> for Vec2 {
    fn from(p: PilPos) -> Self {
        vec2(p.x as f32, p.y)
    }
}

impl From<PilPos> for (usize, f32) {
    fn from(PilPos { x, y }: PilPos) -> Self {
        (x, y)
    }
}

pub fn pilpos(x: usize, y: f32) -> PilPos {
    PilPos { x, y }
}

pub struct CommonState {
    pub rules: GameRules,
    pub board: Board,
    pub next_pillar: Pillar,
    pub palette: Palette,
    pub level: u64,
    pub puffs: Vec<Puff>,
    // pub pillar_spawn_x: usize,
    // pub pillar_spawn_y: f32,
    // pub matches_required: u64,
    //    pub pillar_sz: usize,
    pub drop_count: u64,
    // pub drops_per_level: u64,
    pub pillar_fall_rate: f32,
    // pub fall_rate_increment: f32,
    // pub fall_rate_max: f32,
}

pub fn increase_level(common: &mut CommonState) {
    common.level += 1;
    match common.level % 3 {
        0 | 1 => {
            common.pillar_fall_rate = (common.pillar_fall_rate + common.rules.fall_rate_increment)
                .min(common.rules.max_fall_rate);
        }
        2 => {
            common.palette.expand();
        }
        _ => {}
    }
}

pub struct GameLoop {
    assets: Assets,
    input_state: InputState<PlayerInput, PlayerCommand>,
    current_score: u64,
    high_score: u64,
    state: Option<Box<dyn GameState>>,
    puffs: Vec<Puff>,
}

impl GameLoop {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = Assets::load(ctx)?;
        let high_score = assets
            .high_score_table
            .borrow()
            .best()
            .map_or(0, |h| h.score);
        let state = title_screen::TitleScreen::new();
        Ok(Self {
            assets,
            input_state: InputState::new(default_input_cfg()),
            current_score: 0,
            high_score,
            state: Some(Box::new(state)),
            puffs: vec![],
        })
    }
}

impl EventHandler for GameLoop {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut commands = Commands {
            current_score: self.current_score,
            points_accum: 0,
            puffer: vec![],
            reset_score: false,
        };
        let next_state =
            self.state
                .take()
                .unwrap()
                .update(ctx, &self.assets, &self.input_state, &mut commands);
        self.current_score += commands.points_accum;
        if self.high_score < self.current_score {
            self.high_score = self.current_score;
        }
        if commands.reset_score {
            self.current_score = 0;
        }
        if let Some(s) = next_state {
            self.state = Some(s);
        } else {
            event::quit(ctx);
        }
        for puff in &mut self.puffs {
            puff.update(time_delta(ctx));
        }
        self.puffs.extend(commands.puffer);
        self.puffs.retain(|p| 0.0 < p.life_time);

        self.input_state.save_current();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.16, 0.16, 0.2, 0.2].into());
        self.state.as_ref().unwrap().draw(ctx, &self.assets)?;

        if !self.puffs.is_empty() {
            let mut primitives_mesh_builder = MeshBuilder::new();
            for puff in self.puffs.iter() {
                primitives_mesh_builder.circle(
                    DrawMode::fill(),
                    puff.pos,
                    puff.radius,
                    0.1,
                    puff.color,
                )?;
            }
            let mesh = primitives_mesh_builder.build(ctx)?;
            graphics::draw(ctx, &mesh, (Vec2::ZERO,))?;
        }

        let score_text =
            graphics::Text::new((format!("{:07}", self.current_score), self.assets.font, 25.0));
        let high_score_text =
            graphics::Text::new((format!("{:07}", self.high_score), self.assets.font, 25.0));
        let margin = vec2(20., 20.0);
        graphics::draw(ctx, &score_text, (margin,))?;
        let top_right = vec2(
            graphics::screen_coordinates(ctx).right(),
            graphics::screen_coordinates(ctx).top(),
        );
        let target =
            top_right + vec2(-margin.x, margin.y) - high_score_text.width(ctx) as f32 * Vec2::X;
        graphics::draw(ctx, &high_score_text, (target,))?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
        repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            event::quit(ctx)
        } else if !repeat {
            self.input_state.activate(PlayerInput::Key(keycode));
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        keycode: KeyCode,
        _keymods: event::KeyMods,
    ) {
        self.input_state.deactivate(PlayerInput::Key(keycode));
    }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, _character: char) {}

    fn gamepad_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: Button,
        _id: event::GamepadId,
    ) {
        self.input_state.activate(PlayerInput::Button(button));
    }

    fn gamepad_button_up_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: Button,
        _id: event::GamepadId,
    ) {
        self.input_state.deactivate(PlayerInput::Button(button));
    }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut ggez::Context,
        axis: event::Axis,
        value: f32,
        _id: event::GamepadId,
    ) {
        let dead_zone = 0.25;
        if dead_zone < value.abs() {
            match axis {
                event::Axis::LeftStickX => {}
                event::Axis::LeftStickY => {}
                _ => {}
            }
        }
    }
}
