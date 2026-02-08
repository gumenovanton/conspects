# CASTING NUMERIC TYPES
// there is two methods of casting
select 100::int8;
// or
select cast(100 as int8);
// both the same

## how to check the type
select pg_typeof(100::int8); // integer

## how to check a column size
select pg_column_size(100::int2); // 2
