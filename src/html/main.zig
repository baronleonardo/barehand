const ETree = @import("elements_tree.zig");
const std = @import("std");

const allocator = std.heap.page_allocator;

pub fn main() !void
{
    var tree = try ETree.new(allocator);
    defer tree.destroy();

    try tree.push("html", "lang=en");
    try tree.push("head", "");
    tree.pop("/head");
    try tree.push("body", "");
    try tree.push("p", "class=\"opa\"");
    try tree.push("div", "class=\"opa\"");
    tree.pop("/div");
    tree.push_text("Hello World");
    tree.pop("/p");
    tree.pop("/body");
    tree.pop("/html");

    std.log.warn("", .{});
    for (tree.tree.items) |item, index| {
        std.log.warn("{}: {}", .{index, item});
    }

    tree.print();
}