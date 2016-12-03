#[macro_use]
extern crate log;
extern crate piston_window;
extern crate find_folder;
extern crate gfx;

use piston_window::*;

mod logger;
mod file_utils;

use logger::GameLogger;
use log::LogLevel;
use std::process::exit;
use file_utils::load_img;

fn main() {
    // possible log levels
    // trace!()
    // debuge!()
    // info!()
    // warn!()
    // error!()
    logger::init(LogLevel::Info);
    info!("init log");

    // these shouldn't show because we set log level to info
    debug!("test");
    trace!("2 spoopy 4 me");

    // start piston
    info!("opening Piston Window");
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = match WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .opengl(opengl)
        .build() {
        Ok(window) => window,
        Err(e) => {
            error!("Failed to open piston window");
            exit(1); //quit nothing more to do here
        }
    };

    let rust_logo = load_img("rust.png.old", &mut window.factory);
    info!("begin");
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
        });
    }


}
