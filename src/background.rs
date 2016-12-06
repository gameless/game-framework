use opengl_graphics::Texture;
use file_utils::load_img_from_zip;
use std::path::PathBuf;
use graphics::Context;
use opengl_graphics::GlGraphics;
use graphics::image;
use graphics::ImageSize;
use graphics::Transformed;

pub struct Background {
    tex: Texture,
}

impl Background {
    pub fn new_from_zip(zipname: &str, filename: &PathBuf) -> Background {
        info!("should change Background.draw() to be not a hardcoded resolution");

        Background { tex: load_img_from_zip(zipname, filename) }
    }

    pub fn draw(&self, cam_offset: (f64, f64), c: &Context, gl: &mut GlGraphics) {
        // re assign cam_offset to be the next lowest multiple of tex.width
        let cam_offset =
            ((cam_offset.0 / self.tex.get_width() as f64).ceil() * self.tex.get_width() as f64,
             (cam_offset.1 / self.tex.get_height() as f64).ceil() * self.tex.get_height() as f64);

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
    }
}
