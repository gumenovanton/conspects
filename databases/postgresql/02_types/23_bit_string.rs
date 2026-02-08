#### BIT STRING
// to store many boolean values in one field
// for example feature flags
// select binary string
SELECT B'1010';
// or
SELECT '1010'::BIT(32); // the length in braces

>>>> creation
CREATE TABLE bits (
    bit3 bit(3), // the length in braces
    bitv bit(32)
);
