#[macro_use]
extern crate log;
extern crate zip;

extern crate piston;
extern crate find_folder;
extern crate gfx;
extern crate graphics;
extern crate opengl_graphics;
extern crate gfx_device_gl;
extern crate image;
extern crate piston_window;

// use piston::window::WindowSettings;
use piston::event_loop::*;
// use piston::input::*;
// use sdl2_window::Sdl2Window as Window;
use piston_window::PistonWindow;
use piston_window::WindowSettings;
use piston_window::OpenGL;
use opengl_graphics::GlGraphics;
use graphics::clear;
use piston_window::RenderEvent;
use piston_window::UpdateEvent;
use std::path::PathBuf;
use std::path::Path;
use graphics::Transformed;

mod logger;
mod file_utils;
mod sprite;
mod background;

use logger::GameLogger;
use log::LogLevel;
use std::process::exit;
use file_utils::load_img;
use sprite::Sprite;
use background::Background;

use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Flip;


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

    let mut window: PistonWindow = WindowSettings::new("Game Framework", (800, 600))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap_or_else(|e| {
            error!("Failed to build PistonWindow: {}", e);
            exit(1)
        });

    let assets = Path::new("assets");

    let mut logo = Sprite::new_from_zip("data.zip", &assets.join("logo.png"));

    let mut deltaX = 40.0;
    let mut deltaY = 20.0;

    logo.scale.0 = 0.5;

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;

    // test
    let test_img = Texture::from_path(&mut window.factory,
                                      &assets.join("tile_bg.png"),
                                      Flip::None,
                                      &TextureSettings::new())
        .unwrap();

    // load bg from zip
    // let bg = file_utils::load_img_from_zip("data.zip", &assets.join("bg.png"));
    let bg = Background::new_from_zip("data.zip", &assets.join("tile_bg.png"));

    info!("begin");
    let BG_COLOR = [0.0, 0.0, 0.0, 1.0];

    while let Some(e) = window.next() {

        // render
        window.draw_2d(&e, |mut c, gl| {
            clear(BG_COLOR, gl);
            // graphics::image(&bg, c.transform, gl);
            //
            // draw test graphic



            c.transform = c.transform.trans(cam_x, cam_y);
            // bg.draw((cam_x, cam_y), &c, gl);
            // logo.draw(&c, gl);
        });

        logo.draw(&mut window, &e);


        if let Some(u) = e.update_args() {

            cam_x = (logo.pos.0 * -1.0) + (400.0 - (logo.get_width() as f64 / 2.0));
            cam_y = (logo.pos.1 * -1.0) + (300.0 - (logo.get_height() as f64 / 2.0));


            // update
            if logo.pos.0 <= 0.0 {
                deltaX = 40.0;
            } else if logo.pos.0 as i32 + logo.get_width() as i32 >= 800 {
                deltaX = -40.0;
            }

            if logo.pos.1 <= 0.0 {
                deltaY = 20.0;
            } else if logo.pos.1 as i32 + logo.get_height() as i32 >= 600 {
                deltaY = -20.0;
            }

            logo.pos.0 += deltaX * u.dt;
            logo.pos.1 += deltaY * u.dt;

        }
    }

}
