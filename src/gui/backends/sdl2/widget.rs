use crate::gui::Rect;

use super::Button;

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
    Raw(&'a Widget),
    Button(&'a Button),
}

impl<'a> WidgetType<'a>
{
    pub fn raw(&self) -> &Vec<Rect>
    {
        match self
        {
            WidgetType::Raw(wtype) => &wtype.rectangles,
            WidgetType::Button(wtype) => &wtype.raw().rectangles,
        }
    }
}

#[derive(Debug)]
pub struct Widget
{
    pub rectangles: Vec<Rect>,
}

impl Widget
{
    pub fn new(rectangles: Vec<Rect>) -> Self
    {
        Widget { rectangles }
    }
}

impl<'a> Widget
{
    pub fn raw(&self) -> &Widget
    {
        &self
    }

    pub fn from_raw(raw: Widget) -> Self where Self: Sized {
        raw
    }
}