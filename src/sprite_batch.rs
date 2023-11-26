use std::rc::Rc;
use defaults::Defaults;
use glium::{
    DrawParameters, 
    Program, 
    implement_vertex, 
    Display, 
    glutin::surface::WindowSurface,
    VertexBuffer, 
    IndexBuffer, 
    index::PrimitiveType, 
    texture::Texture2dArray, 
    Surface, 
    uniform, uniforms::{Sampler, SamplerBehavior}
};
use winit::window::Window;

use crate::{math::{Matrix4x4, Vector2, Rectangle}, sprite::Sprite, color::Color};

#[derive(Defaults)]
pub struct DrawData {
    pub sprite: Sprite,
    pub position: Vector2,
    pub source: Option<Rectangle>,
    pub rotation: f32,
    pub origin: Vector2,
    #[def = "Color::new(1f32, 1f32, 1f32, 1f32)"]
    pub color: Color,
    pub depth: f32,
    #[def = "Vector2::ONE"]
    pub scale: Vector2
}

#[derive(Clone, Copy, Default)]
struct Vertex {
    index: u32,
    position: [f32; 3],
    uv: [f32; 2],
    color: [f32; 4],
    matrix: [[f32; 4]; 4]
}

implement_vertex!(Vertex, index, position, uv, color, matrix);
pub struct SpriteBatch<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub sampler_behaviour: SamplerBehavior,
    pub program: Program,
    window: Rc<Window>,
    display: Rc<Display<WindowSurface>>,
    draw_data_cache: Vec<DrawData>,
    texture_array: Texture2dArray
}

impl <'a> SpriteBatch<'a> {
    pub fn new(
        window: Rc<Window>,
        display: Rc<Display<WindowSurface>>,
        program: Program,
        texture_array: Texture2dArray
    ) -> Self {
        Self { 
            draw_parameters: DrawParameters::default(), 
            sampler_behaviour: SamplerBehavior::default(), 
            window,
            display,
            program,
            draw_data_cache: Vec::new(),
            texture_array
        }
    }

    pub fn clear_color(&mut self, color: Color) {
        let (window_width, window_height) = (self.window.inner_size().width as f32, self.window.inner_size().height as f32);
        self.draw(
            DrawData { 
                position: Vector2::new(-window_width / 2f32, -window_height / 2f32),
                color, 
                scale: Vector2::new(window_width, window_height),
                ..Default::default()
            }
        )
    }

    pub fn draw(&mut self, draw_data: DrawData) {
        self.draw_data_cache.push(draw_data);
    }

    pub fn flush(&mut self) -> Result<(), ()> {
        if self.draw_data_cache.is_empty() {
            return Ok(());
        }

        let mut vertices = vec![Vertex::default(); self.draw_data_cache.len() * 4].into_boxed_slice();
        let mut indices = vec![0u32; self.draw_data_cache.len() * 6].into_boxed_slice();

        let (screen_width, screen_height) = {
            let screen_size = self.window.inner_size();
            (screen_size.width as f32, screen_size.height as f32)
        };

        for i in 0..self.draw_data_cache.len() {
            let draw_data = &self.draw_data_cache[i];
            let index = draw_data.sprite.index();
            let max_sprite_size = Vector2::new(self.texture_array.dimensions().0 as f32, self.texture_array.dimensions().1 as f32);
            let sprite_size = Vector2::new(draw_data.sprite.dimensions().0 as f32, draw_data.sprite.dimensions().1 as f32);
            let source = draw_data.source.unwrap_or(Rectangle::new(0f32, 0f32, sprite_size.x, sprite_size.y));

            let matrix = (
                Matrix4x4::new_translation(draw_data.position.x / screen_width, draw_data.position.y / screen_height, 0f32)
                * Matrix4x4::new_scaling(1f32 / screen_width, 1f32 / screen_height, 1f32)
                * Matrix4x4::new_rotation(draw_data.rotation)
                * Matrix4x4::new_scaling(draw_data.scale.x, draw_data.scale.y, 1f32)
                * Matrix4x4::new_translation(-draw_data.origin.x, -draw_data.origin.y, 0f32)
                * Matrix4x4::new_scaling(source.width, source.height, 1f32)
            ).to_array();

            let texture_coordinates_min = source.position / max_sprite_size;
            let texture_coordinates_max = texture_coordinates_min + source.size() / max_sprite_size;
            let color = [draw_data.color.red, draw_data.color.blue, draw_data.color.green, draw_data.color.alpha];

            vertices[i * 4] = Vertex {
                position: [0f32, 0f32, draw_data.depth],
                uv: [texture_coordinates_min.x, texture_coordinates_min.y],
                color,
                index,
                matrix
            };
            vertices[i * 4 + 1] = Vertex {
                position: [1f32, 0f32, draw_data.depth],
                uv: [texture_coordinates_max.x, texture_coordinates_min.y],
                color,
                index,
                matrix
            };
            vertices[i * 4 + 2] = Vertex {
                position: [1f32, 1f32, draw_data.depth],
                uv: [texture_coordinates_max.x, texture_coordinates_max.y],
                color,
                index,
                matrix
            };
            vertices[i * 4 + 3] = Vertex {
                position: [0f32, 1f32, draw_data.depth],
                uv: [texture_coordinates_min.x, texture_coordinates_max.y],
                color,
                index,
                matrix
            };

            indices[i * 6] = i as u32 * 4;
            indices[i * 6 + 1] = i as u32 * 4 + 1;
            indices[i * 6 + 2] = i as u32 * 4 + 2;
            indices[i * 6 + 3] = i as u32 * 4 + 2;
            indices[i * 6 + 4] = i as u32 * 4 + 3;
            indices[i * 6 + 5] = i as u32 * 4;
        }

        self.draw_data_cache.clear();

        let Ok(vertex_buffer) = VertexBuffer::new(self.display.as_ref(), &vertices) else {
            return Err(());
        };
        
        let Ok(index_buffer) = IndexBuffer::new(self.display.as_ref(), PrimitiveType::TrianglesList, &indices) else {
            return Err(());
        };
        
        let mut frame = self.display.draw();
        frame.clear_all((0f32, 0f32, 0f32, 0f32), 0f32, 0i32);
        frame.draw(
            &vertex_buffer,
            &index_buffer,
            &self.program,
            &uniform! {
                textures: Sampler(&self.texture_array, self.sampler_behaviour)
            },
            &self.draw_parameters
        ).map_err(|_| ())?;
        
        frame.finish().map_err(|_| ())
    }

}