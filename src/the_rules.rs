use crate::{board::Palette, game_loop::PilPos};

#[derive(Clone, Debug)]
pub struct GameRules {
    pub matches_required: u64,
    pub drops_per_level: u64,
    pub pillar_sz: usize,
    pub board_sz: [usize; 2],
    pub initial_palette: Palette,
    pub pillar_spawn_pt: PilPos,
    pub initial_fall_rate: f32,
    pub fall_rate_increment: f32,
    pub max_fall_rate: f32,
    pub rot_cooldown: f32,
    pub horizontal_move_cooldown: f32
}

impl Default for GameRules {
    fn default() -> Self {
        let pillar_sz = 3;
        Self {
            matches_required: 3,
            drops_per_level: 10,
            pillar_sz,
            board_sz: [7, 16],
            initial_palette: Palette::new(3),
            pillar_spawn_pt: PilPos {
                x: 3,
                y: pillar_sz as f32,
            },
            initial_fall_rate: 2.4,
            fall_rate_increment: 0.2,
            max_fall_rate: 25.0,
            rot_cooldown: 0.15,
            horizontal_move_cooldown: 0.1,
            
        }
    }
}
