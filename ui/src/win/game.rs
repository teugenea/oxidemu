use crate::GuiCtx;
use common::emulator::EmulMgr;
use crate::ui_error::*;
use common::message::{ ErrorMsg, Msg };
use glium::texture::{ClientFormat, RawImage2d};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerBehavior};
use glium::Texture2d;
use imgui::Ui;
use imgui::Window;
use imgui::{Condition, Image, TextureId, WindowFlags};
use imgui_glium_renderer::Texture;

use std::borrow::Cow;
use std::rc::Rc;

use crate::render::SdlRender;

pub struct GameWindow<'a> {
    texture_id: Option<TextureId>,
    sdl_render: Option<SdlRender<'a>>,
    current_version: u32,
    current_scale: u32,
}

impl<'a> GameWindow<'a> {
    pub fn new() -> Self {
        Self {
            texture_id: None,
            sdl_render: None,
            current_version: 0,
            current_scale: 0,
        }
    }

    pub fn show_window(
        &mut self,
        emul: &EmulMgr,
        ui: &Ui,
        gui_ctx: &mut GuiCtx,
    ) -> Result<(), Box<dyn Msg>> {
        ui.window("Game")
            .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_RESIZE)
            .position(gui_ctx.work_pos(), Condition::Always)
            .size(gui_ctx.work_size(), Condition::Always)
            .build(|| {
                if self.should_update_render(emul) {
                    self.create_render(emul, gui_ctx.state().render_scale);
                }
                let render = match &self.sdl_render {
                    Some(r) => r,
                    None => {
                        let err = ErrorMsg::new(UiErrorTopicId::SdlRender.into(), UiErrorMsgId::NotInitialized.into());
                        let result: Result<(), Box<dyn Msg>> = Err(Box::new(err));
                        return result;
                    }
                };
                let width = render.scaled_size()[0];
                let height = render.scaled_size()[1];
                let pixels = emul.video_buffer()?;
                self.convert_buffer(gui_ctx, pixels)?;
                if let Some(texture_id) = self.texture_id {
                    Image::new(texture_id, [width as f32, height as f32]).build(ui);
                }
                Ok(())
            })
            .unwrap()
    }

    fn convert_buffer(&mut self, gui_ctx: &mut GuiCtx, buff: Vec<u8>) -> Result<(), Box<dyn Msg>> {
        let render = match self.sdl_render.as_mut() {
            Some(r) => r,
            None => {
                let err = ErrorMsg::new(UiErrorTopicId::SdlRender.into(), UiErrorMsgId::NotInitialized.into());
                return Err(Box::new(err));
            }
        };
        let width = render.scaled_size()[0];
        let height = render.scaled_size()[1];
        let pixels = render.get_pixels(buff);
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
            let gl_texture = Texture2d::new(gui_ctx.facade(), raw);
            match gl_texture {
                Err(e) => {
                    let err = ErrorMsg::new(UiErrorTopicId::SdlRender.into(), UiErrorMsgId::NotInitialized.into())
                        .set_source(Box::new(e));
                    return Err(Box::new(err));
                }
                Ok(r) => {
                    self.create_texture(gui_ctx, r);
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    fn create_texture(&mut self, gui_ctx: &mut GuiCtx, gl_texture: Texture2d) {
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

    fn should_update_render(&mut self, emul: &EmulMgr) -> bool {
        match self.sdl_render.as_ref() {
            Some(render) => {
                self.current_version != emul.version() || *render.scale() != self.current_scale
            }
            _ => true,
        }
    }

    fn create_render(&mut self, emul: &EmulMgr, scale: u32) {
        if let Some(render) = self.sdl_render.take() {
            drop(render);
        }
        if let Ok(resolution) = emul.resolution() {
            self.sdl_render = Some(SdlRender::new(resolution, scale));
            self.current_version = emul.version();
            self.current_scale = scale;
        }
    }
}
