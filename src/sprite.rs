// use piston_window::*;
use file_utils::load_img;
use file_utils::load_img_from_zip;
use gfx_device_gl::Resources;
use gfx::Factory;
use graphics::Context;
use graphics::image;
use opengl_graphics::GlGraphics;
use opengl_graphics::Texture;
use graphics::Transformed;
use graphics::ImageSize;
use std::path::PathBuf;

pub struct Sprite {
    tex: Texture,
    pub pos: (f64, f64),
    pub scale: (f64, f64),
}

impl Sprite {
    pub fn new(filename: &str) -> Sprite {
        Sprite {
            tex: load_img(filename),
            pos: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }

    pub fn new_from_zip(zipfile: &str, imgname: &PathBuf) -> Sprite {
        Sprite {
            tex: load_img_from_zip(zipfile, imgname),
            pos: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }

    pub fn draw(&self, c: &Context, gl: &mut GlGraphics) {
        // draw stuff
        image(&self.tex,
              c.transform.trans(self.pos.0, self.pos.1).scale(self.scale.0, self.scale.1),
              gl);
    }

    pub fn get_size(&self) -> (u32, u32) {
        let mut size = self.tex.get_size();
        size.0 = (size.0 as f64 * self.scale.0) as u32;
        size.1 = (size.1 as f64 * self.scale.1) as u32;
        size
    }

    pub fn get_width(&self) -> u32 {
        (self.tex.get_width() as f64 * self.scale.0) as u32
    }

    pub fn get_height(&self) -> u32 {
        (self.tex.get_height() as f64 * self.scale.1) as u32
    }
}
