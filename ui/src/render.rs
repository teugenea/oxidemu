use std::mem;

use sdl2::surface::{Surface};
use sdl2::render::{Canvas, Texture};
use sdl2::pixels::{PixelFormatEnum};

pub struct SdlRender<'a> {
    canvas: Canvas<Surface<'a>>,
    texture: Texture,
    pub size: [u32; 2],
    pub scaled_size: [u32; 2]
}

impl<'a> SdlRender<'a> {
    pub fn new(size: [u32; 2], scale: u32) -> Self {
        let scaled_size = [size[0] * scale, size[1] * scale];
        let surface = Surface::new(scaled_size[0], scaled_size[1], 
            PixelFormatEnum::RGBA8888).expect("Cannot create SDL2 surface");
        let canvas = Canvas::from_surface(surface).expect("Cannot create SDL2 canvas");
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, 
            size[0], size[1]).expect("Cannot create SDL2 texture");
        Self {
            canvas: canvas,
            texture: texture,
            size: size,
            scaled_size: scaled_size,
        }
    }

    pub fn get_pixels(&mut self, pixels: Vec<u8>) -> Vec<u8> {      
        let update_result = self.texture.update(None, &pixels, 
            mem::size_of::<u32>() * self.size[0] as usize);
        match update_result {
            Err(e) => panic!("Cannot update SDL2 texture: {}", e),
            _ => {}
        }
        self.canvas.clear();
        let copy_result = self.canvas.copy(&self.texture, None, None);
        match copy_result {
            Err(e) => panic!("Cannot copy SDL2 texture: {}", e),
            _ => {}
        }
        self.canvas.present();
        self.canvas.read_pixels(None, PixelFormatEnum::RGBA8888).expect("Cannot read pixels")
    }

}