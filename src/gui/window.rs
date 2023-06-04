use super::{Backend, backends::GenericWindow, Application, Widget, Button};

pub struct Window<'a, T: Backend<'a>>
{
    pub raw: T::Window
}

impl<'a, T> GenericWindow<'a, T> for Window<'a, T>
    where T: Backend<'a>
{
    fn new(app: &T::Application, title: &str, width: u32, height: u32, background_color: super::Color, flags: super::WindowFlags) -> Self where Self: Sized {
        Self { raw: T::Window::new(app, title, width, height, background_color, flags) }
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        self.raw.set_fullscreen(fullscreen)
    }

    fn set_bordered(&mut self, bordered: bool) {
        self.raw.set_bordered(bordered)
    }

    fn set_position(&mut self, x: u32, y: u32) {
        self.raw.set_position(x, y)
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.raw.set_size(width, height)
    }

    fn set_opacity(&mut self, opacity: f32) {
        self.raw.set_opacity(opacity)
    }

    fn set_title(&mut self, new_title: &str) {
        self.raw.set_title(new_title)
    }

    fn add_widget(&mut self, widget_type: &'a T::Widget) {
        self.raw.add_widget(widget_type)
    }

    fn add_button(&mut self, button: &'a T::Button) {
        self.raw.add_button(button)
    }

    fn draw(&mut self) {
        self.raw.draw()
    }
}

impl<'a, T> Window<'a, T>
    where T: Backend<'a>
{
    pub fn new(app: &Application<'a, T>, title: &str, width: u32, height: u32, background_color: super::Color, flags: super::WindowFlags) -> Self where Self: Sized {
        Self { raw: T::Window::new(&app.raw, title, width, height, background_color, flags) }
    }

    pub fn add_widget(&mut self, widget_type: &'a Widget<'a, T>) {
        self.raw.add_widget(&widget_type.raw)
    }

    pub fn add_button(&mut self, button: &'a Button<'a, T>) {
        self.raw.add_button(&button.raw)
    }
}