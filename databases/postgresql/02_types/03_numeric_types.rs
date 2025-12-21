# INTEGERS
// before you choose type to table field
// look on your data and analize them
// everytime choose smallest type for your data

//! in pg no unsigned types

## smalint or int2
// takes 2 bytes, in golang its int16

## integer or int4
// takes 4 bytes, in golang its int32

## bigint or int8
// takes 8 bytes, in golang its int64

# NUMERIC - same as decimal
// for money
// but it's very very slow

## numeric without params
// can hold any number, much more then bigint

//<< numeric with params
numeric(precision, scale)
// 14,88888
// precision=7(max number of unrounded digits)
// scale=5

// if i write
numeric(10,2)
// i can put into it
10.34334543
// and pg put 10.34
// but i cant put
138383383.29
// it will be error

// if i write
numeric(5,-2)
// i can put 7 digits
5 - unrounded
2 - rounded
//  i can put
2344332
// and pg puts
2344300

# FLOUTING
// lower ocuracy but very very fast
// for temperature etc
// when i do the mathematical ogeration i can have values like 1.400000000000001
// if i need to compare them, just flour them

## real or float4
// takes 4 bytes, and 6 digits after decimal point, in golang its float32

## double or float8
// takes 8 bytes, and 15 digits after decimal point, in golang its float64

# MONEY
// type to store money id db
// very fast
// but you loose precision
//! do not use it, just store money as integer
select 1000::money;
// $1,000.00

## how to change money sign
// set sc_monetary = 'ru_RU.UTF-8'

# NAN and INFINITY
// forget about it
// NaN - not a number
// INFINITY - infinity

# BEST PRACTICES
// use integers instead of money
// bigint the best type for primary key
