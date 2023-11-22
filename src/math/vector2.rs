use std::ops::{Mul, Add, MulAssign, AddAssign, Sub, SubAssign, DivAssign, Div};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0., y: 0. };
    pub const ONE: Vector2 = Vector2 { x: 1., y: 1. };
    pub const UNIT_X: Vector2 = Vector2 { x: 1., y: 0. };
    pub const UNIT_Y: Vector2 = Vector2 { x: 0., y: 1. };

    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn rotated_by(&self, origin: Vector2, rotation: f32) -> Vector2 {
        let self_normalized = self.clone() - origin;

        Vector2::new(
            self_normalized.x * rotation.cos() + self_normalized.y * -rotation.sin(),
            self_normalized.x * rotation.sin() + self_normalized.y * rotation.cos()
        ) + origin
    }

    pub fn distance_squared(&self, rhs: &Vector2) -> f32 {
        (self.x - rhs.x).powi(2) + (self.y - rhs.y).powi(2)
    }

    pub fn distance(&self, rhs: &Vector2) -> f32 {
        self.distance_squared(rhs).sqrt()
    }

    pub fn as_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Vector2) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Self;

    fn div(self, rhs: Vector2) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl <T: Into<f32> + Clone> Mul<T> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs.clone().into(), y: self.y * rhs.into()}
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}