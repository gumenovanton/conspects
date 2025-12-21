//>> IN 
// set of values to find

//<< IN
SELECT * FROM products
WHERE company IN ('Samsung', 'Xiaomi', 'Huawei');

//<< NOT IN
SELECT * FROM products
WHERE company NOT IN ('Samsung', 'Xiaomi', 'Huawei');

//>> BETWEEN
// send range of values

//<< BETWEEN
SELECT * FROM products
WHERE price BETWEEN 20000 AND 50000;

//<< NOT BETWEEN
SELECT * FROM products
WHERE price NOT BETWEEN 20000 AND 50000;

SELECT * FROM products
WHERE price * product_count BETWEEN 90000 AND 150000;

//>> LIKE
// take template to search values
% - substring
_ - one character

//<< LIKE
// all iPhone models
SELECT * FROM products
WHERE name LIKE 'iPhone%'';// must be one '

//<< NOT LIKE
// not all iPhone models
SELECT * FROM products
WHERE name NOT LIKE 'iPhone%'';// must be one '

//>> GLOB
// like LIKE but have different syntax
* - любое количество символов
? - любой один
. - любой одинчный
[abc] - a или b или c
[a-zA-Z0-9] - диапазон символов 
[^0-9] - не диапазон символов 

//<< GLOB
SELECT * FROM products
WHERE name  GLOB '*Phone*';

//<< NOT GLOB
SELECT * FROM products
WHERE name  NOT GLOB '*Phone*';

//>> IS NULL
// all where field is null

//<< IS NULL
SELECT * FROM products
WHERE product_count IS NULL;

//<< IS NOT NULL
SELECT * FROM products
WHERE product_count IS NOT NULL;
