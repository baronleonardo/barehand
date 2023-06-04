#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position(pub u32, pub u32);

impl Position
{
    pub fn raw(&self) -> (u32, u32)
    {
        (self.0, self.1)
    }
}

impl From<(u32, u32)> for Position {
    fn from((x, y): (u32, u32)) -> Position {
        Position(x, y)
    }
}