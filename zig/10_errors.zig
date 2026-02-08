// errors in zig it's like an enum where each error is a value
// there is no exeptions in zig
const expect = @import("std").testing.expect;

// CREATE AND COERCE
// create an error
const FileOpenError = error{
    AccessDenied,
    OutOfMemory,
    FileNotFound,
};
const AllocationError = error{OutOfMemory};

// coerce of the error
test "coerce error from a subset to a superset" {
    const err: FileOpenError = AllocationError.OutOfMemory;
    try expect(err == FileOpenError.OutOfMemory);
}

// UNION AS AN ERROR
// the normal way in zig to return error!value union from functions, that means error or value
test "error union" {
    const maybe_error: AllocationError!u16 = 10; // AllocationError or u16
    const no_error = maybe_error catch 0;

    try expect(@TypeOf(no_error) == u16);
    try expect(no_error == 10);
}

// CATCH
// if i want to handel error i should use catch
fn failingFunction() error{Oops}!void {
    return error.Oops;
}

test "catch an error" {
    failingFunction() catch |err| {
        try expect(err == error.Oops);
        return;
    };

    try expect(true); // this code never reached
}

// TRY
// if i want to return the error i should use try
// it is a shortcut for
// x catch |err| return err
test "returning an error" { // that test will fail because it returns an error
    try failingFunction();
}

// ERRDEFER
// what to do if error and only if error
// fe close file if opening error
// clear memory if allocate memory and error
// unlock mutex if loc mutex and error
var problems: u32 = 98;

fn failFnCounter() error{Oops}!void {
    errdefer problems += 1;
    try failingFunction();
}

test "errdefer" {
    failFnCounter() catch |err| {
        try expect(err == error.Oops);
        try expect(problems == 99);
        return;
    };
}

// I CAN'T IGNORE ERROR
// if function returns an error i should use try or catch
fn createFile() !void {
    return error.AccessDenied;
}

test "inferred error set" {
    //type coercion successfully takes place
    const x: error{AccessDenied}!void = createFile();

    //Zig does not let us ignore error unions via _ = x;
    //we must unwrap it with "try", "catch", or "if" by any means
    _ = x catch {};
}

// ERRORS CAN BE MURGED
const A = error{ NotDir, PathNotFound };
const B = error{ OutOfMemory, PathNotFound };
const C = A || B;
