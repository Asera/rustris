#![windows_subsystem = "windows"]

mod tetronimoe;
mod game_state;
mod state_renderer;
mod collision_checker;
mod texture_bag;
mod shader_program;
mod normal;
mod vertex;

#[macro_use]
extern crate glium;

use winit::event_loop::ControlFlow;
use glium::{glutin, Surface};
use std::time::Instant;
use std::fs::File;
use winit::event::{Event, WindowEvent, ElementState, VirtualKeyCode, DeviceEvent};
use crate::game_state::GameState;
use crate::state_renderer::render_sate;
use std::borrow::Borrow;
use crate::vertex::Vertex;
use crate::normal::Normal;
use crate::texture_bag::TextureBag;
use crate::collision_checker::position_is_clear;

fn main() {
    implement_vertex!(Vertex, position, texture);
    implement_vertex!(Normal, normal);

    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new().with_title("RTetris");
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window_builder, context, &event_loop).unwrap();
    let textures = TextureBag::init(&display);

    let mut frame_counter: u32 = 0;
    let mut frame_start = Instant::now();
    let mut frame_duration = Instant::now().duration_since(frame_start).as_millis() as u64;

    let font = glium_text_nxt::FontTexture::new(&display, File::open("ClearSans-Medium.ttf").unwrap(), 14).unwrap();
    let mut in_focus = true;

    let mut game_state = GameState::init();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        if frame_duration >= 16 {
            frame_counter += 1;

            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
            render_sate(&game_state, &display, &mut target, font.borrow(), &textures);
            target.finish().unwrap();
            frame_start = Instant::now();

            game_state.update();
            if game_state.game_is_finished() {
                game_state.restart();
            }
        }

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::Resized(size) => display.gl_window().resize(size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Focused(focused) => in_focus = focused,
                    _ => (),
                }
            },
            Event::DeviceEvent {event, ..} => {
                match event {
                    DeviceEvent::Key(input) => {
                        if in_focus && input.state == ElementState::Pressed {
                            match input.virtual_keycode.unwrap() {
                                VirtualKeyCode::Numpad1
                                | VirtualKeyCode::Numpad5
                                | VirtualKeyCode::Numpad9 => game_state.rotate_clockwise(),
                                VirtualKeyCode::Numpad3
                                | VirtualKeyCode::Numpad7 => game_state.rotate_counter_clockwise(),
                                VirtualKeyCode::Numpad8 => game_state.set_hard_drop_gravity(),
                                VirtualKeyCode::Numpad2 => game_state.set_soft_drop_gravity(),
                                VirtualKeyCode::Numpad4 => game_state.left_shift(),
                                VirtualKeyCode::Numpad6 => game_state.right_shift(),
                                VirtualKeyCode::Numpad0 => game_state.hold(),
                                VirtualKeyCode::Escape | VirtualKeyCode::F1 => game_state.pause(),
                                _ => (),
                            }
                        }

                        if in_focus && input.state == ElementState::Released {
                            match input.virtual_keycode.unwrap() {
                                VirtualKeyCode::Numpad8 => game_state.reset_gravity(),
                                VirtualKeyCode::Numpad2 => game_state.reset_gravity(),
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            },
            Event::RedrawRequested(_) => {
                display.swap_buffers();
            },
            _ => (),
        }

        frame_duration = Instant::now().duration_since(frame_start).as_millis() as u64;
    });
}
