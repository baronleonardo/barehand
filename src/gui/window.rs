use bitflags::bitflags;

use super::application::Application;

use super::backends::{Backend, GenericApplication};
pub use super::color::Color;
use super::widget::{Widget, WidgetType};

use crate::gui::backends::GenericWindow;

pub use super::backends::sdl2::Window as SdlWindow;
pub struct Window<'a, T>
    where T: Backend<'a>
{
    pub backend_window: T::Window,
}

pub struct WindowSize(pub u32, pub u32);

bitflags! 
{
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct WindowFlags: u32
    {
        const None = 0;
        const Default = Self::Shown.bits() | Self::Resizable.bits();
        const Fullscreen = 2 << 1;
        // const FullscreenDesktop;
        // const Opengl;
        // const Vulkan;
        const Shown = 2 << 5;
        const Hidden = 2 << 6;
        const Borderless = 2 << 7;
        const Resizable = 2 << 8;
        const Minimized = 2 << 9;
        const Maximized = 2 << 10;
        // const InputGrabbed;
        // const InputFocus;
        // const MouseFocus;
        // const Foreign;
        // const AllowHighdpi;
        // const MouseCapture;
        // const AlwaysOnTop;
        // const SkipTaskbar;
        // const Utility;
        // const Tooltip;
        // const PopupMenu;
    }
}

impl<'a, T> Window<'a, T>
    where T: Backend<'a>
{
    pub fn new(app: &Application<'a, T>, title: &str, size: WindowSize, flags: WindowFlags) -> Window<'a, T>
    {
        Window {
            backend_window: T::Window::new(&app.backend_app, title, size.0, size.1, Color::WHITE, flags)
        }
    }

    pub fn raw(&self) -> &T::Window
    {
        &self.backend_window
    }

    pub fn raw_mut(&'a mut self) -> &'a mut T::Window
    {
        &mut self.backend_window
    }

    pub fn add_widget(&mut self, widget_type: &'a WidgetType<'a, T>)
    {
        self.backend_window.add_widget(widget_type);
    }
}

#[cfg(test)]
mod test
{
    use crate::gui::Sdl;

    use super::Window;
    // use super::SdlWindow;
    use super::WindowFlags;
    use super::WindowSize;
    use super::super::application::Application;

    #[test]
    fn test_run()
    {
        let mut app = Application::<Sdl>::new().unwrap();
        let mut window = Window::<Sdl>::new(&mut app, "rust-sdl2 demo", WindowSize(800, 600), WindowFlags::Default);
    }
}