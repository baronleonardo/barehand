pub use super::{backends::Backend, Position, Color};
pub use super::backends::GenericButton;

pub use super::backends::sdl2::Button as SdlButton;
pub struct Button<'a, T: Backend<'a>>
{
    pub backend_button: T::Button,
}

impl<'a, T> Button<'a, T>
    where T: Backend<'a>
{
    pub fn new(label: String, position: Position, color: Color) -> Button<'a, T>
    {
        Button { backend_button: T::Button::new(label, position, color) }
    }
}