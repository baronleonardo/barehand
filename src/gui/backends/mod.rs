mod sdl2;
pub use self::sdl2::Application as SdlApplication;
pub use self::sdl2::Window as SdlWindow;
pub use self::sdl2::Widget as SdlWidget;
pub use self::sdl2::Button as SdlButton;
pub use self::sdl2::Font as SdlFont;
pub use self::sdl2::FontManager as SdlFontManager;

pub trait Backend<'a>
{
    type Application;
    type Window;
    type Widget;
    type Button;
    type FontManager;
    type Font;
}