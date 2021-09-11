use std::{ops::{Add, Mul, Sub}};

pub struct GamePos {
    pos: (f32, f32)
}

impl Add for GamePos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.pos.0 + rhs.pos.0, self.pos.1 + rhs.pos.1).into()
    }
}
impl Sub for GamePos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.pos.0 - rhs.pos.0, self.pos.1 - rhs.pos.1).into()
    }
}
impl Mul<f32> for GamePos {
    type Output = GamePos;

    fn mul(self, rhs: f32) -> Self::Output {
        (self.pos.0 * rhs, self.pos.1 * rhs).into()
    }
}
impl Into<(f32, f32)> for GamePos {
    fn into(self) -> (f32, f32) {
        self.pos
    }
}
impl From<(f32, f32)> for GamePos {
    fn from(pos: (f32, f32)) -> Self {
        GamePos {
            pos
        }
    }
}