const expect = @import("std").testing.expect;

// INTEGER RULES
const decimal_int: i32 = 98222;
const hex_int: u8 = 0xff;
const another_hex_int: u8 = 0xFF;
const octal_int: u16 = 0o755;
const binary_int: u8 = 0b11110000;

// i can use _ for delimeter
const one_billion: u64 = 1_000_000_000;
const binary_mask: u64 = 0b1_1111_1111;
const permissions: u64 = 0o7_5_5;
const big_address: u64 = 0xFF80_0000_0000_0000;

// UP COERCION
test "integer widening" {
    const a: u8 = 250;
    const b: u16 = a;
    const c: u32 = b;
    try expect(c == a);
}

// CASTING
// if the value given is out of the range of the destination type, this is detectable illegal behaviour.
test "@intCast" {
    const x: u64 = 200;
    const y = @as(u8, @intCast(x));
    try expect(@TypeOf(y) == u8);
}

// OVERFLOW WRAPPERS
// integers by default are not overflowed
// but i can overflow them like in GO with % operator
// + +%
// - -%
// * *%
// += +%=
// -= -%=
// *= *%=
test "well defined overflow" {
    var a: u8 = 255;
    a +%= 1;
    try expect(a == 0);
}
