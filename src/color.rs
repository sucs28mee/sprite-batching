
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32
}

impl Color {
    pub const BLACK: Color = Color { red: 0f32, green: 0f32, blue: 0f32, alpha: 0f32 };
    pub const RED: Color = Color { red: 1f32, green: 0f32, blue: 0f32, alpha: 0f32 };
    pub const GREEN: Color = Color { red: 0f32, green: 1f32, blue: 0f32, alpha: 0f32 };
    pub const BLUE: Color = Color { red: 0f32, green: 0f32, blue: 1f32, alpha: 0f32 };
    
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { red, green, blue, alpha }
    }
}