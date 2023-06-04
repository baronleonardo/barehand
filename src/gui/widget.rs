use super::button::Button;
use super::{Color, Position, Size, backends::Backend};
use super::backends::GenericWidget;

#[derive(Debug)]
pub struct Rect
{
    pub size: Size,
    pub position: Position,
    pub color: Color,
}

pub enum WidgetType<'a, T: Backend<'a>>
{
    Raw(Widget<'a, T>),
    Button(Button<'a, T>),
}

impl<'a, T> WidgetType<'a, T>
    where T: Backend<'a>
{
    pub fn raw(&self) -> &T::Widget
    {
        match self
        {
            WidgetType::Raw(wtype) => &wtype.backend_widget,
            WidgetType::Button(wtype) => wtype.backend_button.raw(),
        }
    }
}

pub use super::backends::sdl2::Widget as SdlWidget;
pub struct Widget<'a, T: Backend<'a>>
{
    pub backend_widget: T::Widget,
}

impl<'a, T> Widget<'a, T>
    where T: Backend<'a>
{
    pub fn from_raw(widget: Widget<'a, T>) -> Self
    {
        widget
    }

    pub fn raw(&self) -> &T::Widget
    {
        &self.backend_widget
    }
}