use piston_window::Texture;
use file_utils::load_img_from_zip;
use std::path::PathBuf;
use graphics::Context;
use piston_window::image;
use graphics::ImageSize;
use graphics::Transformed;
use gfx_device_gl::Factory;
use gfx_device_gl::Resources;
use piston_window::PistonWindow as Window;
use piston_window::Event;

pub struct Background {
    tex: Texture<Resources>,
}

impl Background {
    pub fn new_from_zip(factory: &mut Factory, zipname: &str, filename: &PathBuf) -> Background {
        info!("should change Background.draw() to be not a hardcoded resolution");

        Background { tex: load_img_from_zip(factory, zipname, filename) }
    }

    pub fn draw(&self, cam_offset: (f64, f64), window: &mut Window, event: &Event) {
        window.draw_2d(event, |mut c, gl| {
            c.transform = c.transform.trans(cam_offset.0, cam_offset.1);
            // re assign cam_offset to be the next lowest multiple of tex.width
            let cam_offset = ((cam_offset.0 / self.tex.get_width() as f64).ceil() *
                              self.tex.get_width() as f64,
                              (cam_offset.1 / self.tex.get_height() as f64).ceil() *
                              self.tex.get_height() as f64);

            // loop through screen
            let mut x: f64 = cam_offset.0 * -1.0;
            let mut y: f64 = cam_offset.1 * -1.0;
            while x < 800.0 - cam_offset.0 + self.tex.get_width() as f64 {
                y = cam_offset.1 * -1.0;
                while y < 600.0 - cam_offset.1 + self.tex.get_height() as f64 {
                    image(&self.tex, c.transform.trans(x, y), gl);
                    y += self.tex.get_height() as f64;
                }
                x += self.tex.get_width() as f64;
            }
        });
    }
}
