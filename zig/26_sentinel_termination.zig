// ARRAYS, SLICES, POINTERS
// can be terminated by their child type

// terminated sympol say here is the finish
// when i create the terminated array i do not need to store a length of this array
// i just can read to the end

// used to work with C

// HOW TO CREATE TERMINATED ARRAY
const a: [3:0]u8 = .{ 1, 2, 3 }; // 3 числа + нoль
const b: [5:255]u8 = .{ 10, 20, 30, 40, 50 }; // 5 чисел + 255
const c: [3:null]?*u8 = .{ null, null, null }; // Сентинел null

// STRINGS LINERALS ARE TERMINATED BY DEFAULT
const expect = @import("std").testing.expect;

test "string literal" {
    try expect(@TypeOf("hello") == *const [5:0]u8);
}

// C STRING
test "C string" {
    const c_string: [*:0]const u8 = "hello";
    var array: [5]u8 = undefined;

    var i: usize = 0;
    while (c_string[i] != 0) : (i += 1) {
        array[i] = c_string[i];
    }
}

// COERCION
test "coercion" {
    const a2: [*:0]u8 = undefined;
    const b2: [*]u8 = a2;

    const c2: [5:0]u8 = undefined;
    const d2: [5]u8 = c2;

    const e2: [:0]f32 = undefined;
    const f2: []f32 = e2;

    _ = .{ b2, d2, f2 }; //ignore unused
}

// SLICING
test "sentinel terminated slicing" {
    var x = [_:0]u8{255} ** 3; //sentinel terminated array with 255 values of 3 and 0 at the end
    const y = x[0..3 :0]; // should to add a :0
    _ = y;
}
