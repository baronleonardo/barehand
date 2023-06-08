const sdl2 = @import("sdl2.zig");

const std = @import("std");

const widget_mod = @import("widget.zig");
const font_mod = @import("font.zig");
const Size = @import("../../size.zig");
const Position = @import("../../position.zig");
const Rect = @import("../../rect.zig");
const Color = @import("../../color.zig");

pub const Button = @This();

label: []const u8,
raw_widget: widget_mod.Widget,

const DEFAULT_SIZE = Size { .width = 150, .height = 40 };
const Self = @This();

pub fn new(label: []const u8, position: Position, color: Color, font: *font_mod.Font, allocator: std.mem.Allocator) Button
{
    var outer_part = Rect{ .color = color, .size = DEFAULT_SIZE, .position = position };

    var rects = std.ArrayList(Rect).init(allocator);
    rects.append(outer_part) catch unreachable;

    var surface = font.draw_to_surface(label) catch unreachable;
    surface.rect.x = @intCast(c_int, position.x + (outer_part.size.width - @intCast(u32, surface.rect.w)) / 2);
    surface.rect.y = @intCast(c_int, position.y + (outer_part.size.height - @intCast(u32, surface.rect.h)) / 2);

    var surfaces = std.ArrayList(font_mod.TextSurface).init(allocator);
    surfaces.append(surface) catch unreachable;

    return Button {
        .label = label,
        .raw_widget = widget_mod.Widget.new(rects, surfaces)
    };
}

pub fn raw(self: *Self) *widget_mod.Widget
{
    return &self.*.raw_widget;
}

pub fn destroy(self: *Self) void
{
    self.raw_widget.destroy();
}

test "test button"
{

}