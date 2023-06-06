mod window_flags;
pub use window_flags::WindowFlags;

mod color;
pub use color::Color;

mod position;
pub use position::Position;

mod size;
pub use size::Size;

mod rect;
pub use rect::Rect;

pub mod backends;
pub use backends::Backend;

pub struct Sdl;
impl<'a> Backend<'a> for Sdl {
    type Application = backends::SdlApplication<'a>;
    type Window = backends::SdlWindow<'a>;
    type Widget = backends::SdlWidget<'a>;
    type Button = backends::SdlButton<'a>;
    type FontManager = backends::SdlFontManager<'a>;
    type Font = backends::SdlFont<'a>;
}

#[cfg(test)]
mod test
{
    use super::{Sdl, Backend, Color, WindowFlags};

    #[test]
    fn test_button()
    {
        let mut font_manager = <Sdl as Backend>::FontManager::init();
        let font = font_manager.load(
            "Roboto-Regular",
            "src/gui/assets/Roboto/Roboto-Regular.ttf",
            14,
            Color::BLACK
        ).unwrap();

        let mut app = <Sdl as Backend>::Application::new().unwrap();
        let mut window = <Sdl as Backend>::Window::new(&app, "title", 800, 600, Color::WHITE, WindowFlags::Default);
        let button = <Sdl as Backend>::Button::new("button".into(), (50, 50).into(), (255, 0, 0).into(), &font);
        window.add_button(&button);

        app.draw_window(&mut window);
        app.main_loop();
    }
}