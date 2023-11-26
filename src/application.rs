use std::{time::{Instant, Duration}, rc::Rc};
use glium::{backend::glutin::SimpleWindowBuilder, program};
use image::RgbaImage;
use winit::{event_loop::{EventLoopBuilder, ControlFlow}, event::{WindowEvent, Event, StartCause::ResumeTimeReached}};

use crate::{sprite_batch::SpriteBatch, sprite::SpriteLoader};

pub trait ApplicationContext {
    fn new() -> Self;

    fn load(&mut self, sprite_loader: &mut SpriteLoader) { }
    fn update(&mut self, delta_time: f32) { }
    fn draw(&self, sprite_batch: &mut SpriteBatch) { }
}

pub fn run<T>() where T: ApplicationContext + 'static {
    let event_loop = EventLoopBuilder::new().build();
    let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
    let program = program!(
        &display,
        140 => {
            vertex: include_str!("shaders/default.vert"),
            fragment: include_str!("shaders/default.frag")
        }
    ).unwrap();

    let mut context = T::new();

    let mut sprite_loader = SpriteLoader::new();
    sprite_loader.load_sprite(RgbaImage::from_raw(1u32, 1u32, vec![255, 255, 255, 255]).unwrap());
    context.load(&mut sprite_loader);
    let texture_array = sprite_loader.create_texture_array(&display).unwrap();


    let mut sprite_batch = SpriteBatch::new(
        Rc::new(window),
        Rc::new(display),
        program,
        texture_array
    );

    let mut last_frame_instant = Instant::now();
    event_loop.run(
        move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        },
                        _ => ()
                    }
                },
                Event::NewEvents(ResumeTimeReached { .. }) => {
                    let delta_time = last_frame_instant.elapsed().as_secs_f32();
                    last_frame_instant = Instant::now();
                    context.update(delta_time);
                    context.draw(&mut sprite_batch);
                    sprite_batch.flush().unwrap();
                }
                _ => ()
            }

            if *control_flow != ControlFlow::Exit {
                *control_flow = ControlFlow::WaitUntil(last_frame_instant + Duration::from_millis(1));
            }
        }
    );
}