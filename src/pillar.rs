use std::ops::Index;

use crate::board::{Block, Palette};
use rand::Rng;
#[derive(Clone, Debug)]
pub struct Pillar {
    blocks: Vec<Block>,
}

impl Pillar {
    pub fn new_random<R: Rng>(rng: &mut R, palette: &Palette, len: usize) -> Self {
        Self {
            blocks: (0..len).map(|_| palette.get_random(rng)).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn rot_up(&mut self) {
        self.blocks.rotate_left(1)
    }

    pub fn rot_down(&mut self) {
        self.blocks.rotate_right(1)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Block> {
        self.blocks.iter()
    }
}

impl Index<usize> for Pillar {
    type Output = Block;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.blocks[idx]
    }
}
