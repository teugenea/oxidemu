use gilrs::{Gilrs};

use eframe::{egui, egui::Frame};
use egui::ColorImage;

use crate::render::SdlRender;
use common::cpu::Cpu;
use chip8::chip8::Chip8;
use common::video::VideoOut;

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct OxidemuApp<'a> {
    em: Chip8,
    sdl_render: SdlRender<'a>,
    gilrs: Gilrs,
}

impl<'a> OxidemuApp<'a> {
    
}

impl<'a> Default for OxidemuApp<'a> {
    fn default() -> Self {
        let mut chip = Chip8::new();
        chip.load_rom(String::from(
            "D:\\Projects\\rusty-emul\\chip8-roms\\demos\\Stars [Sergey Naydenov, 2010].ch8",
        ));

        Self {
            em: chip,
            sdl_render: SdlRender::new([64, 32], 10),
            gilrs: Gilrs::new().unwrap(),
        }
    }
}

impl<'a> eframe::App for OxidemuApp<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });
        let central_frame = Frame {
            inner_margin: egui::style::Margin::same(0.0),
            ..Default::default()
        };
        egui::CentralPanel::default()
            .frame(central_frame)
            .show(ctx, |ui| {
                let cnv = egui::Frame::dark_canvas(ui.style())
                    .rounding(eframe::egui::Rounding::none())
                    .inner_margin(egui::style::Margin::same(0.0));
                cnv.show(ui, |ui| {
                    self.em.cycle();
                    let t = self.em.get_video_buf_8();
                    let img = ColorImage::from_rgba_unmultiplied(
                        [
                            self.sdl_render.scaled_size[0] as usize,
                            self.sdl_render.scaled_size[1] as usize,
                        ],
                        &self.sdl_render.get_pixels(t),
                    );

                    let mut t = Option::<egui::TextureHandle>::None;
                    let tt: &egui::TextureHandle =
                        t.get_or_insert_with(|| ui.ctx().load_texture("render_image", img));

                    ui.image(tt, tt.size_vec2());
                    ui.ctx().request_repaint();

                    //self.handle_input(ui);
                    
                    if let Some(event) = self.gilrs.next_event() {
                        println!("{:?}", event);
                    }
                });
            });
    }
}
