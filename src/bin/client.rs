
mod io;
//mod font;

fn main() {
    //let mut frender = font::FontRenderer::new("inputmono.ttf".to_string(), 600, 600);
    //frender.test();
    let mut io_loop = io::IoLoop::new();
    io_loop.start();
}
