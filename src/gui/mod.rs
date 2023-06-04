pub mod application;
pub mod backends;
pub mod window;
pub mod widget;
pub mod button;

mod color;
pub use color::Color;
mod position;
pub use position::Position;
mod size;
pub use size::Size;

use self::backends::Backend;

pub struct Sdl;
impl<'a> Backend<'a> for Sdl {
    type Application = crate::gui::backends::sdl2::Application<'a>;
    type Window = crate::gui::backends::sdl2::Window<'a>;
    type Widget = crate::gui::backends::sdl2::Widget;
    type Button = crate::gui::backends::sdl2::Button;
}