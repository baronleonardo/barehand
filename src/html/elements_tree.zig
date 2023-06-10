const std = @import("std");

const Element = @import("element.zig");

pub const ElementsTree = @This();
const Self = @This();

pub const ElementsTreeError = error {
} || std.mem.Allocator.Error;

const StackNode = struct {
    tag: []const u8,
    index: usize,
};

const TextNode = struct {
    text: []const u8,
    tag: []const u8,

    fn new() TextNode {
        return TextNode {
            .text = "",
            .tag = "",
        };
    }
};

tree: std.ArrayList(Element),
stack: std.ArrayList(StackNode),
text: TextNode,
arena: std.heap.ArenaAllocator,
arena_allocator: std.mem.Allocator,

pub fn new(allocator: std.mem.Allocator) ElementsTreeError!ElementsTree
{
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    const arena_allocator = arena.allocator();

    var tree = std.ArrayList(Element).init(arena_allocator);
    var root = Element.new("", "", "");
    root.children_index = 0;
    try tree.append(root);

    var stack = std.ArrayList(StackNode).init(allocator);
    try stack.append(.{ .tag = "", .index = 0 });

    return ElementsTree {
        .tree = tree,
        .stack = stack,
        .text = TextNode.new(),
        .arena = arena,
        .arena_allocator = arena_allocator,
    };
}

pub fn push(self: *Self, tag: []const u8, attributes: []const u8) ElementsTreeError!void
{
    var element_node = Element.new(tag, attributes, "");

    var stack_tail_index = self.stack.items[self.stack.items.len - 1].index;
    element_node.parent_index = stack_tail_index;

    try self.tree.append(element_node);

    var children_index = self.tree.items[stack_tail_index].children_index;
    if(children_index == null)
    {
        self.tree.items[stack_tail_index].children_index = self.tree.items.len - 1;
    }
    else
    {
        while(self.tree.items[children_index.?].brothers_index != null): (children_index = self.tree.items[children_index.?].brothers_index) {}
        self.tree.items[children_index.?].brothers_index = self.tree.items.len - 1;
    }

    try self.stack.append(.{.tag = tag, .index = self.tree.items.len - 1 });
}

pub fn push_text(self: *Self, text: []const u8) void
{
    var last_parent_index = self.stack.items[self.stack.items.len - 1].index;

    self.text.text = text;
    self.text.tag = self.tree.items[last_parent_index].tag;
}

pub fn pop(self: *Self, tag: []const u8) void
{   
    if(std.mem.endsWith(u8, tag[1..], self.text.tag))
    {
        var stack_last_index = self.stack.items[self.stack.items.len - 1].index;
        self.tree.items[stack_last_index].text = self.text.text;
        self.text = TextNode.new();
    }

    _ = self.stack.popOrNull();
}

pub fn print(self: *Self) void
{
    print_rec(1, 1, &self.tree);

}

fn print_rec(root: usize, spaces: usize, tree: *std.ArrayList(Element)) void
{
    std.log.warn("{} {s}", .{spaces, tree.items[root].tag});

    // print children
    var cur_child = tree.items[root].children_index;
    if(cur_child != null)
    {
        print_rec(cur_child.?, spaces + 1, tree);
    }

    // print brothers
    var cur_brother = tree.items[root].brothers_index;
    if(cur_brother != null)
    {
        print_rec(cur_brother.?, spaces, tree);
    }
}

pub fn destroy(self: *Self) void
{
    self.arena.deinit();
    self.stack.deinit();
}

test "elements_tree"
{
    const allocator = std.testing.allocator;

    var tree = try new(allocator);
    defer tree.destroy();

    try tree.push("html", "lang=en");
    try tree.push("head", "");
    tree.pop("/head");
    try tree.push("body", "");
    try tree.push("p", "class=\"opa\"");
    tree.pop("/p");
    try tree.push("p", "class=\"opa\"");
    try tree.push("div", "class=\"opa\"");
    tree.pop("/div");
    tree.push_text("Hello World");
    tree.pop("/p");
    try tree.push("p", "class=\"opa\"");
    tree.pop("/p");
    tree.pop("/body");
    tree.pop("/html");

    std.log.warn("", .{});
    for (tree.tree.items) |item, index| {
        std.log.warn("{}: {}", .{index, item});
    }

    tree.print();
}