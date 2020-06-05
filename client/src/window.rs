//The io module handles keyboard input, windowing, and drawing.

pub mod window;

extern crate gl;

use std::ffi::CStr;
use std::fs;

mod input;
mod font;

use context::Context;

use glutin::event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::Rect;

pub struct DisplayLoop {}


impl DisplayLoop {
    pub fn new() -> DisplayLoop {

        DisplayLoop {}
    }
    pub fn start(&mut self) {
        let el = EventLoop::new();
        let wb = WindowBuilder::new().with_title("zled");
        let wc = ContextBuilder::new().build_windowed(wb, &el).unwrap();

        let wc = unsafe { wc.make_current().unwrap() };

        let gl = gl::load_with(|ptr| wc.get_proc_address(ptr) as *const _);
        /*
        */

        let font_renderer = font::FontRenderer::new("data/inputmono.ttf".to_string());
        el.run(move |event, _, control_flow| {
            println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::Resized(physical_size) => {
                        wc.resize(physical_size)
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    }
                    /*WindowEvent::CursorMoved {..} => {
                        wc.swap_buffers().unwrap();
                    }*/
                    WindowEvent::KeyboardInput {
                        input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                        ..
                    } => match (virtual_code, state) {
                        (VirtualKeyCode::Escape, _) => {
                            *control_flow = ControlFlow::Exit
                        }
                        (VirtualKeyCode::F, ElementState::Pressed) => {
                           
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
