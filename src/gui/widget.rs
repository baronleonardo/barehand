use super::{Backend, backends::GenericWidget, Rect};
pub struct Widget<'a, T: Backend<'a>> {
    pub raw: T::Widget,
}

impl<'a, T> GenericWidget<'a> for Widget<'a, T>
    where T: Backend<'a>
{
    fn new() -> Self where Self: Sized {
        Self { raw: T::Widget::new() }
    }

    fn from_raw(rectangles: Vec<Rect>) -> Self where Self: Sized {
        Self { raw: T::Widget::from_raw(rectangles) }
    }

    fn raw(&self) -> &Vec<Rect> {
        self.raw.raw()
    }
}