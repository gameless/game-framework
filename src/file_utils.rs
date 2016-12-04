use std::io::prelude::*;
use gfx_device_gl::Resources;
use gfx::Factory;
use find_folder::Search;
use opengl_graphics::Texture;
use zip::ZipArchive;
use zip::read::ZipFile;
use std::fs::File;
use std::process::exit;
use std::io::BufReader;
use std::io::Cursor;
use image;
use opengl_graphics::TextureSettings;
use image::ImageFormat;
use std::ops::Deref;
use image::GenericImage;
use image::Rgba;
use image::Pixel;

pub fn get_placeholder_tex() -> Texture {
    let mut img = image::DynamicImage::new_rgba8(32, 32);
    for x in 0..32 {
        for y in 0..32 {
            let mut color: [u8; 4] = [0, 0, 0, 255];

            if (x < 16 && y < 16) || (x > 16 && y > 16) {
                // magenta
                color[0] = 255; //red
                color[2] = 255; //blue
            }

            img.put_pixel(x,
                          y,
                          Rgba::from_channels(color[0], color[1], color[2], color[3]));
        }
    }
    Texture::from_image(&img.to_rgba(), &TextureSettings::new())
}


pub fn load_img(img_name: &str) -> Texture {
    let path = match Search::ParentsThenKids(3, 3).for_folder("assets") {
        Ok(s) => s,
        Err(e) => {
            error!("failed to open folder assets/");
            return get_placeholder_tex();
        }
    };

    let path = path.join(img_name);
    let img = match Texture::from_path(&path) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to open file assets/{} as image", img_name);
            return get_placeholder_tex();
        }
    };

    return img;
}

pub fn load_zip_archive(zipname: &str) -> ZipArchive<File> {
    let zipfile = match File::open(&zipname) {
        Err(e) => {
            error!("Could not open file: {}", zipname);
            exit(1);
        }
        Ok(s) => s,
    };

    let mut archive = match ZipArchive::new(zipfile) {
        Ok(s) => s,
        Err(e) => {
            error!("file: {} is of unknown type", zipname);
            exit(1);
        }
    };

    archive
}

// pub fn load_file_from_zip(zipname: &str, filename: &str) -> ZipFile<'a> {
// let archive = load_zip_archive(zipname);
//
// let mut file = match archive.by_name(filename) {
// Ok(file) => file,
// Err(..) => {
// error!("File: {} not found in archive: {}", filename, zipname);
// exit(1);
// }
// };
// file
// }

pub fn load_img_from_zip(zipname: &str, imgname: &str) -> Texture {
    let mut archive = load_zip_archive(zipname);

    let mut file = match archive.by_name(imgname) {
        Ok(file) => file,
        Err(..) => {
            error!("File: {} not found in archive: {}", imgname, zipname);
            return get_placeholder_tex();
        }
    };

    // turn file into [u8]
    let mut bytes: Vec<u8> = Vec::new();
    for byte in file.bytes() {
        bytes.push(byte.unwrap());
    }

    let bytes = bytes.deref();

    let img = match image::load(Cursor::new(bytes), ImageFormat::PNG) {
        Ok(s) => s,
        Err(e) => {
            error!("failed to load {}/{} as image", zipname, imgname);
            return get_placeholder_tex();
        }
    };

    let tex = Texture::from_image(&img.to_rgba(), &TextureSettings::new());
    tex
}
