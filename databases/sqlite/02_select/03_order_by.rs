//>> ORDER BY
// sort value by column

SELECT * FROM products
ORDER BY price;

// with alias and calc
SELECT name, product_count * price AS total_sum
FROM products
ORDER BY total_sum;

// calc sort
SELECT name, price, product_count
FROM products
ORDER BY product_count * price;

//>> DESC 
// по убыванию
SELECT name, product_count
FROM products
ORDER BY product_count DESC;

//>> ASC (by default)
// по возрастанию
SELECT name, product_count
FROM products
ORDER BY product_count ASC;

//>> MANY FIELDS SORT
// many fields sort
SELECT name, price, company
FROM products
ORDER BY company, name;

// ASC DESC
SELECT name, price, company
FROM products
ORDER BY company ASC, name DESC;

//>> NULLS FIRST и NULLS LAST
// where to put records with fields with NULL values
SELECT * FROM users
ORDER BY company NULLS LAST;

