use crate::gui::Position;
use crate::gui::Size;
use crate::gui::Rect;
use crate::gui::color::Color;

use super::Widget;

pub struct Button
{
    label: String,
    raw: Widget,
}

impl Button
{
    const DEFAULT_SIZE: Size = Size(150, 40);
}

impl<'a> Button
{
    pub fn new(label: String, position: Position, color: Color) -> Button
    {
        let outer_part = Rect {
            color,
            size: Self::DEFAULT_SIZE,
            position,
        };

        Button {
            label,
            raw: Widget { rectangles: vec![outer_part] },
        }
    }
}

impl<'a> Button
{
    pub fn raw(&self) -> &Widget {
        &self.raw
    }

    pub fn from_raw(raw: Widget) -> Self where Self: Sized {
        Button {
            label: "".to_string(),
            raw: Widget::from_raw(raw),
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::gui::color::Color;
    use crate::gui::window_flags::WindowFlags;
    use crate::gui::backends::sdl2::{Application, Window, WidgetType};

    use super::Button;

    #[test]
    fn test_button()
    {
        let mut app = Application::new().unwrap();
        let mut window = Window::new(&app, "test button", 800, 600, Color::WHITE, WindowFlags::Default);

        let button = Button::new("label".to_string(), (10, 10).into(), (0, 255, 0).into());
        window.add_button(&button);

        app.draw_window(&mut window);
        app.main_loop();
    }
}