use crate::gui::Backend;
use crate::gui::Position;
use crate::gui::Size;
use crate::gui::Rect;
use crate::gui::backends::GenericButton;
use crate::gui::backends::GenericWidget;
use crate::gui::color::Color;

use super::Widget;

pub struct Button<'a, T: Backend<'a>>
{
    label: String,
    raw: T::Widget,
}

impl<'a, T> Button<'a, T>
    where T: Backend<'a>
{
    const DEFAULT_SIZE: Size = Size(150, 40);
}

impl<'a, T> GenericButton<'a, T> for Button<'a, T>
    where T: Backend<'a>
{
    fn new(label: String, position: Position, color: Color) -> Button<'a, T>
    {
        let outer_part = Rect {
            color,
            size: Self::DEFAULT_SIZE,
            position,
        };

        Button {
            label,
            raw: T::Widget::from_raw(vec![outer_part]),
        }
    }

    fn raw(&self) -> &T::Widget
    {
        &self.raw
    }
}

// impl<'a, T> GenericWidget<'a, T> for Button<'a, T>
//     where T: Backend<'a>
// {
//     fn new() -> Self where Self: Sized {

//     }

//     fn raw(&self) -> &T::Widget {
//         &self.raw
//     }

//     fn from_raw(raw: T::Widget) -> Self where Self: Sized {
//         Button {
//             label: "".to_string(),
//             raw: T::Widget::from_raw(raw),
//         }
//     }
// }

#[cfg(test)]
mod test
{
    // use crate::gui::color::Color;
    // use crate::gui::window_flags::WindowFlags;
    // use crate::gui::backends::sdl2::{Application, Window, WidgetType};

    // use super::Button;

    // #[test]
    // fn test_button()
    // {
    //     let mut app = Application::new().unwrap();
    //     let mut window = Window::new(&app, "test button", 800, 600, Color::WHITE, WindowFlags::Default);

    //     let button = Button::new("label".to_string(), (10, 10).into(), (0, 255, 0).into());
    //     window.add_button(&button);

    //     app.draw_window(&mut window);
    //     app.main_loop();
    // }
}