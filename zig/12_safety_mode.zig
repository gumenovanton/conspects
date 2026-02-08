const expect = @import("std").testing.expect;
// SAFETY MODE
// used by default
// it invoke panic on problems during execution

// here will be panic
test "out of bounds" {
    const a = [3]u8{ 1, 2, 3 };
    var index: u8 = 5;
    const b = a[index];

    _ = b;
    index = index;
}

// i can turn it off
// and here will be undefined behaviour
test "out of bounds, no safety" {
    @setRuntimeSafety(false);
    const a = [3]u8{ 1, 2, 3 };
    var index: u8 = 5;
    const b = a[index];

    _ = b;
    index = index;
}

// UNREACHABLE
// when some branch never be rich
fn asciiToUpper(x: u8) u8 {
    return switch (x) {
        'a'...'z' => x + 'A' - 'a',
        'A'...'Z' => x,
        else => unreachable, // that branch will never be reach because x always A-Za-z
    };
}

test "unreachable switch" {
    try expect(asciiToUpper('a') == 'A');
    try expect(asciiToUpper('A') == 'A');
}
