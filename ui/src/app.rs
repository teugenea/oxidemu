#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]

use epaint::ImageData;
use std::time::Instant;
use eframe::egui::Frame;
use egui::ColorImage;
use gilrs::Gilrs;
use epaint::textures::TextureManager;

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
    texture: Option<egui::TextureHandle>,
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
            texture: Option::<egui::TextureHandle>::None,
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

}

pub fn show() {
    let clear_color = [0.0, 0.0, 0.0];

    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let display = create_display(&event_loop);
    let mut egui_glium = egui_glium::EguiGlium::new(&display);

    let mut window = OxidemuApp::default();

    let glium_image = glium::texture::RawImage2d::from_raw_rgba(
        window.sdl_render.get_pixels(window.em.get_video_buf_8()),
        (window.sdl_render.scaled_size[0], window.sdl_render.scaled_size[1]));
    let image_size = egui::Vec2::new(glium_image.width as f32, glium_image.height as f32);
    let glium_texture = glium::texture::SrgbTexture2d::new(&display, glium_image).unwrap();
    let glium_texture = std::rc::Rc::new(glium_texture);
    let texture_id = egui_glium.painter.register_native_texture(glium_texture);

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {

            let res = window.em.cycle();
            if res.video_buff_changed {
                let glium_image = glium::texture::RawImage2d::from_raw_rgba(
                    window.sdl_render.get_pixels(window.em.get_video_buf_8()),
                    (window.sdl_render.scaled_size[0], window.sdl_render.scaled_size[1]));
                let glium_texture = glium::texture::SrgbTexture2d::new(&display, glium_image).unwrap();
                let glium_texture = std::rc::Rc::new(glium_texture);
                egui_glium.painter.replace_native_texture(texture_id, glium_texture);
            }

            let needs_repaint = egui_glium.run(&display, |egui_ctx| {
                egui_ctx.set_visuals(egui::Visuals::dark());
                egui::Window::new("NativeTextureDisplay").show(egui_ctx, |ui| {
                    ui.image(texture_id, image_size);
                });
            });

            *control_flow = if window.quit {
                glutin::event_loop::ControlFlow::Exit
            } else if needs_repaint {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Poll
            };

            // if let Some(gilrs::Event { id, event, time }) = window.gilrs.next_event() {
                // println!("{:?} New event from {}: {:?}", time, id, event);
            // }
            {
                use glium::Surface as _;
                let mut target = display.draw();

                let color = egui::Rgba::from_rgb(0.1, 0.3, 0.2);
                target.clear_color(color[0], color[1], color[2], color[3]);

                // draw things behind egui here

                egui_glium.paint(&display, &mut target);

                // draw things on top of egui here

                target.finish().unwrap();
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

                egui_glium.on_event(&event);

                display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }

            _ => (),
        }
    });
}

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title("egui_glium example");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(false);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}
