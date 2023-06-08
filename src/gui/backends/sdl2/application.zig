const sdl2 = @import("sdl2.zig");

const std = @import("std");

const window_mod = @import("window.zig");
const Button = @import("button.zig");
const font_mod = @import("font.zig");
const Color = @import("../../color.zig");
const Position = @import("../../position.zig");
const WindowFlags = @import("../../window_flags.zig").WindowFlags;

const allocator = std.testing.allocator;

const ApplicationError = error{
    SDL_INIT_FAILED,
    FONT_INIT_FAILED,
};

const Application = struct {
    const Self = @This();

    windows: std.ArrayList(*window_mod.Window),

    pub fn new() ApplicationError!Application {
        if (sdl2.SDL_Init(sdl2.SDL_INIT_EVERYTHING) != 0) {
            std.log.err("SDL_Init Error: {s}", .{sdl2.SDL_GetError()});
            return ApplicationError.SDL_INIT_FAILED;
        }

        if( sdl2.TTF_Init() != 0 ) {
            std.log.err("TTF_Init Error: {s}", .{sdl2.SDL_GetError()});
            return ApplicationError.FONT_INIT_FAILED;
        }

        return Application{ .windows = std.ArrayList(*window_mod.Window).init(allocator) };
    }

    pub fn add_window(self: *Self, window: *window_mod.Window) void {
        self.windows.append(window) catch undefined;
    }

    pub fn main_loop(self: *Self) void {
        var close: bool = false;

        while (!close) {
            var event: sdl2.SDL_Event = undefined;

            while (sdl2.SDL_PollEvent(&event) != 0) {
                switch (event.type) {
                    sdl2.SDL_QUIT => close = true,
                    else => {},
                }

                for (self.windows.items) |window| {
                    window.*.draw();
                }
                std.time.sleep(1_000_000_000 / 60);
            }
        }
    }

    pub fn destroy(self: Self) void {
        sdl2.SDL_Quit();
        sdl2.TTF_Quit();
        self.windows.deinit();
    }
};

test "testing application" {
    var app = try Application.new();
    defer app.destroy();

    var window = try window_mod.Window.new("hello dear", 800, 600, Color.WHITE, WindowFlags { .Shown = true }, &allocator);
    defer window.destroy();

    var font = font_mod.Font.new("src/gui/assets/Roboto/Roboto-Regular.ttf", 16, Color.BLACK) catch unreachable;
    defer font.destroy();

    var button = Button.new("button", Position.new(50, 50), Color.RED, &font, allocator);
    defer button.destroy();

    window.add_button(&button);

    app.add_window(&window);
    app.main_loop();
}
