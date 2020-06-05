//The io module handles keyboard input, windowing, and drawing.
extern crate gl;

use std::ffi::CStr;
use std::ffi::CString;
use std::fs;
use std::ptr;
use std::mem;


use glutin::event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::Rect;
use glutin::dpi::Size;
use glutin::dpi::PhysicalSize;

use crate::font::FontRenderer;


pub struct DisplayLoop {}


impl DisplayLoop {
    pub fn new() -> DisplayLoop {

        DisplayLoop {}
    }
    pub fn start(&mut self) {
        let size = PhysicalSize::new(800, 600);
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("zled")
            .with_inner_size(size);
        let wc = ContextBuilder::new().build_windowed(wb, &el).unwrap();

        let wc = unsafe { wc.make_current().unwrap() };

        let gl = gl::load_with(|ptr| wc.get_proc_address(ptr) as *const _);

        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            //set up VAO and VBO
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<f32>()*6*4) as isize, ptr::null(), gl::DYNAMIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, (4*mem::size_of::<f32>()) as i32, ptr::null_mut());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        let mut font_renderer = FontRenderer::new("data/inputmono.ttf".to_string());

        let mut co = 0.2f32;

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
                            /*
                            font_renderer.render_glyph(vao, vbo, "t".to_string(), 20, 20, 1.0, [255.0, 255.0, 255.0]);
                            wc.swap_buffers().unwrap();
                            */
                            wc.window().request_redraw();
                        }
                        _ => (),
                    },
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    unsafe {
                        gl::ClearColor(co, 0.3f32, 0.3f32, 1.0f32);
                        //co += 1.01;
                        gl::Clear(gl::COLOR_BUFFER_BIT);

                        //font_renderer.render_glyph(vao, vbo, "t".to_string(), 25, 25, 1.0, [0.5, 0.8, 0.2]);
                        font_renderer.render_text(vao, vbo, CString::new("test").unwrap(), 25, 25, 1.0, [0.5, 0.8, 0.2]);

                        wc.swap_buffers().unwrap();
                    }
                }
                _ => (),
            }
        });
    }
}
