use super::{Backend, backends::GenericButton, Widget};
pub struct Button<'a, T: Backend<'a>> {
    pub raw: T::Button,
}

impl<'a, T> GenericButton<'a, T> for Button<'a, T>
    where T: Backend<'a>
{
    fn new(label: String, position: super::Position, color: super::Color) -> Self {
        Self { raw: T::Button::new(label, position, color) }
    }

    fn raw(&self) -> &T::Widget {
        self.raw.raw()
    }
}

// impl<'a, T> Button<'a, T>
//     where T: Backend<'a>
// {
//     pub fn raw(&self) -> &T::Widget {
//         // self.raw.raw()
//         Widget { raw: self.raw.raw() }
//     }
// }