use crate::win::main::MainWindow;
use common::emulator::EmulMgr;
use common::input::*;
use gilrs::{Button, Gilrs};
use glium::backend::Facade;
use glium::glutin;
use glium::glutin::event::{ElementState, Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use imgui::sys::igGetMainViewport;
use json_gettext::JSONGetText;

use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, TextureId, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

use std::time::{ Instant, Duration };
#[macro_use]
extern crate json_gettext;

mod gui_ctx;
mod render;
mod ui_error;
mod win;

use gui_ctx::*;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
    pub texture_id: Option<TextureId>,
    pub emul: EmulMgr,
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &EmulMgr, &Ui, &mut GuiCtx) + 'static>(
        self,
        mut run_ui: F,
    ) {
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            mut emul,
            ..
        } = self;

        let mut last_frame = Instant::now();
        let mut start = std::time::Instant::now();
        let mut frames = 0;
        let mut state = UiState::default();
        let loc = init_local();
        let mut gilrs = Gilrs::new().unwrap();
        let cycles_in_ms = 1.0 / emul.cycles_in_sec().unwrap() as f64 * Duration::from_secs(1).as_millis() as f64;

        event_loop.run(move |event, _, control_flow| {

            if let Some(gilrs::Event { id, event, time }) = gilrs.next_event() {
                println!("{:?}: {:?}", id, event);
            }
            
            match event {
                Event::NewEvents(_) => {
                    frames += 1;
                    if start.elapsed().as_secs() >= 1 {
                        println!(
                            "FPS: {:.0}",
                            frames as f64 / start.elapsed().as_millis() as f64 * 1000.0
                        );
                        frames = 0;
                        start = std::time::Instant::now();
                    }
                    let now = Instant::now();
                    let delta_time = now - last_frame;
                    imgui.io_mut().update_delta_time(delta_time);
                    last_frame = now;

                    let cycle_count = (delta_time.as_millis() as f64 / cycles_in_ms).round() as u128;
                    let mut cnt = 0u128;
                    while cnt <= cycle_count {
                        match emul.cycle() {
                            Ok(res) => cnt += res.last_cycle_count,
                            Err(_) => {}
                        }
                    }
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
                    let (work_size, work_pos) = System::get_viewport_size();
                    let mut gui = GuiCtx::new(
                        renderer.textures(),
                        display.get_context(),
                        &loc,
                        &mut state,
                        work_size,
                        work_pos,
                    );

                    run_ui(&mut run, &emul, &ui, &mut gui);
                    if !run {
                        *control_flow = ControlFlow::Exit;
                    }

                    let gl_window = display.gl_window();
                    let mut target = display.draw();
                    target.clear_color_srgb(0.177, 0.177, 0.177, 1.0);
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
                    event:
                        WindowEvent::KeyboardInput {
                            device_id: _,
                            input,
                            is_synthetic: _,
                        },
                    ..
                } => {
                    
                }
                event => {
                    let gl_window = display.gl_window();
                    platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
                }
            }
        })
    }

    fn get_viewport_size() -> ([f32; 2], [f32; 2]) {
        unsafe {
            let vp = igGetMainViewport();
            let cont_size = (*vp).WorkSize;
            let cont_pos = (*vp).WorkPos;
            ([cont_size.x, cont_size.y], [cont_pos.x, cont_pos.y])
        }
    }
}

fn init_local<'a>() -> JSONGetText<'a> {
    static_json_gettext_build!(
        "en_US";
        "en_US" => "langs/en_US.json",
        "ru_RU" => "langs/ru_RU.json",
    )
    .unwrap()
}

fn init(title: &str, em: EmulMgr) -> System {
    let event_loop = EventLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
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
        emul: em,
    }
}

pub fn show(em: EmulMgr) {
    let system = init("Oxidemu", em);
    let mut main_window = MainWindow::new();
    system.main_loop(move |_, em, ui, gui_ctx| main_window.show(em, ui, gui_ctx));
}
