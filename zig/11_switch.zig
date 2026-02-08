const expect = @import("std").testing.expect;

// AS STATEMENT
test "switch statement" {
    var x: i8 = 10;
    switch (x) {
        -1...1 => {
            x = -x;
        },
        10, 100 => {
            //special considerations must be made
            //when dividing signed integers
            x = @divExact(x, 10);
        },
        else => {},
    }
    try expect(x == 1);
}

// AS EXPRESSION
test "switch expression" {
    var x: i8 = 10;
    x = switch (x) {
        -1...1 => -x, // if x between -1 and 1 change x sign
        10, 100 => @divExact(x, 10), // if x 10 or 100 use a zig function to devide x on 10 without tail
        else => x, // else just return this
    };
    try expect(x == 1);
}
