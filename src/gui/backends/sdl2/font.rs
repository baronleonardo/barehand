use std::{alloc::Layout, collections::HashMap};

// use once_cell::sync::OnceCell;

use crate::gui::Color;

// static mut TTF_CONTEXT: OnceCell<sdl2::ttf::Sdl2TtfContext> = OnceCell::new();

pub struct Font<'a>
{
    pub name: &'a str,
    pub raw: sdl2::ttf::Font<'a, 'a>,
    pub color: Color,
}

pub struct FontManager<'a>
{
    layout: Layout,
    ptr: *mut u8,
    fonts: HashMap<&'a str, Font<'a>>
}

impl<'a> FontManager<'a>
{
    pub fn init() -> FontManager<'a>
    {
        // let f = sdl2::ttf::init().unwrap().load_font("path", 12).unwrap();
        unsafe {
            let layout = std::alloc::Layout::new::<sdl2::ttf::Sdl2TtfContext>();
            let ptr = std::alloc::alloc(layout);

            FontManager { layout: layout, ptr, fonts: Default::default() }
        }
    }

    // pub fn init() -> Result<(), String>
    // {
    //     unsafe
    //     {
    //         TTF_CONTEXT.get_or_init(|| {
    //             sdl2::ttf::init().map_err(|e| { println!("{e:?}"); e.to_string() }).unwrap()
    //         });
    //     }

    //     Ok(())
    // }

    pub fn load(&mut self, name: &'a str, path: &'a str, size: u16, color: Color) -> Result<&Font, String>
    {
        unsafe {
            // let mut layout = std::alloc::Layout::new::<sdl2::ttf::Sdl2TtfContext>();
            // let a = std::alloc::alloc(layout);

            let font = Font {
                name: name.clone(),
                raw: (*(self.ptr as *mut sdl2::ttf::Sdl2TtfContext)).load_font(path, size)?,
                // raw: TTF_CONTEXT.get_or_init(|| {
                //     sdl2::ttf::init().map_err(|e| { println!("{e:?}"); e.to_string() }).unwrap()
                // }).load_font(path, size)?,
                color
            };

            self.fonts.insert(name, font);

            Ok(self.fonts.get(name).unwrap())
        }
    }

    pub fn unload(&mut self, name: &str) -> bool
    {
        match self.fonts.remove(name)
        {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get(&self, name: &str) -> Option<&Font>
    {
        self.fonts.get(name)
    }

    // pub fn get_mut(&mut self, name: &str) -> Option<&mut Font>
    // {
    //     self.fonts.get_mut(name)
    // }

    // pub fn deinit(mut self)
    // {
    //     // unsafe { drop(TTF_CONTEXT.take().unwrap()); }
    //     unsafe {
    //         std::alloc::dealloc(self.ptr, self.layout);
    //     }
    // }
}

impl<'a> Drop for FontManager<'a>
{
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(self.ptr, self.layout);
        }
    }
}