#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Size(pub u32, pub u32);

impl Size
{
    pub fn raw(&self) -> (u32, u32)
    {
        (self.0, self.1)
    }
}

impl From<(u32, u32)> for Size {
    fn from((w, h): (u32, u32)) -> Size {
        Size(w, h)
    }
}