use sdl2::surface::Surface;

use crate::gui::Rect;

use super::{Button, Font};

impl Into<sdl2::rect::Rect> for &Rect
{
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            self.position.0 as i32,
            self.position.1 as i32,
            self.size.0,
            self.size.1,
        )
    }
}

pub enum WidgetType<'a>
{
    Raw(&'a Widget<'a>),
    Button(&'a Button<'a>),
}

impl<'a> WidgetType<'a>
{
    pub fn raw(&self) -> &Widget
    {
        match self
        {
            WidgetType::Raw(wtype) => &wtype,
            WidgetType::Button(wtype) => &wtype.raw(),
        }
    }
}

pub struct Widget<'a>
{
    pub raw: Vec<Rect>,
    pub surfaces: Vec<Surface<'a>>,
    pub font: &'a Font<'a>,
}

impl<'a> Widget<'a>
{
    pub fn new(rectangles: Vec<Rect>, surfaces: Vec<Surface<'a>>, font: &'a Font<'a>) -> Self
    {
        Widget { raw: rectangles, font, surfaces }
    }
}

impl<'a> Widget<'a>
{
    pub fn raw(&self) -> &Widget
    {
        &self
    }

    pub fn from_raw(raw: Widget<'a>) -> Self where Self: Sized {
        raw
    }
}