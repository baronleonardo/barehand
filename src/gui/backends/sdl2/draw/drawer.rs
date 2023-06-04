use sdl2::{render::Canvas, video::Window};
use crate::gui::color::Color;

use super::Drawable;

pub struct Drawer<'a>
{
    canvas: &'a mut Canvas<Window>
}

impl<'a> Drawer<'a>
{
    pub fn new(canvas: &'a mut Canvas<Window>) -> Drawer<'a>
    {
        Drawer { canvas }
    }

    pub fn draw<T>(&mut self, drawable_obj: sdl2::rect::Rect, color: Color)
    where T: Drawable
    {
        self.canvas.set_draw_color(color.rgb());
        // self.canvas.draw_rect(drawable_obj).map_err(|e| eprintln!("Error: {e}"));
        self.canvas.fill_rect(drawable_obj).map_err(|e| eprintln!("Error: {e}"));
    }

    pub fn draw_multiple<T>(&mut self, drawable_objs: &[sdl2::rect::Rect], color: Color)
    where T: Drawable
    {
        self.canvas.set_draw_color(color.rgb());
        self.canvas.fill_rects(drawable_objs).map_err(|e| eprintln!("Error: {e}"));
        // self.canvas.draw_rects(drawable_objs).map_err(|e| eprintln!("Error: {e}"));
    }
}

#[cfg(test)]
mod test
{
    #[test]
    fn test_draw()
    {
        
    }
}