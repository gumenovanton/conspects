const std = @import("std");
const expect = std.testing.expect;

test "arena allocator" {
    var arena: std.heap.ArenaAllocator = .init(std.heap.page_allocator);
    defer arena.deinit(); // need to deinit
    const allocator = arena.allocator();

    _ = try allocator.alloc(u8, 1);
    _ = try allocator.alloc(u8, 10);
    _ = try allocator.alloc(u8, 100);
    _ = try allocator.create(i32); // create a box for only 1 i32 var
    _ = try allocator.dupe(u8, "Hello"); // copy Hello string to box
}

// DESCRIPTION
// give an arena with no size
// take as much memory as you want
// can take memory for any type with create method
// cant free a pease only deinit all arena
// can reset arena

// WHEN TO USE
// many allocations
// all objects live and die at one time
// need max allocation speed
// parsing, request hendling, compilation
// want to simplify memory management
// use it as a cache
