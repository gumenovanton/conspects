const expect = @import("std").testing.expect;

test "defer" {
    var x: i16 = 5;
    {
        defer x += 2;
        try expect(x == 5);
        // defer will execute here
    }
    try expect(x == 7);
    // not here
}

test "multi defer" {
    var x: f32 = 5;
    {
        defer x += 3; // will called second
        defer x /= 2; // will called first
    }
    try expect(x == 5.5);
}
