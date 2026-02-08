// const: imutable value
const constant: i32 = 5;

// var: mutable value
var variable: u32 = 5000;

// i can use explict coercion in asingment
const inferred_constant = @as(i32, 5);
var inferred_variable = @as(u32, 5000);

// i cant asign an empty value into a var or const
// if no known value can be given, the undefined value can be used
const a: i32 = undefined;
