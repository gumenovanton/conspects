# ENUM
// string for me, integer for pg
// think aboit it like aliases for predefined strings
// if i sort values of enums they are will be sorted in creation order, because it is not a string

CREATE TYPE mood AS ENUM ('happy', 'sad', 'neutral');

CREATE TABLE enum_example (
    current_mood mood
);

## how to add variant into enum
// it adds a value to the end
ALTER TYPE mood ADD VALUE 'excited';


## how to add before value
// it adds it before sad
ALTER TYPE mood ADD VALUE 'afraid' BEFORE 'sad';

## how to add after value
// it adds it after sad
ALTER TYPE mood ADD VALUE 'afraid' AFTER 'sad';

// removing is impossible

## how to see indexes of the enums
SELECT * FROM pg_catalog.pg_enum;

## how to see all enum values in its order
// from 0 to end
SELECT enum_range(null::mood);
// from 'afraid' to 'sed'
SELECT enum_range('afraid'::mood, 'sad'::mood);

## if i want separate record for every enum
SELECT unnest(enum_range(NULL::myenum));

# PROS & CONS
// lmaller size
// harder to edit
