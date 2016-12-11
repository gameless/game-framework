use piston_window::PistonWindow as Window;
use piston_window::Event;

pub struct DrawContext<'a> {
    pub cam: (f64, f64),
    pub window: &'a mut Window,
    pub event: &'a Event,
}
