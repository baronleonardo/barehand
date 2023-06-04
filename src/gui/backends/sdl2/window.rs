use sdl2::render::Canvas;

use crate::gui::Backend;
use crate::gui::window_flags::WindowFlags;
use crate::gui::color::{self, Color};

use super::Application;
use super::application::GenericWindow;
use super::widget::WidgetType;

use std::marker::PhantomData;

pub struct Window<'a, T: Backend<'a>>
{
    canvas: Canvas<sdl2::video::Window>,
    background_color: Color,
    widgets: Vec<WidgetType<'a, T>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> PartialEq for Window<'a, T>
    where T: Backend<'a>
{
    fn eq(&self, other: &Self) -> bool {
        if self.canvas.info() == other.canvas.info()
        {
            return true
        }
        else
        {
            return false
        }
    }
}

impl<'a, T> GenericWindow<'a, T> for Window<'a, T>
    where T: Backend<'a>
{
    fn new(app: &T::Application, title: &str, width: u32, height: u32, background_color: color::Color, flags: WindowFlags) -> Window<'a, T>
    {
        let mut window_builder =
            (unsafe { std::mem::transmute::<&T::Application, &Application<T>>(app) })
            .video_subsystem
            .window(title, width, height);

        Self::init_window_flags(flags, &mut window_builder);

        let window = window_builder.build().unwrap();

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();
        canvas.set_draw_color(background_color.rgba());

        Window { canvas, background_color, widgets: vec![], phantom: PhantomData }
    }

    fn set_fullscreen(&mut self, fullscreen: bool)
    {
        self.canvas.window_mut().set_fullscreen(
            if fullscreen { sdl2::video::FullscreenType::True } else { sdl2::video::FullscreenType::Off }
        )
        .map_err(|e| eprintln!("Error: {e}"));
    }

    fn set_bordered(&mut self, bordered: bool)
    {
        self.canvas.window_mut().set_bordered(bordered);
    }

    fn set_position(&mut self, x: u32, y: u32)
    {
        use sdl2::video::WindowPos;
        self.canvas.window_mut().set_position(
            WindowPos::Positioned(x as i32),
            WindowPos::Positioned(y as i32)
        );
    }

    fn set_size(&mut self, width: u32, height: u32)
    {
        self.canvas.window_mut().set_size(width, height)
        .map_err(|e| eprintln!("Error: {e}"));
    }

    fn set_opacity(&mut self, opacity: f32)
    {
        self.canvas
            .window_mut()
            .set_opacity(opacity)
            .map_err(|e| eprintln!("Error: {e}"));
    }

    fn set_title(&mut self, new_title: &str)
    {
        self.canvas
            .window_mut()
            .set_title(new_title)
            .map_err(|e| eprintln!("Error: {e}"));
    }

    fn add_widget(&mut self, widget_type: &'a T::Widget )
    {
        // TODO: this is wrong for now
        // self.widgets.push(WidgetType::Raw(widget_type));
    }

    fn add_button(&mut self, button: &'a T::Button)
    {
        self.widgets.push(WidgetType::Button(button));
    }

    fn draw(&mut self)
    {
        self.canvas.set_draw_color(self.background_color.rgb());
        self.canvas.clear();
        // others will be here
        self.widgets.iter().for_each(|widget_type|{
            let widget_raw = widget_type.raw();

            widget_raw.iter().for_each(|rect| {
                self.canvas.set_draw_color(rect.color.rgba());
                let rect: sdl2::rect::Rect = rect.into();
                self.canvas.fill_rect(rect).map_err(|e| eprintln!("Error: {e}"));
            });
        });
        self.canvas.present();
    }
}

impl<'a, T> Window<'a, T>
    where T: Backend<'a>
{
    fn init_window_flags(flags: WindowFlags, window_builder: &mut sdl2::video::WindowBuilder)
    {
        if flags & WindowFlags::Fullscreen != WindowFlags::None {
            window_builder.fullscreen();
        };

        if flags & WindowFlags::Shown != WindowFlags::None {
            // it's shown by default
        };

        if flags & WindowFlags::Hidden != WindowFlags::None {
            window_builder.hidden();
        };

        if flags & WindowFlags::Borderless != WindowFlags::None {
            window_builder.borderless();
        };

        if flags & WindowFlags::Resizable != WindowFlags::None {
            window_builder.resizable();
        };

        if flags & WindowFlags::Minimized != WindowFlags::None {
            window_builder.minimized();
        };

        if flags & WindowFlags::Maximized != WindowFlags::None {
            window_builder.maximized();
        };
    }
}

#[cfg(test)]
mod test
{
    // use crate::gui::{window::WindowFlags, color};

    // use super::Window;
    // use super::Application;

    #[test]
    fn test_run()
    {
        // let mut app = Application::new().unwrap();
        // let mut window = Window::new(&app, "rust-sdl2 demo", 800, 600, color::Color::WHITE, WindowFlags::Default);

        // app.draw_window(&mut window);

        // app.main_loop()
    }
}