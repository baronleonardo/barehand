use super::{Backend, backends::GenericApplication, Window};
pub struct Application<'a, T: Backend<'a>> {
    pub raw: T::Application,
}

impl<'a, T> GenericApplication<'a, T> for Application<'a, T>
    where T: Backend<'a>
{
    fn new() -> Result<Self, String> where Self: Sized {
        Ok(Self { 
            raw: T::Application::new()?
        })
    }

    fn draw_window(&mut self, window: &'a mut T::Window) {
        self.raw.draw_window(window);
    }

    fn main_loop(&mut self) {
        self.raw.main_loop();
    }
}

impl<'a, T> Application<'a, T>
    where T: Backend<'a>
{
    pub fn draw_window(&mut self, window: &'a mut Window<'a, T>) {
        self.raw.draw_window(&mut window.raw);
    }
}