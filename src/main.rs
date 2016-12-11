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
mod draw;

use logger::GameLogger;
use log::LogLevel;
use std::process::exit;
use file_utils::load_img;
use sprite::Sprite;
use background::Background;

use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Flip;
use draw::DrawContext;
use piston_window::Window;


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

    let mut logo = Sprite::new_from_zip(&mut window.factory, "data.zip", &assets.join("logo.png"));

    let mut deltaX = 40.0;
    let mut deltaY = 20.0;

    logo.scale.0 = 0.5;

    let mut cam_x = 0.0;
    let mut cam_y = 0.0;
	
    // load bg from zip
    // let bg = file_utils::load_img_from_zip("data.zip", &assets.join("bg.png"));
    let bg = Background::new_from_zip(&mut window.factory, "data.zip", &assets.join("tile_bg.png"));

    info!("begin");
    let BG_COLOR = [0.0, 0.0, 0.0, 1.0];
	
	let mut res = (800, 600);

    while let Some(e) = window.next() {
		
		// if resolution has changed log it
		if window.size().width != res.0 || window.size().height != res.1{
			res.0 = window.size().width;
			res.1 = window.size().height;
			debug!("Resolution has changed: {}, {}", res.0, res.1);
		}
		

        // render
        window.draw_2d(&e, |mut c, gl| {
            clear(BG_COLOR, gl);
        });

        // create DrawContext
        let mut d_context = DrawContext {
            cam: (cam_x, cam_y),
            window: &mut window,
            event: &e,
        };

        bg.draw(&mut d_context);
        logo.draw(&mut d_context);


        if let Some(u) = e.update_args() {

            cam_x = (logo.pos.0 * -1.0) + ((res.0 / 2) as f64 - (logo.get_width() as f64 / 2.0));
            cam_y = (logo.pos.1 * -1.0) + ((res.1 / 2) as f64 - (logo.get_height() as f64 / 2.0));


            // update
            if logo.pos.0 <= 0.0 {
                deltaX = 40.0;
            } else if logo.pos.0 as i32 + logo.get_width() as i32 >= res.0 as i32 {
                deltaX = -40.0;
            }

            if logo.pos.1 <= 0.0 {
                deltaY = 20.0;
            } else if logo.pos.1 as i32 + logo.get_height() as i32 >= res.1 as i32 {
                deltaY = -20.0;
            }

            logo.pos.0 += deltaX * u.dt;
            logo.pos.1 += deltaY * u.dt;

        }
    }

}
