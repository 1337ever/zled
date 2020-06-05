//Module for font rendering
extern crate freetype as ft;
use gl::types::*;

use freetype::Library;

use std::fs::File;
use std::ffi::CString;
use std::ptr;
use std::char;
use std::collections::HashMap;

mod glsupp;


struct Glyph {
    texture: u32, //ID of the glyph texture
    size: [i32; 2],
    bearing: [i32; 2], //offset from the origin to the left/top of glyph
    advance: i64, //distance from the origin to the origin of the next glyph
}

pub struct FontRenderer {
    fontname: String,
    map: HashMap<char, Glyph>, //the font map
    program: GLuint,
}

impl FontRenderer {
    pub fn new(fontname: String) -> FontRenderer {
        //start loading shaders
        let vert = fs::read_to_string("src/bin/display/font/font.vsh")
            .expect("failed to load vertex shader!");
        let vs = glsupp::compile_shader(&vert, gl::VERTEX_SHADER);

        //load fonts and all related stuff
        let library = ft::Library::init().unwrap();
        let font = library.new_face(fontname.clone(), 0).unwrap();

        //generate glyph map
        let mut map = HashMap::new();
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            for x in 0..127 {
                //load glyph
                let char = font.load_char(x, ft::face::LoadFlag::RENDER).unwrap();
                let glyph = font.glyph().bitmap();
                let mut texture = ptr::null_mut();
                gl::GenTextures(1, texture);
                gl::BindTexture(gl::TEXTURE_2D, *texture);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    glyph.width(),
                    glyph.rows(),
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    //glyph.buffer().as_mut_ptr(),
                    ptr::null(),
                );
                //configure texture
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                //create glyph struct
                let g_struct = Glyph {
                    texture: *texture,
                    size: [glyph.width(), glyph.rows()],
                    bearing: [font.glyph().bitmap_left(), font.glyph().bitmap_top()],
                    advance: font.glyph().advance().x,
                };
                map.insert(char::from_digit(x as u32, 10).unwrap(), g_struct);

            }
        }

        FontRenderer {
            fontname: fontname,
            map: map,
        }
    }
}
