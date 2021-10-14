#[derive(Clone, Copy, Debug)]
pub struct Timer(f32);

impl Timer {
    pub fn new(t: f32) -> Self {
        Self(t)
    }

    pub fn update(&mut self, time_delta: f32) -> bool {
        self.0 -= time_delta;
        self.has_elapsed()
    }

    pub fn set(&mut self, t: f32) {
        self.0 = t;
    }

    pub fn time_left(self) -> f32 {
        if 0.0 < self.0 {
            self.0
        } else {
            0.0
        }
    }

    pub fn has_elapsed(self) -> bool {
        self.0 <= 0.0
    }
}
