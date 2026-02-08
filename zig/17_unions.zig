const expect = @import("std").testing.expect;

// UNION
// this is the type that can be only one of any
const Result = union {
    int: i64,
    float: f64,
    bool: bool,
};

test "simple union" {
    var result = Result{ .int = 1234 }; // now it is the integer
    result.float = 12.34; // but i can change it
}

// TAGGED UNION
// tags
const Tag = enum { a, b, c };

// tagged union
const Tagged = union(Tag) { a: u8, b: f32, c: bool };

test "switch on tagged union" {
    var value = Tagged{ .b = 1.5 };

    // switch on union
    switch (value) {
        .a => |*byte| byte.* += 1,
        .b => |*float| float.* *= 2,
        .c => |*b| b.* = !b.*,
    }
    try expect(value.b == 3);
}

// infering tag type
const Tagged2 = union(enum) { a: u8, b: f32, c: bool };

// i can add a void memeber
const Tagged3 = union(enum) { a: u8, b: f32, c: bool, none };
