use chip8::chip8::Chip8;
use common::cpu::Cpu;
use common::video::VideoOut;
use glium::backend::Facade;
use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::texture::ClientFormat;
use glium::texture::RawImage2d;
use glium::uniforms::SamplerBehavior;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{Display, Surface, Texture2d};
use imgui::*;
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_glium_renderer::Texture;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::borrow::Cow;
use std::error::Error;
use std::rc::Rc;
use std::time::Instant;

mod render;

struct CustomTexturesApp<'a> {
    my_texture_id: Option<TextureId>,
    sdl_render: render::SdlRender<'a>,
    em: chip8::chip8::Chip8,
}

impl<'a> CustomTexturesApp<'a> {
    fn new() -> Self {
        let mut em = chip8::chip8::Chip8::new();
        em.load_rom(String::from(
            "D:/Projects/rusty-emul/chip8-roms/games/Airplane.ch8",
        ));
        Self {
            my_texture_id: None,
            sdl_render: render::SdlRender::new([64, 32], 10),
            em,
        }
    }

    fn show_textures(&mut self, ui: &Ui, textures: &mut Textures<Texture>, facade: &dyn Facade) {
        Window::new("Hello textures")
            .size([400.0, 700.0], Condition::FirstUseEver)
            .build(ui, || {
                let width = self.sdl_render.scaled_size[0];
                let height = self.sdl_render.scaled_size[1];
                self.update_texture(textures, facade)
                    .expect("Cannot update texture");
                if let Some(my_texture_id) = self.my_texture_id {
                    Image::new(my_texture_id, [width as f32, height as f32]).build(ui);
                }
            });
    }

    fn update_texture(
        &mut self,
        textures: &mut Textures<Texture>,
        gl_ctx: &dyn Facade,
    ) -> Result<(), Box<dyn Error>> {
        let r = self.em.cycle();
        let width = self.sdl_render.scaled_size[0];
        let height = self.sdl_render.scaled_size[1];

        let pixels = self.sdl_render.get_pixels(self.em.get_video_buf_8());
        let raw = RawImage2d {
            data: Cow::Owned(pixels),
            width: width as u32,
            height: height as u32,
            format: ClientFormat::U8U8U8U8,
        };
        if let Some(tex) = self.my_texture_id {
            //textures.replace(tex, texture);
            if let Some(tt) = textures.get(tex) {
                let rc = glium::Rect {
                    left: 0,
                    bottom: 0,
                    width,
                    height,
                };
                tt.texture.write(rc, raw);
            }
        } else {
            let gl_texture = Texture2d::new(gl_ctx, raw)?;
            let texture = Texture {
                texture: Rc::new(gl_texture),
                sampler: SamplerBehavior {
                    magnify_filter: MagnifySamplerFilter::Linear,
                    minify_filter: MinifySamplerFilter::Linear,
                    ..Default::default()
                },
            };
            let texture_id = textures.insert(texture);
            self.my_texture_id = Some(texture_id);
        }
        Ok(())
    }
}

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
    pub texture_id: Option<TextureId>,
}

impl System {
    pub fn main_loop<
        F: FnMut(&mut bool, &mut Ui, &mut Textures<Texture>, &dyn Facade) + 'static,
    >(
        self,
        mut run_ui: F,
    ) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();
        let mut start = std::time::Instant::now();
        let mut frames = 0;

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                frames += 1;
                if start.elapsed().as_secs() >= 1 {
                    println!("FPS: {:.0}", frames as f64 / start.elapsed().as_millis() as f64 * 1000.0);
                    frames = 0;
                    start = std::time::Instant::now();
                }
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
                
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let mut ui = imgui.frame();
                let mut run = true;

                run_ui(
                    &mut run,
                    &mut ui,
                    renderer.textures(),
                    display.get_context(),
                );
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
                gl_window.window().request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput{device_id, input, is_synthetic},
                ..
            } => {
                println!("{:?}", input);
            }
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}

pub fn init(title: &str) -> System {
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title(title.to_owned())
        .with_inner_size(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let display =
        Display::new(builder, context, &event_loop).expect("Failed to initialize display");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
    }

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        },
        FontSource::TtfData {
            data: include_bytes!("../../resources/mplus-1p-regular.ttf"),
            size_pixels: font_size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::japanese(),
                ..FontConfig::default()
            }),
        },
    ]);

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    System {
        event_loop,
        display,
        imgui,
        platform,
        renderer,
        font_size,
        texture_id: None,
    }
}

pub fn show() {
    let mut system = init("ttt");
    let mut my_app = CustomTexturesApp::new();
    // my_app
    //     .register_textures(system.display.get_context(), system.renderer.textures())
    //     .expect("Failed to register textures");
    system.main_loop(move |_, ui, tex, facade| my_app.show_textures(ui, tex, facade));
}
