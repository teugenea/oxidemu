use crate::sys::igGetMainViewport;
use chip8::chip8::Chip8;
use crate::win::render::RenderWindow;
use json_gettext::JSONGetText;
use common::emulator::Emulator;
use glium::backend::Facade;
use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;

use glium::{Display, Surface};
use imgui::*;
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_glium_renderer::Texture;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use std::time::Instant;
#[macro_use] extern crate json_gettext;

mod render;
mod win;

struct State {
    open_file: bool,
}

pub struct GuiCtx<'a> {
    ui: &'a Ui<'a>, 
    textures: &'a mut Textures<Texture>, 
    facade: &'a dyn Facade,
    work_size: [f32; 2],
    work_pos: [f32; 2],
}

pub struct AppCtx<'a> {
    emulator: Box<dyn Emulator>,
    get_text: JSONGetText<'a>,
}

impl<'a> AppCtx<'a> {
    pub fn new() -> Self {
        Self {
            emulator: Box::new(Chip8::new()),
            get_text: AppCtx::init_localization(),
        }
    }

    fn init_localization() -> JSONGetText<'a> {
        static_json_gettext_build!(
            "en_US";
            "en_US" => "langs/en_US.json",
            "ru_RU" => "langs/ru_RU.json",
        ).unwrap()
    }

    pub fn emulator(&self) -> &Box<dyn Emulator> {
        &self.emulator
    }

    pub fn get_text(&self) -> &JSONGetText<'a> {
        &self.get_text
    }
}

struct App<'a> {
    ctx: AppCtx<'a>,
    rn: RenderWindow<'a>,
    state: State,
}

impl<'a> App<'a> {
    fn new() -> Self {
        Self {
            rn: RenderWindow::new(),
            ctx: AppCtx::new(),
            state: State {
                open_file: false,
            }
        }
    }

    fn show(&mut self, gui_ctx: &mut GuiCtx) {
        self.rn.show_window(&self.ctx, gui_ctx);
        self.main_menu(gui_ctx.ui);
        if self.state.open_file {
            println!("Open file");
        }
    }

    fn main_menu(&mut self, ui: &Ui)  {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu("File") {
                MenuItem::new("Open").build_with_ref(ui, &mut self.state.open_file);
                ui.separator();
                MenuItem::new("Exit").build(ui);
                menu.end();
            }
            menu_bar.end();
        }
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
        F: FnMut(&mut bool, &mut GuiCtx) + 'static,
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
                let ui = imgui.frame();
                let mut run = true;
                let mut gui = GuiCtx {
                    ui: &ui,
                    textures: renderer.textures(),
                    facade: display.get_context(),
                    work_size: [0.0, 0.0],
                    work_pos: [0.0, 0.0],
                };

                unsafe {
                    let vp = igGetMainViewport();
                    let cont_size = (*vp).WorkSize;
                    let cont_pos = (*vp).WorkPos;
                    gui.work_size = [cont_size.x, cont_size.y];
                    gui.work_pos = [cont_pos.x, cont_pos.y];
                }


                run_ui(&mut run, &mut gui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                //println!("{:?}", gl_window.window().inner_size());
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
                glyph_ranges: FontGlyphRanges::cyrillic(),
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
    let system = init("Oxidemu");
    let mut my_app = App::new();
    system.main_loop(move |_, gui_ctx | my_app.show(gui_ctx));
}
