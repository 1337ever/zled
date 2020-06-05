//module to create a window context
extern crate glutin;

use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
use glutin::Rect;

pub struct Context {
    pub el: EventLoop<()>,
    pub wc: ContextWrapper<PossiblyCurrent, glutin::window::Window>
}

impl Context {
    pub fn new() -> Context {
        let el = EventLoop::new();
        let wb = WindowBuilder::new().with_title("zled");
        let wc = ContextBuilder::new().build_windowed(wb, &el).unwrap();

        let wc = unsafe { wc.make_current().unwrap() };

        Context {
            el: el,
            wc: wc,
        }
    }
}
