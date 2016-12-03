use piston_window::*;
use gfx::Resources;
use gfx::Factory;
use find_folder::Search;


pub fn load_img<R, F>(img_name: &str, factory: &mut F) -> Texture<R>
    where R: Resources,
          F: Factory<R>
{
    let path = match Search::ParentsThenKids(3, 3).for_folder("assets") {
        Ok(s) => s,
        Err(e) => {
            error!("failed to open folder assets/");
            return Texture::empty(factory).unwrap();
        }
    };

    let path = path.join(img_name);
    let img = match Texture::from_path(factory, &path, Flip::None, &TextureSettings::new()) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to open file assets/{}", img_name);
            return Texture::empty(factory).unwrap();
        }
    };

    return img;
}
