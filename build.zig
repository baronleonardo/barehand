const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();

    // const lib = b.addStaticLibrary("barehand", "src/main.zig");
    // lib.setBuildMode(mode);
    // lib.linkLibC();
    // lib.addSystemIncludePath("SDL2");
    // lib.linkSystemLibraryNeeded("SDL2");
    // lib.addSystemIncludePath("SDL2_ttf");
    // lib.linkSystemLibraryNeeded("SDL2_ttf");
    // lib.install();

    const gui_tests = b.addTest("src/gui/backends/sdl2/application.zig");
    gui_tests.setBuildMode(mode);
    gui_tests.setMainPkgPath(".");
    gui_tests.linkLibC();
    gui_tests.addSystemIncludePath("SDL2");
    gui_tests.linkSystemLibraryNeeded("SDL2");
    gui_tests.addSystemIncludePath("SDL2_ttf");
    gui_tests.linkSystemLibraryNeeded("SDL2_ttf");

    const gui_test_step = b.step("test_gui", "Run GUI library tests");
    gui_test_step.dependOn(&gui_tests.step);

    const html_tests = b.addTest("src/html/html.zig");
    html_tests.setBuildMode(mode);
    html_tests.setMainPkgPath(".");
    // html_tests.linkLibC();

    const html_test_step = b.step("test_html", "Run HTML library tests");
    html_test_step.dependOn(&html_tests.step);
}
