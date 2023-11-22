mod math;
mod sprite_batch;


use std::time::{Instant, Duration};

use glium::{backend::glutin::SimpleWindowBuilder, DrawParameters, program};
use math::Vector2;
use sprite_batch::{SpriteBatch, DrawData};
use winit::{event_loop::{EventLoopBuilder, ControlFlow}, event::{WindowEvent, Event, ElementState}};

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

    let mut image = image::RgbaImage::new(100, 100);
    for (i, _, pixel) in image.enumerate_pixels_mut() {
        let red = (255f32 * i as f32 / 100f32) as u8;
        *pixel = image::Rgba([red, 255 - red, 0, 255]);
    }

    let target_frame_rate = 60;
    let mut particles = (0..100)
        .map(|index| (Vector2::ZERO, Vector2::UNIT_Y.rotated_by(Vector2::ZERO, index as f32) * 20f32))
        .collect::<Vec<_>>();
    
    event_loop.run(
        move |event, _, control_flow| {
            let frame_start_time = Instant::now();
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::MouseInput { state: ElementState::Released, .. } => {
                            window.request_redraw();
                        },
                        _ => ()
                    }
                },
                Event::RedrawRequested(_) => {
                    let mut sprite_batch = SpriteBatch::new(
                        DrawParameters::default(),
                        &window, 
                        &display, 
                        &program
                    );
                    
                    for particle in particles.iter_mut() {
                        particle.0 += particle.1;
                        particle.1 *= 0.95f32;
        
                        sprite_batch.draw(
                            DrawData {
                                position: particle.0,
                                rotation: 0f32,
                                depth: 0f32,
                                image: &image,
                                scale: Vector2::ONE
                            }
                        );
                    }
        
                    sprite_batch.flush().unwrap();
                },
                _ => ()
            }

            window.request_redraw();
        }
    );
}
