const expect = @import("std").testing.expect;

//CREATE A STRUCT
const Vec3 = struct { x: f32, y: f32, z: f32 };

test "struct usage" {
    // init an object
    const my_vector: Vec3 = .{
        .x = 0,
        .y = 100,
        .z = 50,
    };
    _ = my_vector;
}

// i can't miss the value
test "missing struct field" {
    const my_vector: Vec3 = .{
        .x = 0,
        .z = 50,
    };
    _ = my_vector;
}

// DEFAULT VALUES
const Vec4 = struct { x: f32 = 0, y: f32 = 0, z: f32 = 0, w: f32 = 0 };

test "struct defaults" {
    const my_vector: Vec4 = .{
        .x = 25,
        .y = -50,
    };
    _ = my_vector;
}

// METHODS
const Stuff = struct {
    x: i32,
    y: i32,
    fn swap(self: *Stuff) void {
        const tmp = self.x;
        self.x = self.y;
        self.y = tmp;
    }
};

test "automatic dereference" {
    var thing = Stuff{ .x = 10, .y = 20 }; // i can use this syntax too
    thing.swap();
    try expect(thing.x == 20);
    try expect(thing.y == 10);
}

// ANONYMOUS STRUCTS
test "fully anonymous struct" {
    try dump(.{
        .int = @as(u32, 1234),
        .float = @as(f64, 12.34),
        .b = true,
        .s = "hi",
    });
}

fn dump(args: anytype) !void {
    try expect(args.int == 1234);
    try expect(args.float == 12.34);
    try expect(args.b);
    try expect(args.s[0] == 'h');
    try expect(args.s[1] == 'i');
}

// PRIVATE FIELDS
// zig have no private fields
// if i want to has a private fields i need to use OPAQUE type
// but i can mark fields that anyone should to be aware of with _

// TUPLES - ANONIMOUS STRUCT WITHOUT FIELD NAMES
test "tuple" {
    const values = .{
        @as(u32, 1234),
        @as(f64, 12.34),
        true,
        "hi",
    } ++ .{false} ** 2;
    try expect(values[0] == 1234); // access with indexes
    try expect(values[4] == false);
    inline for (values, 0..) |v, i| {
        if (i != 2) continue;
        try expect(v);
    }
    try expect(values.len == 6);
    try expect(values.@"3"[0] == 'h');
}
