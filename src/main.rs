#[macro_use]
extern crate log;
extern crate piston_window;
extern crate find_folder;

use piston_window::*;

mod logger;

use logger::GameLogger;
use log::LogLevel;
use std::process::exit;

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

    // init assets
    info!("init assets");
    let assets = match find_folder::Search::ParentsThenKids(3, 3).for_folder("assets") {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to open folder assets/");
            exit(1);
        }
    };

    info!("open rust.png");
    let rust_logo = assets.join("rust.png");
    info!("creating texture from rust.png");
    let rust_logo = match Texture::from_path(&mut window.factory,
                                             &rust_logo,
                                             Flip::None,
                                             &TextureSettings::new()) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to open file assets/rust.png");
            exit(1); //when I write this into a loader function have a placeholder image to use
        }
    };
    info!("begin");
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
        });
    }


}
