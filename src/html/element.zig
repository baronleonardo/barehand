const std = @import("std");

pub const Element = @This();

tag: []const u8,
attributes: []const u8,
text: []const u8,
parent_index: ?usize,
children_index: ?usize,
brothers_index: ?usize,

pub fn new(tag: []const u8, attributes: []const u8, text: []const u8) Element
{
    return Element {
        .tag = tag,
        .attributes = attributes,
        .text = text,
        .parent_index = null,
        .children_index = null,
        .brothers_index = null,
    };
}

// pub fn format(
//     self: Element,
//     comptime fmt: []const u8,
//     options: std.fmt.FormatOptions,
//     writer: anytype,
// ) !void
// {
//     _ = fmt;
//     _ = options;

//     try writer.print("element{ .tag = {s}, .attributes = {s}, .text = {s}, .parent_index = {d}, children_index = {d}, .brothers_index = {d}",
//         .{self.tag, self.attributes, self.text, self.parent_index, self.children_index, self.brothers_index}
//     );

//     try writer.writeAll(" }");
// }

test "element"
{

}