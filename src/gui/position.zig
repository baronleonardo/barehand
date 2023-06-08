pub const Position = @This();

x: u32,
y: u32,

pub fn new(x: u32, y: u32) Position
{
    return Position { .x = x, .y = y };
}

pub fn raw(self: Position) .{ u32, u32 } {
    return .{ self.x, self.y };
}
