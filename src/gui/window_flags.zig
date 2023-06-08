pub const WindowFlags = packed struct(u32) {
    None: bool = false,
    Fullscreen: bool = false,
    // FullscreenDesktop,
    // Opengl,
    // Vulkan,
    Shown: bool = false,
    Hidden: bool = false,
    Borderless: bool = false,
    Resizable: bool = false,
    Minimized: bool = false,
    Maximized: bool = false,
    // InputGrabbed,
    // InputFocus,
    // MouseFocus,
    // Foreign,
    // AllowHighdpi,
    // MouseCapture,
    // AlwaysOnTop,
    // SkipTaskbar,
    // Utility,
    // Tooltip,
    // PopupMenu,
    _padding: u24 = 0,

    const Self = @This();
    pub fn default() Self {
        return WindowFlags { .Shown = true, .Resizable = true } ;
    }

    pub fn as_u32(self: Self) u32
    {
        return @bitCast(u32, self);
    }
};

const std = @import("std");
test "window flags" {
    std.debug.print("Enum: 0x{x}\n", .{WindowFlags.default().as_u32()});
}
