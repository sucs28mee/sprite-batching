use glium::{
    DrawParameters, 
    Program, 
    implement_vertex, 
    Display, 
    glutin::surface::WindowSurface,
    VertexBuffer, 
    IndexBuffer, 
    index::PrimitiveType, 
    texture::{RawImage2d, Texture2dArray}, 
    Surface, 
    uniform
};

use image::RgbaImage;
use winit::window::Window;
use crate::math::{Matrix4x4, Vector2};

pub struct DrawData<'a> {
    pub position: Vector2,
    pub rotation: f32,
    pub origin: Vector2,
    pub depth: f32,
    pub image: &'a RgbaImage,
    pub scale: Vector2
}

#[derive(Clone, Copy, Default)]
struct Vertex {
    index: u32,
    position: [f32; 3],
    matrix: [[f32; 4]; 4]
}

implement_vertex!(Vertex, index, position, matrix);
pub struct SpriteBatch<'a> {
    pub draw_parameters: DrawParameters<'a>,
    window: &'a Window,
    display: &'a Display<WindowSurface>,
    program: &'a Program,
    draw_data_cache: Vec<DrawData<'a>>
}

impl <'a> SpriteBatch<'a> {
    pub fn new(
        draw_parameters: DrawParameters<'a>, 
        window: &'a Window, 
        display: &'a Display<WindowSurface>, 
        program: &'a Program
    ) -> Self {
        Self { draw_parameters, window, display, program, draw_data_cache: Vec::new() }
    }

    pub fn draw(&mut self, draw_data: DrawData<'a>) {
        self.draw_data_cache.push(draw_data);
    }

    pub fn flush(self) -> Result<(), ()> {
        if self.draw_data_cache.is_empty() {
            return Ok(());
        }

        let mut vertices = vec![Vertex::default(); self.draw_data_cache.len() * 4].into_boxed_slice();
        let mut indices = vec![0u32; self.draw_data_cache.len() * 6].into_boxed_slice();
        let mut textures = Vec::with_capacity(self.draw_data_cache.len());

        let (screen_width, screen_height) = {
            let screen_size = self.window.inner_size();
            (screen_size.width as f32, screen_size.height as f32)
        };

        for i in 0..self.draw_data_cache.len() {
            let draw_data = &self.draw_data_cache[i];

            let image_dimensions = draw_data.image.dimensions();
            let raw_image = RawImage2d::from_raw_rgba(
                draw_data.image.clone().into_raw(), 
                image_dimensions
            );
            textures.push(raw_image);

            let matrix = (
                Matrix4x4::new_translation(draw_data.position.x / screen_width, draw_data.position.y / screen_height, 0f32)
                * Matrix4x4::new_scaling(1f32 / screen_width, 1f32 / screen_height, 1f32)
                * Matrix4x4::new_rotation(draw_data.rotation)
                * Matrix4x4::new_scaling(draw_data.scale.x, draw_data.scale.y, 1f32)
                * Matrix4x4::new_translation(-draw_data.origin.x, -draw_data.origin.y, 0f32)
                * Matrix4x4::new_scaling(image_dimensions.0 as f32, image_dimensions.1 as f32, 1f32)
            ).to_array();

            vertices[i * 4] = Vertex {
                position: [0f32, 0f32, draw_data.depth],
                index: i as u32,
                matrix
            };
            vertices[i * 4 + 1] = Vertex {
                position: [1f32, 0f32, draw_data.depth],
                index: i as u32,
                matrix
            };
            vertices[i * 4 + 2] = Vertex {
                position: [1f32, 1f32, draw_data.depth],
                index: i as u32,
                matrix
            };
            vertices[i * 4 + 3] = Vertex {
                position: [0f32, 1f32, draw_data.depth],
                index: i as u32,
                matrix
            };

            indices[i * 6] = i as u32 * 4;
            indices[i * 6 + 1] = i as u32 * 4 + 1;
            indices[i * 6 + 2] = i as u32 * 4 + 2;
            indices[i * 6 + 3] = i as u32 * 4 + 2;
            indices[i * 6 + 4] = i as u32 * 4 + 3;
            indices[i * 6 + 5] = i as u32 * 4;
        }

        let Ok(vertex_buffer) = VertexBuffer::new(self.display, &vertices) else {
            return Err(());
        };

        
        let Ok(index_buffer) = IndexBuffer::new(self.display, PrimitiveType::TrianglesList, &indices) else {
            return Err(());
        };
        
        let Ok(textures) = Texture2dArray::new(self.display, textures) else {
            return Err(());
        };
        
        let mut frame = self.display.draw();
        frame.clear_color(0f32, 0f32, 0f32, 0f32);
        frame.draw(
            &vertex_buffer,
            &index_buffer,
            self.program,
            &uniform! {
                textures: textures
            },
            &self.draw_parameters
        ).map_err(|_| ())?;
        
        frame.finish().map_err(|_| ())
    }

}