#### ARRAY
// type to store arrays
// best way to store cell data of deagram
CREATE TABLE array_example (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    int_arr INTEGER[],
    text_arr TEXT[],
    bool_arr BOOLEAN[],
    nested_arr INTEGER[][]
);

>>>> how to insert into array
// long
INSERT INTO
    array_example (int_array, text_array, bool_array)
VALUES
    (
        ARRAY [1, 2, 3, 4],
        ARRAY ['marigold', 'dasy', 'poppy', 'sunflower'],
        ARRAY [true, false, true, false]
    );

// short
INSERT INTO array_example (nested_array) VALUES
    ('{{1,2,3}, {4,5,6}, {7,8,9}}');

#### FUNCTIONS
>>>> get by index, starts from 1 not from 0
SELECT
    id,
    text_array[1]
FROM
    array_example;

>>>> slicing
SELECT
    id,
    text_array[1:4] // [:3], [4:] is correct
FROM
    array_example;

>>>> select all arrays that includes some val
// this query returns entire array
SELECT
    id,
    text_array
FROM
    array_example
WHERE
    text_array @> ARRAY['poppy']; // {'marigold', 'dasy', 'poppy','sunflower'}
    // or
    //text_array @> {'poppy'};

>>>> array values as field values
SELECT
    id,
    unnest(text_array)
FROM
    array_example;
// 1 marigold
// 2 dasy
// 3 poppy
// 4 sunflower
