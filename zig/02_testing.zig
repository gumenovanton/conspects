// i can write tests anywhere i want, even directly in the source code file
// to run test i stold run
// $ zig test test_file_name.zig
const std = @import("std");
const expect = std.testing.expect;

test "always_succeeds" {
    try expect(true);
}
