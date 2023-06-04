use std::time::Duration;

use sdl2::event::Event;

pub use crate::gui::backends::GenericApplication;
pub use crate::gui::backends::GenericWindow;

use super::Window;

pub struct Application<'a>
{
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub event_pump: sdl2::EventPump,
    pub windows: Vec<&'a mut super::Window<'a>>,
}

impl<'a> GenericApplication<'a> for Application<'a>
{
    // type WindowType = Window;

    fn new() -> Result<Application<'a>, String>
    {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump()?;
        let video_subsystem = sdl_context.video()?;

        Ok(Application {
            sdl_context,
            video_subsystem,
            event_pump,
            windows: vec![],
        })
    }

    fn draw_window<W: GenericWindow<'a>>(&mut self, window: &'a mut W)
    {
        self.windows.push(unsafe { std::mem::transmute(window) });
    }

    fn main_loop(&mut self)
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