// pointers cant be null or 0
const expect = @import("std").testing.expect;

fn increment(num: *u8) void { // get the refference
    num.* += 1; // dereferencing
}

test "pointers" {
    var x: u8 = 1; // var
    increment(&x); // pass the reference
    try expect(x == 2);
}

// POINTERS ON 0 is illegal
test "naughty pointer" {
    var x: u16 = 5;
    x -= 5;
    var y: *u8 = @ptrFromInt(x);
    y = y;
}

// CONST POINTERS
// you can't modify the value
test "const pointers" {
    const x: u8 = 1;
    var y = &x;
    y.* += 1;
}

// POINTERS SIZE
test "usize" {
    try expect(@sizeOf(usize) == @sizeOf(*u8));
    try expect(@sizeOf(isize) == @sizeOf(*u8));
}

// MANY ITEM POINTERS SYNTAX
// used for work with C
// pointer to an array with unknown length but nown where the start
fn doubleAllManypointer(buffer: [*]u8, byte_count: usize) void { // get a pointer on unknown byte size
    var i: usize = 0;
    while (i < byte_count) : (i += 1) buffer[i] *= 2; // while short syntax, mutate the array
}

test "many-item pointers" {
    var buffer: [100]u8 = [_]u8{1} ** 100; // array of u8, add 100 values each equals 1
    const buffer_ptr: *[100]u8 = &buffer; // create pointer on array

    const buffer_many_ptr: [*]u8 = buffer_ptr; // pointer to many bytes, this pointer do not know the length but it equals 100
    doubleAllManypointer(buffer_many_ptr, buffer.len); // double every value
    for (buffer) |byte| try expect(byte == 2); // iteration through the array

    const first_elem_ptr: *u8 = &buffer_many_ptr[0]; // take pointer on 1 element
    const first_elem_ptr_2: *u8 = @ptrCast(buffer_many_ptr); // take pointer on 1 element
    try expect(first_elem_ptr == first_elem_ptr_2); // they equal
}
