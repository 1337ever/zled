//Module for font rendering
extern crate png;
extern crate cairo;

use cairo::{ ImageSurface, Format, Context };
use std::fs::File;

pub struct FontRenderer {
    fontname: String,
    context: Context,
    surface: ImageSurface,
}

impl FontRenderer {
    pub fn new(font: String, surfwidth: i32, surfheight: i32) -> FontRenderer {
        let surface = ImageSurface::create(Format::ARgb32, surfwidth, surfheight)
            .expect("Failed to create a surface!");
        let context = Context::new(&surface);

        FontRenderer {
            fontname: font,
            context: context,
            surface: surface,
        }
    }
    pub fn test(&mut self) {
        let mut file = File::create("output.png")
            .expect("Failed to create file!");
        self.context.set_source_rgb(1.0, 0.0, 0.0);
        self.context.paint();
        self.surface.write_to_png(&mut file)
            .expect("Failed to write png");
    }
}
