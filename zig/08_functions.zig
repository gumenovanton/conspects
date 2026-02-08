// function names are cammelCase instead of var names whitch are snake_case
// all function arguments are immutable by default

const expect = @import("std").testing.expect;

// SIMPLE
fn addFive(x: u32) u32 {
    // x = 4; // ERROR cant assign to constant
    return x + 5;
}

test "function" {
    const y = addFive(0);
    try expect(@TypeOf(y) == u32);
    try expect(y == 5);
}

// RECURSION
// but it can provide the stack overflow, how to achieve sefe recursion will be covered NEXT
fn fibonacci(n: u16) u16 {
    // or word
    if (n == 0 or n == 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

test "function recursion" {
    const x = fibonacci(10);
    try expect(x == 55);
}
