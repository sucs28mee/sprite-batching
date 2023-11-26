use defaults::Defaults;
use glium::{texture::{Texture2dArray, RawImage2d, TextureCreationError}, glutin::surface::WindowSurface, Display};
use image::{RgbaImage, ImageBuffer, Rgba};

#[derive(Clone, Copy, Debug, Defaults)]
pub struct Sprite {
    index: u32,
    #[def = "(1u32, 1u32)"]
    dimensions: (u32, u32)
}

impl Sprite {
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
}

pub struct SpriteLoader {
    images: Vec<RgbaImage>,
}

impl SpriteLoader {
    pub fn new() -> Self {
        Self { images: Vec::new() }
    }

    pub fn load_sprite(&mut self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Sprite {
        let dimensions = image.dimensions();
        self.images.push(image);
        Sprite { index: (self.images.len() - 1) as u32, dimensions }
    }

    pub fn create_texture_array(mut self, display: &Display<WindowSurface>) -> Result<Texture2dArray, TextureCreationError> {
        let mut max_width = 0;
        let mut max_height = 0;
        for image in self.images.iter_mut() {
            if image.width() > max_width {
                max_width = image.width();
            }

            if image.height() > max_height {
                max_height = image.height();
            }
        }

        self.images = self.images.iter().map(
            |image| {
                let mut new_image = RgbaImage::new(max_width, max_height);
                for (i, j, pixel) in new_image.enumerate_pixels_mut() {
                    if i < image.width() && j < image.height() {
                        *pixel = *image.get_pixel(i, j);
                    } else {
                        *pixel = Rgba([0, 0, 0, 0]);
                    }
                }

                new_image
            }
        ).collect();

        Texture2dArray::new(
            display, 
            self.images.iter().map(
                |image| {
                    RawImage2d::from_raw_rgba_reversed(image.as_raw(), image.dimensions())
                }
            ).collect()
        )
    }
}