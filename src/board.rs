use ggez::graphics::{Color, WHITE};
use glam::{vec2, Vec2};
use rand::Rng;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{
    drawing::rgb,
    game_loop::PilPos,
    helpful_things::{neighbours, search},
    pillar::Pillar,
};

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, Hash, PartialEq)]
pub enum Block {
    Red = 0,
    Green = 1,
    Blue = 2,
    Orange = 3,
    Purple = 4,
    Yellow = 5,
    Cyan = 6,
    Violet = 7,
    White = 8,
    Black = 9,
}

impl Block {
    pub fn color(self) -> Color {
        match self {
            Self::Red => Color::from_rgb_u32(0xFF6961),
            Self::Green => Color::from_rgb_u32(0x77CC77),
            Self::Blue => Color::from_rgb_u32(0x5080EC),
            Self::Orange => Color::from_rgb_u32(0xFFA510),
            Self::Purple => Color::from_rgb_u32(0xBE94E6),
            Self::Yellow => Color::from_rgb_u32(0xFFFD40),
            Self::Cyan => Color::from_rgb_u32(0x85E3FF),
            Self::Violet => rgb(0.5, 0.1, 0.9),
            Self::White => WHITE,
            Self::Black => rgb(0.4, 0.3, 0.3),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Palette {
    blocks: Vec<Block>,
}

impl Palette {
    pub fn new(sz: usize) -> Self {
        Self {
            blocks: (0..sz).filter_map(Block::from_usize).collect(),
        }
    }

    pub fn get_random<R: Rng>(&self, rng: &mut R) -> Block {
        let idx = rng.gen_range(0, self.blocks.len());
        self.blocks[idx]
    }

    /// add another block color to palette,
    /// returns false if we've run out of colors.
    pub fn expand(&mut self) -> bool {
        if let Some(next) = Block::from_usize(self.blocks.len()) {
            self.blocks.push(next);
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum BoardCell {
    OutOfBounds,
    Empty,
    Contains(Block),
}

pub struct Board {
    blocks: Vec<Vec<Option<Block>>>,
}

impl Board {
    pub fn new(sz: [usize; 2]) -> Self {
        assert!(0 < sz[0]);
        assert!(3 < sz[1]);
        let blocks = vec![vec![None; sz[1]]; sz[0]];
        Self { blocks }
    }

    pub fn x_len(&self) -> usize {
        self.blocks.len()
    }

    pub fn y_len(&self) -> usize {
        self.blocks[0].len()
    }

    pub fn get_pp(&self, PilPos { x, y }: PilPos) -> BoardCell {
        if self.x_len() <= x {
            return BoardCell::OutOfBounds;
        }
        if y < 0.0 || self.y_len() as f32 <= y {
            return BoardCell::OutOfBounds;
        }
        let cell = self[[x, y as usize]];
        match cell {
            Some(block) => BoardCell::Contains(block),
            None => BoardCell::Empty,
        }
    }

    pub fn get(&self, [x, y]: [usize; 2]) -> BoardCell {
        if self.x_len() <= x || self.y_len() <= y {
            return BoardCell::OutOfBounds;
        }
        let cell = self[[x, y as usize]];
        match cell {
            Some(block) => BoardCell::Contains(block),
            None => BoardCell::Empty,
        }
    }

    pub fn sz(&self) -> [usize; 2] {
        [self.x_len(), self.y_len()]
    }

    pub fn px_sz(&self, tile_sz: Vec2) -> Vec2 {
        vec2(
            self.x_len() as f32 * tile_sz.x,
            self.y_len() as f32 * tile_sz.y,
        )
    }
}

impl std::ops::Index<[usize; 2]> for Board {
    type Output = Option<Block>;
    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.blocks[idx[0]][idx[1]]
    }
}

impl std::ops::IndexMut<[usize; 2]> for Board {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        &mut self.blocks[idx[0]][idx[1]]
    }
}

impl std::ops::Index<PilPos> for Board {
    type Output = Option<Block>;
    fn index(&self, idx: PilPos) -> &Self::Output {
        assert!(0.0 <= idx.y);
        &self.blocks[idx.x][idx.y as usize]
    }
}

impl std::ops::IndexMut<PilPos> for Board {
    fn index_mut(&mut self, idx: PilPos) -> &mut Self::Output {
        assert!(0.0 <= idx.y);
        &mut self.blocks[idx.x][idx.y as usize]
    }
}

pub fn find_matches(board: &Board, matches_required: u64) -> Vec<([usize; 2], Block)> {
    let mut matching = Vec::new();
    for x in 0..board.x_len() {
        for y in 0..board.y_len() {
            if let BoardCell::Contains(block) = board.get([x, y]) {
                let search_fn = |n: [usize; 2]| {
                    let mut out = vec![];
                    for &m in neighbours(n).iter() {
                        if board.get(m) == BoardCell::Contains(block) {
                            out.push(m);
                        }
                    }
                    out
                };
                let matches = search([x, y], search_fn);
                if matches_required <= matches.len() as u64 {
                    matching.push(([x, y], block));
                }
            }
        }
    }
    matching
}

pub fn write_pillar(board: &mut Board, pillar: &Pillar, PilPos { x, y }: PilPos) {
    let mut cursor = [x, y as usize - pillar.len()];
    for &block in pillar.iter() {
        cursor[1] += 1;
        board[cursor] = Some(block);
    }
}

pub fn fall_down(board: &mut Board) -> bool {
    let mut dropping = true;
    let mut drops = 0;
    while dropping {
        dropping = false;
        for x in 0..board.x_len() {
            for y in 0..board.y_len() - 1 {
                if let Some(block) = board[[x, y]] {
                    if board[[x, y + 1]].is_none() {
                        board[[x, y]] = None;
                        board[[x, y + 1]] = Some(block);
                        dropping = true;
                        drops += 1;
                    }
                }
            }
        }
    }
    drops != 0
}

pub fn is_game_over(board: &Board, death_zone: usize) -> bool {
    for x in 0..board.x_len() {
        for y in 0..=death_zone {
            if board[[x, y]].is_some() {
                return true;
            }
        }
    }
    false
}
