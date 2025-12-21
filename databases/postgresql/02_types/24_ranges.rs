#### CREATE RANGES
// creating via literal
SELECT '[1,5]'::numrange; // [1,5] include 1 and include 5, not transformed because here infinite values
SELECT '[1,5]'::int4range; // [1,6) include 1 and exclude 5, transforms automatically to [1,6) because here only 5 values
SELECT '[1,6)'::int4range; // [1,6) include 1 and exclude 5

// creating via constructor function
select numrange(1, 5); // [1,5)
select int4range(1, 5); // [1,5)
select numrange(1, 5, '[]'); // [1,5]
select int4range(1, 5, '[]'); // [1,5]

>>>> creation in table
CREATE TABLE range_example(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    num_range NUMRANGE,
    int_range INT4RANGE,
    date_range DATE_RANGE,
    ts_range TSRANGE
);

#### SELECTION
// if a range contains a value
SELECT * FROM range_example WHERE int_range @> 3;

// to find a range that overlaps with another range
SELECT * FROM range_example WHERE int_range && int4range(2, 4);
// or
SELECT * FROM range_example WHERE int_range && '[2, 4)';

// select intersection of two ranges
SELECT int4range(10, 20) && int4range(15, 25); // [15,20)

#### MULTIRANGE
// to store many ranges in one field
// there is many types of multiranges
// int4multirange
// int8multirange
// nummultirange
// tsmultirange
// tstzmultirange
// datemultirange
SELECT '{[3,7],[10,15]}'::int4multirange;
