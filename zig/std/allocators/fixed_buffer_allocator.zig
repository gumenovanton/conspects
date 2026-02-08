const std = @import("std");
const expect = std.testing.expect;

test "fixed buffer allocator" {
    // buffer on the stack or on static memory, never in the heap
    var buffer: [1000]u8 = undefined;

    // create an itstance
    var fba: std.heap.FixedBufferAllocator = .init(&buffer);
    const allocator = fba.allocator();

    const memory = try allocator.alloc(u8, 100);
    defer allocator.free(memory); // i can free a peace memory but it do not get it accessible, and i cant free all buffer, just reset

    // can reset buffer, all slices will be broken

    try expect(memory.len == 100);
    try expect(@TypeOf(memory) == []u8);
}

// DESCRIPTION
// create buffer and i can use part of this buffer
// can cut it by peaces with any length
// and reset when i need to reuse all buffer
// cant free it, it is removed from memory when function ends

// WHEN TO USE
// when i want to get big buffer and cut it on peaces
// temporary calculation
// to handle one request
// recursive jobs with known size
// embeded systems
// tests and banchmarks

// PROS
// no system calls
// very fast
// sefe, if have no space - give me an error on try alloc
// auto free when destroy

// CONS
// limited size
// cant reuse
