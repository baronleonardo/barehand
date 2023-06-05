mod sdl2;
pub use self::sdl2::Application as SdlApplication;
pub use self::sdl2::Window as SdlWindow;
pub use self::sdl2::Widget as SdlWidget;
pub use self::sdl2::Button as SdlButton;

pub trait Backend<'a>
{
    type Application;
    type Window;
    type Widget;
    type Button;
}

// pub trait Backend<'a> {
//     type Application: GenericApplication<'a>;
//     type Window: GenericWindow<'a>;
//     type Widget: GenericWidget<'a>;
//     type Button: GenericWidget<'a> + GenericButton<'a>;
// }

// pub trait GenericApplication<'a>
// {
//     fn new() -> Result<Self, String> where Self: Sized;
//     fn draw_window<W: GenericWindow<'a>>(&mut self, window: &'a mut W);
//     fn main_loop(&mut self);
// }

// pub trait GenericWindow<'a>
// {
//     fn new<A: GenericApplication<'a>>(app: &A, title: &str, width: u32, height: u32, background_color: Color, flags: WindowFlags) -> Self where Self: Sized;
//     fn set_fullscreen(&mut self, fullscreen: bool);
//     fn set_bordered(&mut self, bordered: bool);
//     fn set_position(&mut self, x: u32, y: u32);
//     fn set_size(&mut self, width: u32, height: u32);
//     fn set_opacity(&mut self, opacity: f32);
//     fn set_title(&mut self, new_title: &str);
//     fn add_widget(&mut self, widget_type: &'a WidgetType);
//     fn draw(&mut self);
// }

// pub trait GenericWidget<'a>
// {
//     fn from_raw(raw: Widget) -> Self where Self: Sized;
//     fn raw(&self) -> &Widget;
// }

// pub trait GenericButton<'a>
// {
//     fn new(label: String, position: Position, color: Color) -> Self;
// }