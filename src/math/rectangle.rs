use super::Vector2;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rectangle {
    pub position: Vector2,
    pub width: f32,
    pub height: f32
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { position: Vector2::new(x, y), width, height }
    }

    pub fn new_center(center: Vector2, width: f32, height: f32) -> Self {
        Self { position: center - Vector2::new(width, height) * 0.5, width, height }
    }

    pub fn size(&self) -> Vector2 {
        Vector2::new(self.width, self.height)
    }

    pub fn center(&self) -> Vector2 {
        self.position + Vector2::new(self.width, self.height) * 0.5
    }

    pub fn top(&self) -> f32 {
        self.position.y + self.height
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.width
    }

    pub fn intersects(&self, rhs: &Rectangle) -> bool {
        rhs.position.x < self.right() && self.position.x < rhs.right() 
            && rhs.position.y < self.top() && self.position.y < rhs.top()
    }
}