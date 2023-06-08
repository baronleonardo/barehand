const std = @import("std");

pub const Color = @This();

r: u8,
g: u8,
b: u8,
a: u8,

pub fn RGB(r: u8, g: u8, b: u8) Color {
    return Color{ .r = r, .g = g, .b = b, .a = 0xff };
}

pub fn RGBA(r: u8, g: u8, b: u8, a: u8) Color {
    return Color{ .r = r, .g = g, .b = b, .a = a };
}

pub fn rgb(self: Color) .{ u8, u8, u8 } {
    return .{ self.r, self.g, self.b };
}

pub fn rgba(self: Color) .{ u8, u8, u8, u8 } {
    return .{ self.r, self.g, self.b, self.a };
}

pub const WHITE: Color = Color.RGBA(255, 255, 255, 255);
pub const BLACK: Color = Color.RGBA(0, 0, 0, 255);
pub const GRAY: Color = Color.RGBA(128, 128, 128, 255);
pub const GREY: Color = Color.GRAY;
pub const RED: Color = Color.RGBA(255, 0, 0, 255);
pub const GREEN: Color = Color.RGBA(0, 255, 0, 255);
pub const BLUE: Color = Color.RGBA(0, 0, 255, 255);
pub const MAGENTA: Color = Color.RGBA(255, 0, 255, 255);
pub const YELLOW: Color = Color.RGBA(255, 255, 0, 255);
pub const CYAN: Color = Color.RGBA(0, 255, 255, 255);

test "simple test" {
    var color = Color.RGB(255, 255, 255);
    _ = color;
    std.debug.print("Color: {}\n", .{Color.WHITE});
}
