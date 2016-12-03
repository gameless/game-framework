// use piston_window::*;
use file_utils::load_img;
use gfx_device_gl::Resources;
use gfx::Factory;
use graphics::Context;
use graphics::image;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use graphics::Transformed;
use graphics::ImageSize;

pub struct Sprite {
    tex: Texture,
    pub pos: (f64, f64),
}

impl Sprite {
    pub fn new(filename: &str) -> Sprite {
        Sprite {
            tex: load_img(filename),
            pos: (0.0, 0.0),
        }
    }

    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        // draw stuff
        image(&self.tex, c.transform.trans(self.pos.0, self.pos.1), gl);
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.tex.get_size()
    }

    pub fn get_width(&self) -> u32 {
        self.tex.get_width()
    }

    pub fn get_height(&self) -> u32 {
        self.tex.get_height()
    }
}
