const std = @import("std");

const TagNode = struct {
    str: [] const u8,
    next: ?*const TagNode,
};

const SingltonTags = [_]?TagNode {
    .{ .str = "area", .next = null },
    .{ .str = "base", .next = &.{ .str = "br", .next = null } },
    .{ .str = "col", .next = &.{ .str = "command", .next = null } },
    null,
    .{ .str = "embed", .next = null },
    null,
    null,
    .{ .str = "hr", .next = null },
    .{ .str = "img", .next = &.{ .str = "input", .next = null } },
    null,
    .{ .str = "keygen", .next = null },
    .{ .str = "link", .next = null },
    .{ .str = "meta", .next = null },
    null,
    null,
    .{ .str = "param", .next = null },
    null,
    null,
    .{ .str = "source", .next = null },
    .{ .str = "track", .next = null },
    null,
    null,
    .{ .str = "wbr", .next = null },
    null,
    null,
    null,
};

pub fn is_singlton_tag(tag: []const u8) bool
{
    if(tag.len > 0)
    {
        if(SingltonTags[tag[0] - 'a'] != null)
        {
            var singlton_tag = SingltonTags[tag[0] - 'a'];
            if(std.mem.eql(u8, tag, singlton_tag.?.str))
            {
                return true;
            }
            else if(singlton_tag.?.next != null)
            {
                if(std.mem.eql(u8, tag, singlton_tag.?.next.?.str))
                {
                    return true;
                }
            }
        }
    }

    return false;
}

test "singlton tag"
{
    var tag: []const u8 = "br";
    try std.testing.expectEqual(tag, SingltonTags[1].?.next.?.str);

    var tag2: []const u8 = "command";
    try std.testing.expect(is_singlton_tag(tag2));

    var tag3: []const u8 = "tag";
    try std.testing.expectEqual(false, is_singlton_tag(tag3));
}