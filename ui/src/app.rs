#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]

use eframe::egui::Frame;
use egui::ColorImage;
use gilrs::Gilrs;

use crate::render::SdlRender;
use common::cpu::Cpu;
use chip8::chip8::Chip8;
use common::video::VideoOut;
use common::input::*;

pub struct OxidemuApp<'a> {
    quit: bool,
    em: Chip8,
    sdl_render: SdlRender<'a>,
    gilrs: Gilrs,
}

impl<'a> Default for OxidemuApp<'a> {
    fn default() -> Self {
        let mut chip = Chip8::new();
        chip.load_rom(String::from(
            "D:\\Projects\\rusty-emul\\chip8-roms\\games\\Airplane.ch8",
        ));

        Self {
            em: chip,
            sdl_render: SdlRender::new([64, 32], 10),
            gilrs: Gilrs::new().unwrap(),
            quit: false,
        }
    }
}

impl<'a> OxidemuApp<'a> {
    fn add_menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        self.quit = true;
                    }
                });
            });
        });
    }

    fn add_video(&mut self, ctx: &egui::Context) {
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
                    
                    if let Some(event) = self.gilrs.next_event() {
                        println!("{:?}", event);
                    }
                });
            });
    }
}

pub fn show() {
    let clear_color = [0.0, 0.0, 0.0];

    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let (gl_window, gl) = create_display(&event_loop);
    let gl = std::rc::Rc::new(gl);

    let mut egui_glow = egui_glow::winit::EguiGlow::new(gl_window.window(), gl.clone());
    let mut window = OxidemuApp::default();

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {

            let needs_repaint = egui_glow.run(gl_window.window(), |egui_ctx| {
                egui_ctx.set_visuals(egui::Visuals::dark());
                window.add_menu(egui_ctx);
                window.add_video(egui_ctx);
            });

            *control_flow = if window.quit {
                glutin::event_loop::ControlFlow::Exit
            } else if needs_repaint {
                gl_window.window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            if let Some(gilrs::Event { id, event, time }) = window.gilrs.next_event() {
                println!("{:?} New event from {}: {:?}", time, id, event);
            }

            {
                unsafe {
                    use glow::HasContext as _;
                    gl.clear_color(clear_color[0], clear_color[1], clear_color[2], 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                // draw things behind egui here

                egui_glow.paint(gl_window.window());

                // draw things on top of egui here

                gl_window.swap_buffers().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                if let glutin::event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic: _,
                } = &event
                {
                    window.em.process_input(InputKey::new(InputDevice::Keyboard(0),
                        input.scancode,
                        input.state == winit::event::ElementState::Pressed));
                }

                if let glutin::event::WindowEvent::Resized(physical_size) = &event {
                    gl_window.resize(*physical_size);
                } else if let glutin::event::WindowEvent::ScaleFactorChanged {
                    new_inner_size,
                    ..
                } = &event
                {
                    gl_window.resize(**new_inner_size);
                }

                egui_glow.on_event(&event);

                gl_window.window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }
            glutin::event::Event::LoopDestroyed => {
                egui_glow.destroy();
            }

            _ => (),
        }
    });
}

fn create_display(
    event_loop: &glutin::event_loop::EventLoop<()>,
) -> (
    glutin::WindowedContext<glutin::PossiblyCurrent>,
    glow::Context,
) {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title("egui_glow example");

    let gl_window = unsafe {
        glutin::ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(true)
            .build_windowed(window_builder, event_loop)
            .unwrap()
            .make_current()
            .unwrap()
    };

    let gl = unsafe { glow::Context::from_loader_function(|s| gl_window.get_proc_address(s)) };

    (gl_window, gl)
}
