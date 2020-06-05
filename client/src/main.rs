mod glsupp;
mod font;
mod window;

fn main() {
    //let mut frender = font::FontRenderer::new("inputmono.ttf".to_string(), 600, 600);
    //frender.test();
    let mut display_loop = window::DisplayLoop::new();
    display_loop.start();
}
