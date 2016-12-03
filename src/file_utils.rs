// use piston_window::*;
use gfx_device_gl::Resources;
use gfx::Factory;
use find_folder::Search;
use opengl_graphics::Texture;
// use piston_window::{Flip, TextureSettings};

pub fn load_img(img_name: &str) -> Texture {
    let path = match Search::ParentsThenKids(3, 3).for_folder("assets") {
        Ok(s) => s,
        Err(e) => {
            error!("failed to open folder assets/");
            return Texture::empty().unwrap();
        }
    };

    let path = path.join(img_name);
    let img = match Texture::from_path(&path) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to open file assets/{}", img_name);
            return Texture::empty().unwrap();
        }
    };

    return img;
}
