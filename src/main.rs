#[macro_use]
extern crate log;
extern crate zip;
extern crate sdl2_window;
extern crate piston;
extern crate find_folder;
extern crate gfx;
extern crate graphics;
extern crate opengl_graphics;
extern crate gfx_device_gl;
extern crate image;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::OpenGL;
use opengl_graphics::GlGraphics;
use graphics::clear;
// use graphics::image;

mod logger;
mod file_utils;
mod sprite;

use logger::GameLogger;
use log::LogLevel;
use std::process::exit;
use file_utils::load_img;
use sprite::Sprite;

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
    let mut window: Window = match WindowSettings::new("Game Framework", [800, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .build() {
        Ok(window) => window,
        Err(e) => {
            error!("Failed to open piston window");
            exit(1); //quit nothing more to do here
        }
    };

    let mut gl = GlGraphics::new(opengl);

    let mut logo = Sprite::new("logo.png");

    let mut deltaX = 40.0;
    let mut deltaY = 20.0;

    logo.scale.0 = 0.5;

    // load bg from zip
    let bg = file_utils::load_img_from_zip("data.zip", "bg.png");

    info!("begin");
    let BG_COLOR = [0.0, 0.0, 0.0, 1.0];
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            // render
            gl.draw(r.viewport(), |mut c, gl| {
                clear(BG_COLOR, gl);
                graphics::image(&bg, c.transform, gl);
                logo.draw(&c, gl);
            });
        }

        if let Some(u) = e.update_args() {
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
