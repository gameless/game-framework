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
            error!("Failed to open file assets/{} as image", img_name);
            return Texture::empty().unwrap();
        }
    };

    return img;
}

pub fn load_zip_archive(zipname: &str) -> ZipArchive<File> {
    let zipfile = match File::open(&zipname) {
        Err(e) => {
            error!("Could not open file: {}", zipname);
            exit(1)
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
            exit(1);
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
            exit(1);
        }
    };

    let tex = Texture::from_image(&img.to_rgba(), &TextureSettings::new());
    tex
}
