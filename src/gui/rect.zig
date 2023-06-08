const position = @import("position.zig");
const Size = @import("size.zig");
const color = @import("color.zig");

pub const Rect = @This();

size: Size,
position: position.Position,
color: color.Color,

test "test rect" {}
