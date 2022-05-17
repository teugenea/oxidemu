use imgui::Ui;
use common::emulator::Emul;
use imgui::Window;
use crate::GuiCtx;
use glium::texture::{ClientFormat, RawImage2d};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerBehavior};
use glium::Texture2d;
use imgui::{Condition, Image, TextureId, WindowFlags};
use imgui_glium_renderer::Texture;

use std::borrow::Cow;
use std::error::Error;
use std::rc::Rc;

use crate::render::SdlRender;

pub struct GameWindow<'a> {
    texture_id: Option<TextureId>,
    sdl_render: SdlRender<'a>,
}

impl<'a> GameWindow<'a> {
    pub fn new() -> Self {
        Self {
            texture_id: None,
            sdl_render: SdlRender::new([64, 32], 10),
        }
    }

    pub fn show_window(
        &mut self,
        emul: Emul,
        ui: &Ui,
        gui_ctx: &mut GuiCtx,
    ) {
        Window::new("Game")
            .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_RESIZE)
            .position(gui_ctx.work_pos(), Condition::Always)
            .size(gui_ctx.work_size(), Condition::Always)
            .build(ui, || {
                let width = self.sdl_render.scaled_size[0];
                let height = self.sdl_render.scaled_size[1];
                self.update_texture(emul, gui_ctx).expect("Cannot update texture");
                if let Some(texture_id) = self.texture_id {
                    Image::new(texture_id, [width as f32, height as f32]).build(ui);
                }
            });
    }

    fn update_texture(
        &mut self,
        emul: Emul,
        gui_ctx: &mut GuiCtx
    ) -> Result<(), Box<dyn Error>> {
        let width = self.sdl_render.scaled_size[0];
        let height = self.sdl_render.scaled_size[1];

        let pixels = self.sdl_render.get_pixels(emul.lock().unwrap().video_buffer());
        let raw = RawImage2d {
            data: Cow::Owned(pixels),
            width: width as u32,
            height: height as u32,
            format: ClientFormat::U8U8U8U8,
        };
        if let Some(tex) = self.texture_id {
            if let Some(tt) = gui_ctx.textures().get(tex) {
                let rc = glium::Rect {
                    left: 0,
                    bottom: 0,
                    width,
                    height,
                };
                tt.texture.write(rc, raw);
            }
        } else {
            let gl_texture = Texture2d::new(gui_ctx.facade(), raw)?;
            let texture = Texture {
                texture: Rc::new(gl_texture),
                sampler: SamplerBehavior {
                    magnify_filter: MagnifySamplerFilter::Linear,
                    minify_filter: MinifySamplerFilter::Linear,
                    ..Default::default()
                },
            };
            let texture_id = gui_ctx.textures().insert(texture);
            self.texture_id = Some(texture_id);
        }
        Ok(())
    }
    
}
