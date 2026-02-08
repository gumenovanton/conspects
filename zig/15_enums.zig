const expect = @import("std").testing.expect;

// ENUM
const Direction = enum { north, south, east, west }; // create an enum

// INTEGER TAG

// i can set an integer tag like IOTA in GO
const Value = enum(u2) { zero, one, two };

test "enum ordinal value" {
    try expect(@intFromEnum(Value.zero) == 0);
    try expect(@intFromEnum(Value.one) == 1);
    try expect(@intFromEnum(Value.two) == 2);
}

// OVERRIDIN THE TAGS
const Value2 = enum(u32) {
    hundred = 100,
    thousand = 1000,
    million = 1000000,
    next, // previous + 1
};

test "set enum ordinal value" {
    try expect(@intFromEnum(Value2.hundred) == 100);
    try expect(@intFromEnum(Value2.thousand) == 1000);
    try expect(@intFromEnum(Value2.million) == 1000000);
    try expect(@intFromEnum(Value2.next) == 1000001); // previous + 1
}

// METHODS

const Suit = enum {
    clubs,
    spades,
    diamonds,
    hearts,
    pub fn isClubs(self: Suit) bool {
        return self == Suit.clubs;
    }
};

// usage
test "enum method" {
    try expect(Suit.spades.isClubs() == Suit.isClubs(.spades));
}

// VARIABLE ENUMS
const Mode = enum {
    var count: u32 = 0; // i can store a value in the enum
    on, // it has no tag
    off,
};

test "hmm" {
    Mode.count += 1;
    try expect(Mode.count == 1);
}
