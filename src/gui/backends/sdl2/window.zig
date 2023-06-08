const sdl2 = @import("sdl2.zig");

const std = @import("std");

const Color = @import("../../color.zig");
const WindowFlags = @import("../../window_flags.zig").WindowFlags;
const widget_mod = @import("widget.zig");
const Button = @import("button.zig");

pub const WindowError = error{
    WINDOW_CREATION_FAILED,
    RENDERER_CREATION_FAILED,
};

const WindowWidget = struct 
{
    widget: widget_mod.WidgetType,
    textures: std.ArrayList(widget_mod.WidgetTexture)
};

pub const Window = struct {
    const Self = @This();

    raw: *sdl2.SDL_Window,
    renderer: *sdl2.SDL_Renderer,
    background_color: Color,
    widgets: std.ArrayList(WindowWidget),
    allocator: *const std.mem.Allocator,

    pub fn new(title: []const u8, width: u32, height: u32, background_color: Color, flags: WindowFlags, allocator: *const std.mem.Allocator) WindowError!Window {

        // window
        var win: ?*sdl2.SDL_Window = sdl2.SDL_CreateWindow(@ptrCast([*c]const u8, title), @intCast(c_int, 100), @intCast(c_int, 100), @intCast(c_int, width), @intCast(c_int, height), flags.as_u32());

        if (win == null) {
            std.log.err("SDL_CreateWindow Error: {s}\n", .{sdl2.SDL_GetError()});
            return WindowError.WINDOW_CREATION_FAILED;
        }

        // render
        var ren: ?*sdl2.SDL_Renderer = sdl2.SDL_CreateRenderer(win, -1, sdl2.SDL_RENDERER_ACCELERATED | sdl2.SDL_RENDERER_PRESENTVSYNC);
        if (ren == null) {
            std.log.err("SDL_CreateRenderer Error: {s}\n", .{sdl2.SDL_GetError()});
            return WindowError.RENDERER_CREATION_FAILED;
        }

        return Window {
            .raw = win.?,
            .renderer = ren.?,
            .background_color = background_color,
            .widgets = std.ArrayList(WindowWidget).init(allocator.*),
            .allocator = allocator,
        };
    }

    pub fn add_widget(self: *Self, widget_type: *widget_mod.WidgetType) !void
    {
        var textures = widget_type.raw().to_textures(self, self.allocator);
        self.widgets.widgets.append(widget_type) catch unreachable;
        self.widgets.textures.?.append(textures) catch unreachable;
    }

    pub fn add_button(self: *Self, button: *Button) void
    {
        var widget_type = widget_mod.WidgetType { .Button = button };
        var textures = widget_type.raw().to_textures(self, self.allocator.*) catch unreachable;
        self.widgets.append( WindowWidget { .widget = widget_type, .textures = textures }) catch unreachable;
    }

    pub fn draw(self: Self) void {
        _ = sdl2.SDL_SetRenderDrawColor(self.renderer, self.background_color.r, self.background_color.g, self.background_color.b, self.background_color.a);
        _ = sdl2.SDL_RenderClear(self.renderer);

        // others go here
        for (self.widgets.items) |*window_widget| {
            switch (window_widget.widget) {
                inline else => |widget| {
                    for (widget.raw().raw_rects.items) |rectangle| {
                        _ = sdl2.SDL_SetRenderDrawColor(
                            self.renderer,
                            rectangle.color.r,
                            rectangle.color.g,
                            rectangle.color.b,
                            rectangle.color.a
                        );

                        var rect = sdl2.SDL_Rect { 
                            .x = @intCast(c_int, rectangle.position.x),
                            .y = @intCast(c_int, rectangle.position.y),
                            .w = @intCast(c_int, rectangle.size.width),
                            .h = @intCast(c_int, rectangle.size.height),
                        };

                        _ = sdl2.SDL_RenderFillRect(self.renderer, &rect);

                        for (window_widget.textures.items) |texture|
                        {
                            _ = sdl2.SDL_RenderCopy(self.renderer, texture.raw, null, &texture.rect);
                        }
                    }
                },
            }
        }

        sdl2.SDL_RenderPresent(self.renderer);
    }

    pub fn destroy(self: Self) void {
        sdl2.SDL_DestroyRenderer(self.renderer);
        sdl2.SDL_DestroyWindow(self.raw);

        for (self.widgets.items) |widget| {
            widget.textures.deinit();
        }
        self.widgets.deinit();
    }
};

test "test window" {
    const allocator = std.testing.allocator;

    var win = try Window.new("This is a test", 800, 600, Color.YELLOW, WindowFlags { .Shown = true }, &allocator);
    win.destroy();
}
