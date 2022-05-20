use std::mem;

use sdl2::surface::{Surface};
use sdl2::render::{Canvas, Texture};
use sdl2::pixels::{PixelFormatEnum};

pub struct SdlRender<'a> {
    canvas: Canvas<Surface<'a>>,
    texture: Option<Texture>,
    size: [u32; 2],
    scaled_size: [u32; 2],
    scale: u32,
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
            texture: Some(texture),
            size: size,
            scale: scale,
            scaled_size: scaled_size,
        }
    }

    pub fn get_pixels(&mut self, pixels: Vec<u8>) -> Vec<u8> {
        if self.texture.is_none() {
            panic!("Texture is none");
        }
        let texture = self.texture.as_mut().ok_or("Cannot get texture").unwrap();
        let update_result = texture.update(None, &pixels, 
            mem::size_of::<u32>() * self.size[0] as usize);
        match update_result {
            Err(e) => panic!("Cannot update SDL2 texture: {}", e),
            _ => {}
        }
        self.canvas.clear();
        let copy_result = self.canvas.copy(&texture, None, None);
        match copy_result {
            Err(e) => panic!("Cannot copy SDL2 texture: {}", e),
            _ => {}
        }
        self.canvas.present();
        self.canvas.read_pixels(None, PixelFormatEnum::RGBA8888).expect("Cannot read pixels")
    }

    pub fn size(&self) -> &[u32; 2] { &self.size }

    pub fn scale(&self) -> &u32 { &self.scale }

    pub fn scaled_size(&self) -> &[u32; 2] { &self.scaled_size }

}

impl<'a> Drop for SdlRender<'a> {
    fn drop(&mut self) {
        let texture = self.texture.take();
        match texture {
            Some(t) => unsafe { t.destroy() },
            _ => {}
        }
    }
}