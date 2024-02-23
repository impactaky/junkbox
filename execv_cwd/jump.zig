const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    const args = try std.process.argsAlloc(allocator);
    const env = comptime [_:null]?[*:0]const u8{"foo=var"};
    const my_args = comptime [_:null]?[*:0]const u8{"mimiced!!!"};
    std.os.execveZ(args[1], &my_args, &env) catch unreachable; // This code won't be executed if execveZ is successful
}
