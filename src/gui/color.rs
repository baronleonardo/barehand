#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Color
{
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color
{
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Color
    {
        Color { r, g, b, a: 0xff }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color
    {
        Color { r, g, b, a }
    }

    #[inline]
    pub const fn rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub const fn rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub const WHITE: Color = Color::RGBA(255, 255, 255, 255);
    pub const BLACK: Color = Color::RGBA(0, 0, 0, 255);
    pub const GRAY: Color = Color::RGBA(128, 128, 128, 255);
    pub const GREY: Color = Color::GRAY;
    pub const RED: Color = Color::RGBA(255, 0, 0, 255);
    pub const GREEN: Color = Color::RGBA(0, 255, 0, 255);
    pub const BLUE: Color = Color::RGBA(0, 0, 255, 255);
    pub const MAGENTA: Color = Color::RGBA(255, 0, 255, 255);
    pub const YELLOW: Color = Color::RGBA(255, 255, 0, 255);
    pub const CYAN: Color = Color::RGBA(0, 255, 255, 255);
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color::RGB(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color::RGBA(r, g, b, a)
    }
}