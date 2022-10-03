use ggez::graphics::Color;
use glam::Vec2;
use rand::Rng;

use crate::helpful_things::random_dir;

pub struct Puff {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub expand_rate: f32,
    pub color: Color,
    pub fade_rate: f32,
    pub life_time: f32,
}

impl Puff {
    pub fn update(&mut self, t: f32) {
        self.life_time -= t;
        self.pos += t * self.vel;
        self.radius += t * self.expand_rate;
        self.color.a -= t * self.fade_rate;
    }
}

pub fn create_puff_plosion<R>(rng: &mut R, ctr: Vec2, color: Color, out: &mut Vec<Puff>)
where
    R: Rng,
{
    let n = 10;
    let max_r = 10.0;
    for _ in 0..n {
        let r = rng.gen_range(0.0..max_r);
        let d = random_dir(rng);
        let pos = ctr + r * d;
        let vel = rng.gen_range(5.0..30.0) * d;
        let rad = rng.gen_range(1.0..5.0) + (16. - r).log2();
        let expand_rate = rng.gen_range(1.0..30.0);
        let fade_rate = r / 30.0 + 1.3;
        let life_time = 1.0;
        let puff = Puff {
            pos,
            vel,
            radius: rad,
            expand_rate,
            color,
            fade_rate,
            life_time,
        };
        out.push(puff);
    }
}

pub fn create_puff_plosion_puff<R, P>(rng: &mut R, ctr: Vec2, color: Color, out: &mut P)
where
    R: Rng,
    P: FnMut(Puff),
{
    let n = 10;
    let max_r = 10.0;
    for _ in 0..n {
        let r = rng.gen_range(0.0..max_r);
        let d = random_dir(rng);
        let pos = ctr + r * d;
        let vel = rng.gen_range(5.0..30.0) * d;
        let rad = rng.gen_range(1.0..5.0) + (16. - r).log2();
        let expand_rate = rng.gen_range(1.0..30.0);
        let fade_rate = r / 30.0 + 1.3;
        let life_time = 1.0;
        let puff = Puff {
            pos,
            vel,
            radius: rad,
            expand_rate,
            color,
            fade_rate,
            life_time,
        };
        out(puff);
    }
}
