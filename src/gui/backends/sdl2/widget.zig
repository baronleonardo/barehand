const sdl2 = @import("sdl2.zig");

const std = @import("std");

const Rect = @import("../../rect.zig");
const button_mod = @import("button.zig");
const font_mod = @import("font.zig");
const window_mod = @import("window.zig");

pub const WidgetType = union(enum)
{
    Raw: *Widget,
    Button: *button_mod.Button,

    const Self = @This();

    pub fn raw(self: *Self) *Widget
    {
        return switch (self.*) {
            inline else => |case| case.raw(),
        };
    }
};

pub const WidgetTexture = struct {
    raw: ?*sdl2.SDL_Texture,
    rect: sdl2.SDL_Rect,
};

pub const WidgetError = error {
    TEXTURE_CREATION_FAILED
};

pub const Widget = struct {
    raw_rects: std.ArrayList(Rect),
    surfaces: std.ArrayList(font_mod.TextSurface),

    const Self = @This();

    pub fn new(rectangles: std.ArrayList(Rect), surfaces: std.ArrayList(font_mod.TextSurface)) Widget
    {
        return Widget {
            .raw_rects = rectangles,
            .surfaces = surfaces,
        };
    }

    pub fn to_textures(self: *Self, window: *window_mod.Window, allocator: std.mem.Allocator) WidgetError!std.ArrayList(WidgetTexture)
    {
        var textures = std.ArrayList(WidgetTexture).init(allocator);

        for (self.surfaces.items) |surface| {
            var texture: ?*sdl2.SDL_Texture = sdl2.SDL_CreateTextureFromSurface(window.renderer, surface.raw);
            if(texture == null)
            {
                std.log.err("SDL_CreateTextureFromSurface Error: {s}\n", .{sdl2.SDL_GetError()});
                return WidgetError.TEXTURE_CREATION_FAILED;
            }

            textures.append(WidgetTexture {.raw = texture, .rect = surface.rect}) catch unreachable;
        }

        return textures;
    }

    pub fn raw(self: *Self) *Widget
    {
        return self;
    }

    pub fn from_raw(widget: Widget) Widget
    {
        return widget;
    }

    pub fn destroy(self: *Self) void
    {
        self.raw_rects.deinit();
        self.surfaces.deinit();
    }
};

test "test widget"
{
}