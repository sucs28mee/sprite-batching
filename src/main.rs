mod math;
mod sprite_batch;

use std::{collections::LinkedList, io::Cursor};
use glium::{backend::glutin::SimpleWindowBuilder, program, Surface, glutin::{surface::WindowSurface}, Display, implement_vertex, VertexBuffer, IndexBuffer, texture::{Texture2dArray, Texture1dArray, CompressedTexture2d, RawImage2d, CompressedTexture2dArray}, index::{PrimitiveType, self, Index}, vertex, uniform, DrawParameters, Rect, program::{ProgramChooserCreationError}, Profile, Program, Texture2d};
use image::{RgbaImage, DynamicImage, GenericImageView};
use math::{Matrix4x4, Vector2};
use sprite_batch::{SpriteBatch, DrawData};
use winit::{event_loop::EventLoopBuilder, event::{WindowEvent, Event}};



fn main() {
    let event_loop = EventLoopBuilder::new().build();
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
    let program = program!(
        &display,
        140 => {
            vertex: include_str!("shaders/default.vert"),
            fragment: include_str!("shaders/default.frag")
        }
    ).unwrap();

    let mut image = image::RgbaImage::new(500, 500);
    for (i, _, pixel) in image.enumerate_pixels_mut() {
        *pixel = image::Rgba([(255f32 * i as f32 / 500f32) as u8, 0, 0, 255]);
    }

    let mut particles = (0..1)
        .map(|index| (Vector2::ZERO, Vector2::UNIT_Y.rotated_by(Vector2::ZERO, index as f32) * 20f32))
        .collect::<Vec<_>>();
    
    event_loop.run(
        move |event, _, control_flow| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    _ => ()
                }
            }

            let mut sprite_batch = SpriteBatch::new(DrawParameters::default());
            for (mut position, mut velocity) in particles.iter_mut() {
                position.x += 2.5f32;
                velocity *= 0.95f32;

                sprite_batch.draw(
                    DrawData {
                        position,
                        rotation: 0f32,
                        depth: 0f32,
                        image: &image,
                        scale: Vector2::ONE
                    }
                );
            }

            sprite_batch.flush(&window, &display, &program).unwrap();
        }
    );
}
