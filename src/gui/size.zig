pub const Size = @This();

width: u32,
height: u32,

pub fn new(width: u32, height: u32) Size
{
    return Size { .width = width, .height = height };
}

pub fn raw(self: Size) .{ u32, u32 } {
    return .{ self.width, self.height };
}

