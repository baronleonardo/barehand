use super::backends::GenericApplication;
use super::{backends::Backend, window::Window};

pub use super::backends::sdl2::Application as SdlApplication;
pub struct Application<'a, T: Backend<'a>>
{
    pub backend_app: T::Application,
}

impl<'a, T> Application<'a, T>
    where T: Backend<'a>
{
    pub fn new() -> Result<Application<'a, T>, String>
    {
        Ok(Application {
            backend_app: T::Application::new()?
        })
    }

    pub fn draw_window(&mut self, window: &'a mut Window<'a, T>)
    {
        self.backend_app.draw_window(window.raw_mut());
    }

    pub fn main_loop(&mut self)
    {
        self.backend_app.main_loop();
    }
}