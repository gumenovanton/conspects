#### ROWS FROM
// when i want to output two of generated colums together in one column

// for example i want to group this generated values in one column
select * from generate_series(1,10);
select * from generate_series(100,111);

// i can do it with rows from

select * from rows from (
    generate_series(1,10);
    generate_series(100,111);
) as t(lower, upper);

// lower    upper
// 1        101
// 2        102
// 3        103
// 4        104
// 5        105
// 6        106
// 7        107
// 8        108
// 9        109
// 10       110
// NULL     111

// another example
select date:date, num from rows from (
    generate_series('2024-01-01'::date, '2024-12-31'::date, '1 day'),
    generate_series(1, 380)
) as t(date, num);

// date         num
// 2024-01-02   1

// another example
select * from rows from(
    unnest(array[101,102,103]),
    unnest(array['laptop','smartphone','tablet']),
    unnest(array[999.99,499.99,299.99])
) as combined(product_id, product_name, price);
