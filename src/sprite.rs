// use piston_window::*;
use file_utils::load_img;
use gfx_device_gl::Resources;
use gfx::Factory;
use graphics::Context;
use graphics::image;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use graphics::Transformed;

pub struct Sprite {
    tex: Texture,
    pub pos: [f64; 2],
}

impl Sprite {
    pub fn new(filename: &str) -> Sprite {
        Sprite {
            tex: load_img(filename),
            pos: [0.0, 0.0],
        }
    }

    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        // draw stuff
        image(&self.tex, c.transform.trans(self.pos[0], self.pos[1]), gl);
    }
}
