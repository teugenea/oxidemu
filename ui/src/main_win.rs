use eframe::{egui, epi, egui::Frame};
use egui::ColorImage;

use crate::render::SdlRender;
use common::cpu::Cpu;

use common::{ video::VideoOut };
use chip8::chip8::Chip8;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct OxidemuApp<'a> {
    texture: Option<egui::TextureHandle>,
    em: Chip8,
    sdl_render: SdlRender<'a>,
}

impl<'a> OxidemuApp<'a> {

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
                self.em.cycle();
                let img = ColorImage::from_rgba_unmultiplied(
                    [self.sdl_render.scaled_size[0] as usize, self.sdl_render.scaled_size[1] as usize], 
                    &self.sdl_render.get_pixels(self.em.get_video_buf_8())
                );
                let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                    ui.ctx().load_texture("render_image", img)
                });
                ui.image(texture, texture.size_vec2());
            });
        });
        
    }

    fn name(&self) -> &str { "Oxidemu" }
}