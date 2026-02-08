// if statement accept only bool
const std = @import("std");
const expect = std.testing.expect;

test "if statement" {
    const a = true;
    var x: u16 = 0;

    // statement
    if (a) {
        x += 1;
    } else {
        x += 2;
    }

    try expect(x == 1);
}

test "if statement expression" {
    const a = true;
    var x: u16 = 0;

    // expression
    x += if (a) 1 else 2;

    try expect(x == 1);
}

// INFO: there is no && and ||
// if i need && i should use 'and' keyword
// if i need || i should use 'or' keword
