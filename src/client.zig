const std = @import("std");
const net = std.net;

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    const stdin = init_stdin: {
        const file = std.io.getStdIn().reader();
        var br = std.io.bufferedReader(file);
        break :init_stdin br.reader();
    };

    var line_buf = std.ArrayList(u8).init(std.heap.c_allocator);

    const address = try net.Address.resolveIp("127.0.0.1", 32444);
    var stream = try net.tcpConnectToAddress(address);
    defer stream.close();

    const stream_reader = init_stream: {
        var br = std.io.bufferedReader(stream.reader());
        break :init_stream br.reader();
    };

    std.debug.print("Connected to {}\n", .{address});

    while (true) {
        line_buf.clearRetainingCapacity();
        stdin.streamUntilDelimiter(line_buf.writer(), '\n', null) catch |err| switch (err) {
            error.EndOfStream => break,
            else => return err,
        };

        try stream.writer().writeAll(line_buf.items);
        try stream.writer().writeAll("\n");

        line_buf.clearRetainingCapacity();
        try stream_reader.streamUntilDelimiter(line_buf.writer(), '\n', null);
        try stdout.writeAll(line_buf.items);
        try stdout.writeAll("\n");
        try bw.flush();
    }
}
