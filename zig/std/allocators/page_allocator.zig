const std = @import("std");
const expect = std.testing.expect;

test "allocation" {
    const allocator = std.heap.page_allocator;

    const memory = try allocator.alloc(u8, 100); // give you one page 4096 bites
    defer allocator.free(memory);

    try expect(memory.len == 100);
    try expect(@TypeOf(memory) == []u8);
}

// PAGE_ALLOCATOR
// give you as mutch pages as you want in memory at a time (usually 4KB on x86 on page)
// the simpliest alocator

// WHEN TO USE
// with big objects
const texture_size = 1024 * 1024; // 1MB for texture
const texture = try page_allocator.alloc(u8, texture_size);
// when need temporary big buffers
const file_size = 10 * 1024 * 1024;
const buffer = try page_allocator.alloc(u8, file_size);
defer page_allocator.free(buffer); // free all at once
                                  
// PROS 
// fast for big data and one allocation
// no fragmentation
// free all at a time
// good for huge pages

// CONS
// if need 10 byte it takes 4096, and other memory is unused and locked
// slow on friquent allocations
// cant reuse memory
// min 1 page