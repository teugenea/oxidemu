use eframe::{egui, epi, egui::Frame};
use egui::ColorImage;

use sdl2::surface::Surface;
use sdl2::render::{Canvas, TextureAccess};
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::rect::{Point, Rect};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct OxidemuApp {
    texture: Option<egui::TextureHandle>,
}

impl Default for OxidemuApp {
    fn default() -> Self {
        OxidemuApp {
            texture: None,
        }
    }
}

impl epi::App for OxidemuApp {
    
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
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
                //ui.allocate_exact_size(egui::Vec2::new(1024.0, 768.0), egui::Sense::focusable_noninteractive());
                let img = ColorImage::from_rgba_unmultiplied([1024, 768], &create_sdl());
                //img.pixels = create_sdl();
                //create_sdl();
                let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                    // Load the texture only once.
                    ui.ctx().load_texture("my-image", img)
                });
                ui.image(texture, texture.size_vec2());
            });
        });
        
    }

    fn name(&self) -> &str { "Oxidemu" }
}

pub fn create_sdl() -> Vec<u8> {
    let surf = Surface::new(1024, 768, PixelFormatEnum::RGBA8888)
        .expect("Cannot create SDL2 surface");
    let mut canvas = Canvas::from_surface(surf).expect("Cannot create SDL2 caanvas");
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture(PixelFormatEnum::RGBA8888, 
        TextureAccess::Streaming, 1024, 768).expect("Cannot crate texture");
    
    //texture.update(None, &[u8; 10], 32*64);
    canvas.clear();
    canvas.set_draw_color(Color::RED);
    canvas.draw_line(Point::new(0, 0), Point::new(500, 500));
    canvas.present();
    //canvas.copy(&texture, None, None);
    let pixels = canvas.read_pixels(None, PixelFormatEnum::RGBA8888).expect("Cannot read pixels");
    pixels
}