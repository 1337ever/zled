//The io module handles keyboard input, windowing, and drawing.

extern crate gl;

use std::ffi::CStr;

mod input;
mod context;

use context::Context;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::Rect;

pub struct IoLoop {}

impl IoLoop {
    pub fn new() -> IoLoop {

        IoLoop {}
    }
    pub fn start(&mut self) {
        let el = EventLoop::new();
        let wb = WindowBuilder::new().with_title("zled");
        let wc = ContextBuilder::new().build_windowed(wb, &el).unwrap();

        let wc = unsafe { wc.make_current().unwrap() };

        let gl = gl::load_with(|ptr| wc.get_proc_address(ptr) as *const _);

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
                    WindowEvent::CursorMoved {..} => {
                        wc.swap_buffers().unwrap();
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
