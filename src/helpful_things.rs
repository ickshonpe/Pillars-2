use std::{collections::HashSet, hash::Hash};

use ggez::{graphics, Context};
use glam::{vec2, Mat2, Vec2};
use rand::Rng;

pub trait Center {
    fn center(&self) -> Vec2;
}

pub trait HalfSizeCtx {
    fn half_sz(&self, ctx: &mut Context) -> Vec2;
}

impl Center for graphics::Rect {
    fn center(&self) -> Vec2 {
        vec2(self.x + 0.5 * self.w, self.y + 0.5 * self.h)
    }
}

impl HalfSizeCtx for graphics::Text {
    fn half_sz(&self, ctx: &mut Context) -> Vec2 {
        let dim = self.dimensions(ctx);
        0.5 * vec2(dim.0 as f32, dim.1 as f32)
    }
}

pub fn neighbours([x, y]: [usize; 2]) -> [[usize; 2]; 4] {
    [
        [x + 1, y],
        [x, y + 1],
        [x.wrapping_sub(1), y],
        [x, y.wrapping_sub(1)],
    ]
}

pub fn search<N, F>(start: N, neighbours: F) -> Vec<N>
where
    N: Copy + Hash + Eq,
    F: Fn(N) -> Vec<N>,
{
    let mut open: Vec<N> = vec![start];
    let mut visited: HashSet<N> = HashSet::new();
    while let Some(n) = open.pop() {
        visited.insert(n);
        for m in neighbours(n) {
            if !visited.contains(&m) {
                open.push(m);
            }
        }
    }
    visited.into_iter().collect()
}

pub fn time_delta(ctx: &mut Context) -> f32 {
    ggez::timer::duration_to_f64(ggez::timer::delta(ctx)) as f32
}

pub fn random_vector<R>(rng: &mut R, low: f32, high: f32) -> Vec2
where
    R: Rng,
{
    let angle = rng.gen_range(0.0, std::f32::consts::TAU);
    let m = Mat2::from_angle(angle);
    let _l = rng.gen_range(low, high);
    m * Vec2::unit_x()
}

pub fn random_dir<R>(rng: &mut R) -> Vec2
where
    R: Rng,
{
    let angle = rng.gen_range(0.0, std::f32::consts::TAU);
    let m = Mat2::from_angle(angle);
    m * Vec2::unit_x()
}
