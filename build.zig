const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();

    const lib = b.addStaticLibrary("barehand", "src/main.zig");
    lib.setBuildMode(mode);
    lib.linkLibC();
    lib.addSystemIncludePath("SDL2");
    lib.linkSystemLibraryNeeded("SDL2");
    lib.addSystemIncludePath("SDL2_ttf");
    lib.linkSystemLibraryNeeded("SDL2_ttf");
    lib.install();

    const main_tests = b.addTest("src/gui/backends/sdl2/application.zig");
    main_tests.setBuildMode(mode);
    main_tests.setMainPkgPath(".");
    main_tests.linkLibC();
    main_tests.addSystemIncludePath("SDL2");
    main_tests.linkSystemLibraryNeeded("SDL2");
    main_tests.addSystemIncludePath("SDL2_ttf");
    main_tests.linkSystemLibraryNeeded("SDL2_ttf");

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);
}
