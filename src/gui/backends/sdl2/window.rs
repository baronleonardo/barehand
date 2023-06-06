use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::video::WindowContext;

use crate::gui::window_flags::WindowFlags;
use crate::gui::color::{self, Color};

use super::{Application, Button};
use super::widget::WidgetType;

pub struct Window<'a>
{
    canvas: Canvas<sdl2::video::Window>,
    texture_creator: TextureCreator<WindowContext>,
    background_color: Color,
    widgets: Vec<(WidgetType<'a>, Vec<&'a Texture<'a>>)>,
}

impl<'a> PartialEq for Window<'a>
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

impl<'a> Window<'a>
{
    pub fn new(app: &Application, title: &str, width: u32, height: u32, background_color: color::Color, flags: WindowFlags) -> Window<'a>
    {
        let mut window_builder = app.video_subsystem.window(title, width, height);

        Self::init_window_flags(flags, &mut window_builder);

        let window = window_builder.build().unwrap();

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();
        canvas.set_draw_color(background_color.rgba());

        let texture_creator = canvas.texture_creator();

        Window { canvas, texture_creator, background_color, widgets: vec![] }
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool)
    {
        self.canvas.window_mut().set_fullscreen(
            if fullscreen { sdl2::video::FullscreenType::True } else { sdl2::video::FullscreenType::Off }
        )
        .map_err(|e| eprintln!("Error: {e}"));
    }

    pub fn set_bordered(&mut self, bordered: bool)
    {
        self.canvas.window_mut().set_bordered(bordered);
    }

    pub fn set_position(&mut self, x: u32, y: u32)
    {
        use sdl2::video::WindowPos;
        self.canvas.window_mut().set_position(
            WindowPos::Positioned(x as i32),
            WindowPos::Positioned(y as i32)
        );
    }

    pub fn set_size(&mut self, width: u32, height: u32)
    {
        self.canvas.window_mut().set_size(width, height)
        .map_err(|e| eprintln!("Error: {e}"));
    }

    pub fn set_opacity(&mut self, opacity: f32)
    {
        self.canvas
            .window_mut()
            .set_opacity(opacity)
            .map_err(|e| eprintln!("Error: {e}"));
    }

    pub fn set_title(&mut self, new_title: &str)
    {
        self.canvas
            .window_mut()
            .set_title(new_title)
            .map_err(|e| eprintln!("Error: {e}"));
    }

    pub fn add_widget(&'a mut self, widget_type: WidgetType<'a>)
    {
        let mut textures: Vec<&Texture> = vec![];
        for surface in &widget_type.raw().surfaces
        {
            textures.push(&surface.as_texture(&self.texture_creator).unwrap());
        }

        self.widgets.push((widget_type, textures));
    }

    pub fn add_button(&'a mut self, button: &'a Button)
    {
        let mut textures: Vec<&Texture> = vec![];
        for surface in &button.raw().surfaces
        {
            textures.push(&surface.as_texture(&self.texture_creator).unwrap());
        }

        self.widgets.push((WidgetType::Button(button), textures));
    }

    pub fn draw(&mut self)
    {
        self.canvas.set_draw_color(self.background_color.rgb());
        self.canvas.clear();
        // others will be here
        self.widgets.iter().for_each(|widget|{
            let widget_raw = widget.0.raw();
            let textures = &widget.1;

            widget_raw.raw.iter().for_each(|rect| {
                self.canvas.set_draw_color(rect.color.rgba());
                let rect: sdl2::rect::Rect = rect.into();
                self.canvas.fill_rect(rect).map_err(|e| eprintln!("Error: {e}"));
            });

            textures.iter().for_each(|texture| {
                // texture.
            });
        });
        self.canvas.present();
    }
}

impl<'a> Window<'a>
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

    use super::Window;
    use super::Application;

    #[test]
    fn test_run()
    {
        // let mut app = Application::new().unwrap();
        // let mut window = Window::new(&app, "rust-sdl2 demo", 800, 600, color::Color::WHITE, WindowFlags::Default);

        // app.draw_window(&mut window);

        // app.main_loop()
    }
}