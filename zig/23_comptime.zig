// CODE CAN BE EXECUTE IN COMPTIME
// with comptime keyword
const expect = @import("std").testing.expect;

fn fibonacci(n: u16) u16 {
    if (n == 0 or n == 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

test "comptime blocks" {
    const x = comptime fibonacci(10);
    const y = comptime blk: {
        break :blk fibonacci(10);
    };
    try expect(y == 55);
    try expect(x == 55);
}

// COMPTIME INTEGERS AND FLOATS
// all const values are comptime and have type comptime_int
// all vars are runtime
// comptime_floats always f128, and can't cast to integers
test "comptime_int" {
    const a = 12; // all consts are comptime
    const b = a + 10; // this in allthough comptime

    const c = a; // comptime
    const d: f32 = b;

    try expect(c == 12);
    try expect(d == 22);
}

// CHECKING BY TYPE
test "branching on types" {
    const a = 5;
    const b: if (a < 10) f32 else i32 = 5;
    try expect(b == 5);
    try expect(@TypeOf(b) == f32);
}

// COMPTIME PARAMS
// that mean that params should be known at comptime
fn Matrix(
    comptime T: type,
    comptime width: comptime_int,
    comptime height: comptime_int,
) type {
    return [height][width]T;
}

test "returning a type" {
    try expect(Matrix(f32, 4, 4) == [4][4]f32);
}

// REFLECTIONS
fn addSmallInts(comptime T: type, a: T, b: T) T {
    return switch (@typeInfo(T)) {
        .comptime_int => a + b,
        .int => |info| if (info.bits <= 16)
            a + b
        else
            @compileError("ints too large"),
        else => @compileError("only ints accepted"),
    };
}

test "typeinfo switch" {
    const x = addSmallInts(u16, 20, 30);
    try expect(@TypeOf(x) == u16);
    try expect(x == 50);
}

// RETURN TYPE FROM TYPE INFO
fn GetBiggerInt(comptime T: type) type {
    return @Type(.{
        .int = .{
            .bits = @typeInfo(T).int.bits + 1,
            .signedness = @typeInfo(T).int.signedness,
        },
    });
}

test "@Type" {
    try expect(GetBiggerInt(u8) == u9);
    try expect(GetBiggerInt(i31) == i32);
}
