use crate::gui::Position;
use crate::gui::Size;
use crate::gui::backends::GenericButton;
use crate::gui::backends::GenericWidget;
use crate::gui::color::Color;
use crate::gui::widget::Rect;

use super::Widget;

pub struct Button
{
    label: String,
    raw: Widget,
}

impl Button
{
    const DEFAULT_SIZE: Size = Size(100, 20);
}

impl<'a> GenericButton<'a> for Button
{
    fn new(label: String, position: Position, color: Color) -> Button
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

impl<'a> GenericWidget<'a> for Button
{
    fn raw(&self) -> &Widget {
        &self.raw
    }

    fn from_raw(raw: Widget) -> Self where Self: Sized {
        Button {
            label: "".to_string(),
            raw: Widget::from_raw(raw),
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::gui::backends::GenericApplication;
    use crate::gui::color::Color;
    use crate::gui::window::WindowFlags;
    use crate::gui::backends::GenericWindow;
    use crate::gui::backends::sdl2::{Application, Window};

    use super::Button;

    #[test]
    fn test_button()
    {
        // let mut app = Application::new().unwrap();
        // let mut window = Window::new(&app, "test button", 800, 600, Color::WHITE, WindowFlags::Default);

        // let button = Button::new("label".to_string(), (10, 10).into(), (0, 255, 0).into());
        // window.add_widget(&button);

        // app.draw_window(&mut window);
        // app.main_loop();
    }
}