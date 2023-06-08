const sdl2 = @import("sdl2.zig");

const std = @import("std");

const Color = @import("../../color.zig");

const FontError = error {
    OPEN_FONT_ERROR,
    CREATE_TEXT_SURFACE_ERROR,
};

pub const TextSurface = struct {
    raw: ?*sdl2.SDL_Surface,
    rect: sdl2.SDL_Rect,
};

pub const Font = struct {
    raw: *sdl2.TTF_Font,
    color: Color,

    const Self = @This();

    pub fn new(path: []const u8, size: u16, color: Color) FontError!Font
    {
        var sdl_font: ?*sdl2.TTF_Font  = sdl2.TTF_OpenFont(@ptrCast([*c]const u8, path), @intCast(c_int, size));

        if(sdl_font == null)
        {
            std.log.err("TTF_OpenFont Error: {s}", .{sdl2.SDL_GetError()});
            return FontError.OPEN_FONT_ERROR;
        }

        return Font { .raw = sdl_font.?, .color = color };
    }

    pub fn draw_to_surface(self: *Self, text: []const u8) FontError!TextSurface
    {
        var surface: ?*sdl2.SDL_Surface = sdl2.TTF_RenderUTF8_Blended(
            self.raw,
            @ptrCast([*c]const u8,text),
            sdl2.SDL_Color { .r = self.color.r, .g = self.color.g, .b = self.color.b, .a = self.color.a }
        );

        if(surface == null)
        {
            std.log.err("TTF_RenderUTF8_Blended Error: {s}", .{sdl2.SDL_GetError()});
            return FontError.CREATE_TEXT_SURFACE_ERROR;
        }

        var width: c_int = undefined;
        var height: c_int = undefined;
        _ = sdl2.TTF_SizeUTF8(self.raw, @ptrCast([*c]const u8, text), &width, &height);

        var rect = sdl2.SDL_Rect { .x = 0, .y = 0, .w = width, .h = height };

        return TextSurface { .raw = surface, .rect = rect };
    }

    pub fn destroy(self: *Self) void
    {
        sdl2.TTF_CloseFont(self.raw);
    }

};

test "font"
{

}