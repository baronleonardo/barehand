const std = @import("std");

const singlton_tag = @import("singlton_tag.zig");

pub const Html = @This();

allocator: std.mem.Allocator,

pub fn new(raw: []u8, allocator: std.mem.Allocator) Html
{
    build_tree(raw, allocator);

    return Html {
        .allocator = allocator,
    };
}

pub fn build_tree(raw: []u8, allocator: std.mem.Allocator) void
{
    var stack = std.ArrayList([]u8).init(allocator);
    defer stack.deinit();

    var iii: usize = 0;
    while (iii < raw.len): (iii += 1)
    {
        if (raw[iii] == '<')
        {
            iii += 1;

            // comment
            if(iii < raw.len and raw[iii] == '!')
            {
                if(iii + 2 < raw.len and raw[iii + 1] == '-')
                {
                    // skip '!--'
                    iii += 3;

                    // skip the whole comments
                    var comment_start = iii;
                    while(iii + 3 < raw.len and !std.mem.eql(u8, "-->", raw[iii..iii + 3])): (iii += 1) {}
                    var comment_end = iii;
                    std.log.warn("-- [comment]: `{s}`", .{raw[comment_start..comment_end]});

                    // skip '-->'
                    iii += 3;
                }
                else
                {
                    var special_tag_start = iii;
                    while(iii < raw.len and raw[iii] != '>') : (iii += 1) {}
                    var special_tag_end = iii;

                    std.log.warn("-- [special tag]: `{s}`", .{raw[special_tag_start..special_tag_end]});

                    // skip '>'
                    // iii += 1;
                }
            }
            // normal elements
            else
            {
                // end tags
                if(iii < raw.len and raw[iii] == '/')
                {
                    var end_start = iii;
                    while(iii < raw.len and raw[iii] != '>') : (iii += 1) {}
                    var end_end = iii;

                    std.log.warn("-- [end]: `{s}`", .{raw[end_start..end_end]});
                    _ = stack.popOrNull();

                    // // sometimes there would be values after the end tag
                    // var start_element_value = iii + 1;
                    // while(iii < raw.len and raw[iii] != '<') : (iii += 1) {}
                    // var end_element_value = iii;

                    // iii -= 1;

                    // std.log.warn("---- [value]: `{s}`", .{raw[start_element_value..end_element_value]});

                    // skip '>'
                    // iii += 1;
                }
                // start tags
                else
                {
                    // tag
                    var tag_start = iii;
                    while(iii < raw.len and (raw[iii] != ' ' and raw[iii] != '>')) : (iii += 1) {}
                    var tag_end = iii;
                    var tag = raw[tag_start..tag_end];
                    stack.append(tag) catch unreachable;

                    // element arguments
                    var start_element_args = iii;
                    while(iii < raw.len and raw[iii] != '>') : (iii += 1) {}
                    var end_element_args = iii;

                    if(singlton_tag.is_singlton_tag(tag))
                    {
                        std.log.warn("-- [singlton start]: `{s}`", .{tag});

                        // if(raw[start_element_args] == ' ') start_element_args += 1;
                        if(raw[end_element_args - 1] == '/') end_element_args -= 1;
                        std.log.warn("---- [singlton args]: `{s}`", .{raw[start_element_args..end_element_args]});
                    }
                    else
                    {
                        std.log.warn("-- [start]: `{s}`", .{tag});
                        std.log.warn("---- [args]: `{s}`", .{raw[start_element_args..end_element_args]});

                        // skip '>'
                        // iii += 1;

                        // var start_element_value = iii + 1;
                        // while(iii < raw.len and raw[iii] != '<') : (iii += 1) {}
                        // var end_element_value = iii;

                        // iii -= 1;

                        // std.log.warn("---- [value]: `{s}`", .{raw[start_element_value..end_element_value]});
                    }
                }
            }
        }
        else
        {
            var start_element_text = iii;
            while(iii < raw.len and raw[iii] != '<') : (iii += 1) {}
            var end_element_text = iii;

            iii -= 1;

            const trimmed_element_text = std.mem.trim(u8, raw[start_element_text..end_element_text], &std.ascii.whitespace);
            if(trimmed_element_text.len > 0)
            {
                std.log.warn("---- [text]: `{s}`", .{trimmed_element_text});
            }
        }
    }
}

test "html"
{
    const allocator = std.testing.allocator;
    var raw_html = try std.fs.cwd().readFileAlloc(allocator, "src/html/assets/index.html", 512 * 1024);
    _ = new(raw_html, allocator);
    allocator.free(raw_html);
}