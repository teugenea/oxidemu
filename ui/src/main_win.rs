use std::mem;
use eframe::{egui, epi, egui::Frame};
use egui::ColorImage;

use sdl2::surface::{Surface, SurfaceContext};
use sdl2::render::{Canvas, TextureAccess, TextureCreator, Texture};
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::rect::{Point, Rect};

use common::{ Emulator, video::VideoOut, cpu::Cpu };
use chip8::chip8::Chip8;

struct SdlRender<'a> {
    canvas: Canvas<Surface<'a>>,
    texture: Texture,
    size: [u32; 2],
    scaled_size: [u32; 2]
}

impl<'a> SdlRender<'a> {
    pub fn new(size: [u32; 2], scale: u32) -> Self {
        if scale > 10 {
            panic!("Scale is too big");
        }
        let scaled_size = [size[0] * scale, size[1] * scale];
        let surface = Surface::new(scaled_size[0], scaled_size[1], 
            PixelFormatEnum::RGBA8888).expect("Cannot create SDL2 surface");
        let canvas = Canvas::from_surface(surface).expect("Cannot create SDL2 canvas");
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture(PixelFormatEnum::RGBA8888, 
            TextureAccess::Streaming, size[0], size[1]).expect("Cannot create SDL2 texture");
        Self {
            canvas: canvas,
            texture: texture,
            size: size,
            scaled_size: scaled_size,
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct OxidemuApp<'a> {
    texture: Option<egui::TextureHandle>,
    em: Chip8,
    sdl_render: SdlRender<'a>,
}

impl<'a> OxidemuApp<'a> {
    fn render(&mut self, pixels: Vec<u8>) -> Vec<u8> {      
        let update_result = self.sdl_render.texture.update(None, &pixels, 
            mem::size_of::<u32>() * self.sdl_render.size[0] as usize);
        match update_result {
            Err(e) => panic!("Cannot update SDL2 texture: {}", e),
            _ => {}
        }
        self.sdl_render.canvas.clear();
        let copy_result = self.sdl_render.canvas.copy(&self.sdl_render.texture, None, None);
        match copy_result {
            Err(e) => panic!("Cannot copy SDL2 texture: {}", e),
            _ => {}
        }
        self.sdl_render.canvas.present();
        self.sdl_render.canvas.read_pixels(None, PixelFormatEnum::RGBA8888).expect("Cannot read pixels")
    }
}

impl<'a> Default for OxidemuApp<'a> {
    fn default() -> Self {
        Self {
            texture: None,
            em: Chip8::new(),
            sdl_render: SdlRender::new([64, 32], 10),
        }
    }
}

impl<'a> epi::App for OxidemuApp<'a> {
    
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });
        let central_frame = Frame{
            margin: egui::style::Margin::same(0.0),
            ..Default::default()
        };
        egui::CentralPanel::default().frame(central_frame).show(ctx, |ui| {
            let cnv = egui::Frame::dark_canvas(ui.style())
                .rounding(eframe::egui::Rounding::none())
                .margin(egui::style::Margin::same(0.0));
            cnv.show(ui, |ui| {
                let img = ColorImage::from_rgba_unmultiplied(
                    [self.sdl_render.scaled_size[0] as usize, self.sdl_render.scaled_size[1] as usize], 
                    &self.render(self.em.get_video_buf_8()));
                let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                    ui.ctx().load_texture("render_image", img)
                });
                ui.image(texture, texture.size_vec2());
            });
        });
        
    }

    fn name(&self) -> &str { "Oxidemu" }
}