# TIMESTAMP
// without time zone
CREATE TABLE timestamp_example (
    "id" SERIAL PRIMARY KEY,
    "timestamp" timestamp without time zone
);

// with time zone
CREATE TABLE timestamp_example (
    "id" SERIAL PRIMARY KEY,
    "timestamp" timestamp without time zone
);
//or
CREATE TABLE timestamp_example (
    "id" SERIAL PRIMARY KEY,
    "timestamp" timestamptz // default 6 digits after . in seconds
);

//with time zone with fractional seconds
CREATE TABLE timestamp_example (
    "id" SERIAL PRIMARY KEY,
    "timestamp" timestamptz(0-4) // 0-4 digits after . in seconds, i can use an exact value TIME(5)
);

## how to set date style
// ISO 8601 is
2025-01-31 11:30:09.000+6:00
// or
2025-01-31T11:30:09.000+6:00
// or
2025-01-31T11:30:09.000Z
// by default
// postgres has ISO, DMY settings
// that means
select '2024-01-31'::date; // will be parsed as ydm, becaus it obviously
select '1/3/2013'::date; // will be parsed as dmy

// if i want to change date style
set datestyle = 'ISO, MDY';
select '1/3/2013'::date; // will be parsed as mdy

# DATE
// type to store date separately
CREATE TABLE date_example (
    date_col DATE
);

# TIME
// type to store time separately
CREATE TABLE time_example (
    time_col TIME
);
//or
CREATE TABLE time_example (
    time_col TIME(0-4) // 0-4 digits after . in seconds, i can use an exact value TIME(5)
);

# time in local time zone
// this will output time in utc with +5:00
SELECT now();
// but this will output current time in local time zone
SELECT LOCALTIME;



# BEST PRACTICE
// store timestamps always with timezone
// send into db and store timestamps always in ISO 8601 format or unix format
// store timestamps as long as possible in UTC, and convert to local time zone when displaying to users in an application
// if needed to set timezone do not use offset, use named timezone instead
//! i will use timestamp in UNIX format, because its always in UTC
// if i need to store date and time, never store it separately
// never use timetz type
