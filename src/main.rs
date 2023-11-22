mod math;
mod sprite_batch;


use std::time::{Instant, Duration};

use glium::{backend::glutin::SimpleWindowBuilder, DrawParameters, program};
use math::Vector2;
use sprite_batch::{SpriteBatch, DrawData};
use winit::{event_loop::{EventLoopBuilder, ControlFlow}, event::{WindowEvent, Event, ElementState, DeviceEvent}};

struct Particle {
    position: Vector2,
    velocity: Vector2,
    scale: Vector2,
}

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

    let mut particles = (0..100)
        .map(
            |index| Particle { 
                position: Vector2::ZERO, 
                velocity: Vector2::UNIT_Y.rotated_by(Vector2::ZERO, index as f32 * 0.1) * 20f32,
                scale: Vector2::new((index as f32).sin() + 1f32, 1f32)
            }
        )
        .collect::<Vec<_>>();
    let target_fps = 60;
    let mut time = 0f32;
    event_loop.run(
        move |event, _, control_flow| {
            let start_time = Instant::now();
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                            return;
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
                        particle.position += particle.velocity;
                        particle.velocity *= 0.95f32;
        
                        sprite_batch.draw(
                            DrawData {
                                position: particle.position,
                                rotation: 0.5f32 + time * 0.01f32,
                                origin: Vector2::ONE * 50f32,
                                depth: 0f32,
                                image: &image,
                                scale: particle.scale * (time.sin() + 1f32)
                            }
                        );
                    }
        
                    sprite_batch.flush().unwrap();
                }
                _ => ()
            }
            
            if *control_flow != ControlFlow::Exit {
                window.request_redraw();
                let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u64;

                let wait_millis = match 1000 / target_fps >= elapsed_time {
                    true => 1000 / target_fps - elapsed_time,
                    false => 0
                };
                let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
                *control_flow = ControlFlow::WaitUntil(new_inst);
            }

            time += 0.02f32;
        }
    );
}
