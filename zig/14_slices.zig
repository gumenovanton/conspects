// thay have double pointer size
// because they are a many item pointers with length value
// slices are the most common way to pass around buffers in Zig

// slicing in ZED like slicing in GO array[start:end]
// but in zed i can store slices in the stack or in the heap, it is my choice

// i can't resize a slice

// ITERATION THROUGH THE SLICES
const expect = @import("std").testing.expect;

fn total(values: []const u8) usize {
    var sum: usize = 0;
    for (values) |v| sum += v; // short syntax
    return sum;
}

test "slices" {
    const array = [_]u8{ 1, 2, 3, 4, 5 };
    const slice = array[0..3]; // create a const slice - when start and end is known in a compile time, this is the pointer
    try expect(total(slice) == 6);
}

test "slices 3" {
    var array = [_]u8{ 1, 2, 3, 4, 5 };
    const slice = array[0..];
    _ = slice;
}
