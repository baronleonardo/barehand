mod sdl2;
pub use self::sdl2::Application as SdlApplication;
pub use self::sdl2::Window as SdlWindow;
pub use self::sdl2::Widget as SdlWidget;
pub use self::sdl2::Button as SdlButton;

use crate::gui::WindowFlags;
use super::Color;
use super::Position;
use super::Rect;

pub trait Backend<'a>
    where Self: Sized
{
    type Application: GenericApplication<'a, Self>;
    type Window: GenericWindow<'a, Self>;
    type Widget: GenericWidget<'a>;
    type Button: GenericButton<'a, Self>;
}

pub trait GenericApplication<'a, T: Backend<'a>>
{
    fn new() -> Result<Self, String> where Self: Sized;
    fn draw_window(&mut self, window: &'a mut T::Window);
    fn main_loop(&mut self);
}

pub trait GenericWindow<'a, T: Backend<'a>>
{
    fn new(app: &T::Application, title: &str, width: u32, height: u32, background_color: Color, flags: WindowFlags) -> Self where Self: Sized;
    fn set_fullscreen(&mut self, fullscreen: bool);
    fn set_bordered(&mut self, bordered: bool);
    fn set_position(&mut self, x: u32, y: u32);
    fn set_size(&mut self, width: u32, height: u32);
    fn set_opacity(&mut self, opacity: f32);
    fn set_title(&mut self, new_title: &str);
    fn add_widget(&mut self, widget_type: &'a T::Widget);
    fn add_button(&mut self, button: &'a T::Button);
    fn draw(&mut self);
}

pub trait GenericWidget<'a>
{
    fn new() -> Self where Self: Sized;
    fn from_raw(raw: Vec<Rect>) -> Self where Self: Sized;
    fn raw(&self) -> &Vec<Rect>;
}

pub trait GenericButton<'a, T: Backend<'a>>
{
    fn new(label: String, position: Position, color: Color) -> Self;
    fn raw(&self) -> &T::Widget;
}