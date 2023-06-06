use std::{time::Duration, collections::HashMap, iter::Once, ops::DerefMut};

use sdl2::{event::Event};
use once_cell::sync::OnceCell;

use crate::gui::Color;

use super::{Window, Font};

// pub use crate::gui::backends::GenericApplication;
// pub use crate::gui::backends::GenericWindow;

// static mut TTF_CONTEXT: ManuallyDrop<sdl2::ttf::Sdl2TtfContext> = ManuallyDrop::new(sdl2::ttf::Sdl2TtfContext {});

pub struct Application<'a>
{
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub event_pump: sdl2::EventPump,
    pub windows: Vec<&'a mut super::Window<'a>>,
    // pub font_manager: FontManager<'a>,
    // pub fonts: HashMap<&'a str, &'a Font<'a>>,
    // pub font: &'a Font<'a>,
    // pub fonts: HashMap<String, Font<'a>>,
    // pub ttf_context: OnceCell<sdl2::ttf::Sdl2TtfContext>,
    // pub ttf_context: ManuallyDrop<sdl2::ttf::Sdl2TtfContext>,
}

impl<'a> Application<'a>
{
    pub fn new() -> Result<Application<'a>, String>
    {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump()?;
        let video_subsystem = sdl_context.video()?;
        // Font::init()?;
        // let ttf_context = ManuallyDrop::new(sdl2::ttf::init().map_err(|e| e.to_string())?);

        Ok(Application {
            sdl_context,
            video_subsystem,
            event_pump,
            windows: vec![],
            // font,
            // font_manager: FontManager::init(),
            // fonts: HashMap::new(),
            // ttf_context: OnceCell::new(),
            // ttf_context,
        })

        // res.ttf_context.set(sdl2::ttf::init().map_err(|e| e.to_string())?).map_err(|_| "Error".to_string())?;

        // let font = res.ttf_context.get().unwrap().load_font("src/gui/assets/Roboto/Roboto-Regular.ttf", 12)?;

        // let font = res.font_manager.load("name".to_string(), "src/gui/assets/Roboto/Roboto-Regular.ttf", 12, Color::BLACK)?;

        // res.fonts.insert(font.name.as_str(), &font);
        // res.fonts.insert("font.name".to_string(), font);

        // Ok(res)
    }

    // pub fn get_font(&self) -> &'a Font
    // {
    //     self.font
    // }

    // pub fn get_font_mut(&'a mut self, font_name: &str) -> Option<&mut &'a Font<'a>>
    // {
    //     self.fonts.get_mut(font_name).map(|font| font)
    // }

    pub fn draw_window(&mut self, window: &'a mut Window<'a>)
    {
        self.windows.push(window);
    }

    pub fn main_loop(&mut self)
    {
        // self.canvas.clear();
        // self.canvas.present();
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} /*|
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. }*/ => {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
            self.windows.iter_mut().for_each(|window| window.draw());

            // // put new color here
            // self.canvas.clear();
            // self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

// impl<'a> Drop for Application<'a>
// {
//     fn drop(&mut self) {
//         drop(self.windows);
//         Font::deinit();
//     }
// }