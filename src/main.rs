mod math;
mod sprite_batch;
mod application;
mod sprite;
mod color;

use std::{collections::HashMap, default::Default};
use application::ApplicationContext;
use color::Color;
use glium::{Blend, uniforms::MagnifySamplerFilter};
use image::ImageFormat;
use math::{Rectangle, Vector2};
use rand::{Rng, rngs::ThreadRng};
use sprite::{Sprite, SpriteLoader};
use sprite_batch::{SpriteBatch, DrawData};

struct Particle {
    position: Vector2,
    velocity: Vector2,
    rotation: f32,
    time_left: f32
}

struct Application { 
    sprites: HashMap<String, Sprite>,
    particles: Vec<Particle>,
    time: f32,
    next_spawn_time: f32,
    random: ThreadRng,
    background_color: Color
}

impl ApplicationContext for Application {
    fn new() -> Self {
        Self { sprites: HashMap::new(), particles: Vec::new(), time: 0f32, next_spawn_time: 0f32, random: rand::thread_rng(), background_color: Color::GREEN }
    }

    fn load(&mut self, sprite_loader: &mut SpriteLoader) {
        self.sprites.insert(
            "Slime".to_owned(), 
            sprite_loader.load_sprite(
                image::load_from_memory_with_format(
                    include_bytes!("../assets/Slime.png"), 
                    ImageFormat::Png
                ).unwrap().into_rgba8()
            )
        );
    }

    fn update(&mut self, delta_time: f32) {
        println!("FPS: {}", 1f32 / delta_time);
        if self.time > self.next_spawn_time {
            self.next_spawn_time = self.time + 0.0001f32;
            self.particles.push(
                Particle { 
                    position: Vector2::ZERO, 
                    velocity: Vector2::new(self.random.gen_range(-500f32..500f32), self.random.gen_range(500f32..1000f32)), 
                    rotation:  self.random.gen_range(-1f32..1f32),
                    time_left: 20f32
                }
            )
        }

        self.particles.retain_mut(
            |particle| {
                particle.position += particle.velocity * delta_time;
                particle.velocity.y -= 250f32 * delta_time;
                particle.rotation += particle.velocity.y * delta_time * 0.001f32;

                particle.time_left -= delta_time;
                particle.time_left > 0f32 
            }
        );

        self.time += delta_time;
    }

    fn draw(&self, sprite_batch: &mut SpriteBatch) { 
        sprite_batch.clear_color(self.background_color);

        sprite_batch.sampler_behaviour.magnify_filter = MagnifySamplerFilter::Nearest;
        sprite_batch.draw_parameters.blend = Blend::alpha_blending();
        for particle in self.particles.iter() {
            sprite_batch.draw(
                DrawData {
                    position: particle.position,
                    sprite: self.sprites["Slime"],
                    source: Some(Rectangle::new(0f32, ((self.time * 4f32) as u32 % 3) as f32 * 16f32, 16f32, 16f32)),
                    scale: Vector2::ONE * 10f32,
                    origin: Vector2::ONE * 8f32,
                    rotation: particle.rotation,
                    ..Default::default()
                }
            );
        }
    }
}

fn main() {
    application::run::<Application>();
}
