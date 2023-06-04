use super::Size;
use super::Position;
use super::Color;

#[derive(Debug)]
pub struct Rect
{
    pub size: Size,
    pub position: Position,
    pub color: Color,
}