extern crate gfx_graphics;

// use piston_window::*;
use file_utils::load_img;
use file_utils::load_img_from_zip;
use gfx_device_gl::Resources;
use gfx_device_gl::Factory;
use graphics::Context;
use piston_window::image;
use piston_window::PistonWindow as Window;
use piston_window::Texture;
use graphics::Transformed;
use graphics::ImageSize;
use std::path::PathBuf;
use piston_window::Event;
use draw::DrawContext;

pub struct Sprite {
    tex: Texture<Resources>,
    pub pos: (f64, f64),
    pub scale: (f64, f64),
}

impl Sprite {
    pub fn new(factory: &mut Factory, filename: &str) -> Sprite {
        Sprite {
            tex: load_img(factory, filename),
            pos: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }

    pub fn new_from_zip(factory: &mut Factory, zipfile: &str, imgname: &PathBuf) -> Sprite {
        Sprite {
            tex: load_img_from_zip(factory, zipfile, imgname),
            pos: (0.0, 0.0),
            scale: (1.0, 1.0),
        }
    }

    pub fn draw(&self, context: &mut DrawContext) {
        // draw stuff
        //
        let cam = context.cam.clone();

        context.window.draw_2d(context.event, |mut c, gl| {
            c.transform = c.transform.trans(cam.0, cam.1);
            image(&self.tex,
                  c.transform.trans(self.pos.0, self.pos.1).scale(self.scale.0, self.scale.1),
                  gl);
        });
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
