// to iterate through arrays
const expect = @import("std").testing.expect;

test "for" {
    //character literals are equivalent to integer literals
    const string = [_]u8{ 'a', 'b', 'c' };

    // i can iterate from any index to any index, and get an element and an index
    for (string, 0..) |character, index| {
        _ = character;
        _ = index;
    }

    // i can iterate through all elements and get only elemenst
    for (string) |character| {
        _ = character;
    }

    // i can get only indexes
    for (string, 0..) |_, index| {
        _ = index;
    }

    // i can get nothing
    for (string) |_| {}
}

// INLINE LOOPS
// thay unwraps in compile time, to increase performance
// while loops work similary
// do not use it with big arrays
test "inline for" {
    const types = [_]type{ i32, f32, u8, bool };
    var sum: usize = 0;
    inline for (types) |T| sum += @sizeOf(T);
    try expect(sum == 10);
}
