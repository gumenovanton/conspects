const std = @import("std"); // import std lib

// to run it
// zig run 01_hello_world.zig
pub fn main() !void { // ! means that it can return an error
    // write to console
    std.debug.print("Hello, {s}!\n", .{"World"});
}
