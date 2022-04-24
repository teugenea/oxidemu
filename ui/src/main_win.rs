use std::mem;
use eframe::{egui, epi, egui::Frame};
use egui::ColorImage;

use sdl2::surface::Surface;
use sdl2::render::{Canvas, TextureAccess};
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::rect::{Point, Rect};

use common::{ Emulator, video::VideoOut, cpu::Cpu };
use chip8::chip8::Chip8;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct OxidemuApp {
    texture: Option<egui::TextureHandle>,
    em: Chip8
}

impl OxidemuApp {
    fn render(&self, pixels: Vec<u8>) -> Vec<u8> {
        let surf = Surface::new(1024, 768, PixelFormatEnum::RGBA8888)
            .expect("Cannot create SDL2 surface");
        let mut canvas = Canvas::from_surface(surf).expect("Cannot create SDL2 caanvas");
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator.create_texture(PixelFormatEnum::RGBA8888, 
            TextureAccess::Streaming, 64, 32).expect("Cannot crate texture");
        
        texture.update(None, &pixels, mem::size_of::<u32>() * 64);
        canvas.clear();
        canvas.copy(&texture, None, None);
        canvas.present();
        let pixels = canvas.read_pixels(None, PixelFormatEnum::RGBA8888).expect("Cannot read pixels");
        pixels
    }
}

impl Default for OxidemuApp {
    fn default() -> Self {
        Self {
            texture: None,
            em: Chip8::new()
        }
    }
}

impl epi::App for OxidemuApp {
    
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
                let img = ColorImage::from_rgba_unmultiplied([1024, 768], 
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