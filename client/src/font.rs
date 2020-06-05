//Module for font rendering
extern crate freetype as ft;

use freetype::Library;
use std::collections::HashMap;


struct Glyph {
    texture: u32, //ID of the glyph texture
    size: [i32; 2],
    bearing: [i32; 2], //offset from the origin to the left/top of glyph
    advance: i64, //distance from the origin to the origin of the next glyph
}

pub struct FontRenderer {
    fontname: String,
    map: HashMap<u32, Glyph>, //the font map
}
/*
impl FontRenderer {
    pub fn new(fontname: String) -> FontRenderer {
        //start loading shaders
        let vert = fs::read_to_string("data/font.vsh")
            .expect("failed to load vertex shader!");
        let vs = compile_shader(&vert, gl::VERTEX_SHADER);
        let frag = fs::read_to_string("data/font.fsh")
            .expect("failed to load fragment shader!");
        let fs = compile_shader(&frag, gl::FRAGMENT_SHADER);
        println!("shaders compiled");

        //load fonts and all related stuff
        let library = ft::Library::init().unwrap();
        let font = library.new_face(fontname.clone(), 0).unwrap();

        //generate glyph map
        let mut map = HashMap::new();
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            for x in 0..127 {
                //load glyph
                font.set_char_size(40*64, 0, 50, 0).unwrap();
                font.load_char(x, ft::face::LoadFlag::RENDER).unwrap();
                let glyph = font.glyph().bitmap();
                let mut texture: GLuint = 0;
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    glyph.width(),
                    glyph.rows(),
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    glyph.buffer().as_ptr() as *const std::ffi::c_void,
                );
                //configure texture
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                //create glyph struct
                let g_struct = Glyph {
                    texture: texture,
                    size: [glyph.width(), glyph.rows()],
                    bearing: [font.glyph().bitmap_left(), font.glyph().bitmap_top()],
                    advance: font.glyph().advance().x,
                };
                let c = x as c_char;
                //println!("{}", std::str::from_utf8(&[c as u8]).unwrap());
                map.insert(c, g_struct);
                //map.insert(char::from_digit(8, 10).unwrap(), g_struct);

            }
        }

        FontRenderer {
            fontname: fontname,
            map: map,
            program: link_program(vs, fs),
        }
    }
    pub fn render_glyph(&mut self, vao: GLuint, vbo: GLuint, id: c_char, x: i32, y: i32, scale: f32, color: [f32; 3]) {
        unsafe {
            let projection: glm::Mat4 = glm::ortho(0.0f32, 800.0, 600.0, 0.0f32, 0.0f32, 1.0f32);
            //activate render state
            gl::UseProgram(self.program);
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, to_glchar("projection")), 1, gl::FALSE, glm::value_ptr(&projection).as_ptr());
            gl::UseProgram(self.program);
            gl::Uniform3f(gl::GetUniformLocation(self.program, to_glchar("textColor")), color[0], color[1], color[2]);
            //panic!("{:?}", gl::GetUniformLocation(self.program, loc));
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(vao);

            let tg = &self.map[&id];
            let xpos = (x + tg.bearing[0]) as f32 * scale ;
            let ypos = (y - (tg.size[1] - tg.bearing[1])) as f32 * scale;
            let w = tg.size[0] as f32 * scale;
            let h = tg.size[1] as f32 * scale;
            //update vbo
            let vertices = [
                [xpos, ypos+h, 0.0f32, 0.0f32],
                [xpos, ypos, 0.0f32, 1.0f32],
                [xpos+w, ypos, 1.0f32, 1.0f32],

                [xpos, ypos+h, 0.0f32, 0.0f32],
                [xpos+w, ypos, 1.0f32, 1.0f32],
                [xpos+w, ypos+h, 1.0f32, 0.0f32],
            ];
            //render glyph to quad
            gl::BindTexture(gl::TEXTURE_2D, tg.texture);
            //update vbo memory
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, std::mem::size_of_val(&vertices) as isize, glm::value_ptr(vertices));

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            //render quad
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
    pub fn render_text(&mut self, vao: GLuint, vbo: GLuint, text: CString, x: i32, y: i32, scale: f32, color: [f32; 3]) {
        let mut x = x;
        for c in text.into_bytes() {
            self.render_glyph(vao, vbo, c as i8, x, y, scale, color);
            x += ((self.map.get(&(c as i8)).unwrap().advance >> 6) as f32 * scale) as i32;
            println!("{}", c);
        }

    }
}
*/
