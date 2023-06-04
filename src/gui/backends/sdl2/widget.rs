use crate::gui::{Rect, Backend, backends::{GenericWidget, GenericButton}};

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

pub enum WidgetType<'a, T>
    where T: Backend<'a>
{
    Raw(&'a T::Widget),
    Button(&'a T::Button),
}

impl<'a, T> WidgetType<'a, T>
    where T: Backend<'a>
{
    pub fn raw(&self) -> &Vec<Rect>
    {
        match self
        {
            WidgetType::Raw(wtype) => &(unsafe { std::mem::transmute::<&&<T as Backend<'_>>::Widget, &Widget>(wtype) }).rectangles,
            WidgetType::Button(wtype) => &(unsafe { std::mem::transmute::<&<T as Backend<'_>>::Widget, &Widget>(wtype.raw()) }).rectangles,
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

impl<'a> GenericWidget<'a> for Widget
{
    fn new() -> Self where Self: Sized
    {
        Self { rectangles: vec![] }
    }

    fn raw(&self) -> &Vec<Rect>
    {
        &self.rectangles
    }

    fn from_raw(raw: Vec<Rect>) -> Self where Self: Sized {
        Self { rectangles: raw }
    }
}

impl Widget
{
    pub fn raw(&self) -> &Widget
    {
        &self
    }

    pub fn from_raw(raw: Vec<Rect>) -> Self where Self: Sized {
        Self { rectangles: raw }
    }
}