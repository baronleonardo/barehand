use sdl2::rect::Rect;

use crate::gui::color::Color;

pub trait Drawable
{
    fn raw(&self) -> &Vec<Rect>;
    fn color(&self) -> &Vec<Color>;
}