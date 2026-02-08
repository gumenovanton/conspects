//>> COUNT - вычисляет количество строк в запросе
SELECT COUNT(*) FROM products;

// by column
SELECT COUNT(company) FROM products;

// unique
SELECT COUNT(DISTINCT company) FROM products;

//>> AVG - вычисляет среднее значение
SELECT AVG(price) AS average_price FROM products;

// with filter
SELECT AVG(price) FROM products
WHERE company='Apple';

//<< ALL DISTINCT
// ALL - use all values to calc avg
// DISTINCT - use just unique values to calc avg
SELECT AVG(DISTINCT price) FROM products;

//>> SUM - вычисляет сумму значений
SELECT SUM(product_count) FROM products;

// with calc
SELECT SUM(product_count * price) FROM products;

// unique
SELECT SUM(DISTINCT price) FROM products;

//>> MIN - вычисляет наименьшее значение
//>> MAX - вычисляет наибольшее значение
// ignores NULL 
SELECT MIN(price), MAX(price) FROM products;

//>> COMBINATION
SELECT COUNT(DISTINCT company) AS companies_count, 
SUM(product_count) AS total_count,
MIN(price) AS min_price,
MAX(price) AS max_price,
AVG(price) AS avg_price
FROM products;
