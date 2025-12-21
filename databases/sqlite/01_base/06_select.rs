//>> SELECT
//<< CALCULATIONS
SELECT 5 * 6;

SELECT 5 * 6, 
       5 + 6, 
       5 - 6;

//<< SELECT ALL
SELECT * FROM users;

//<< SELECT FIELDS
SELECT id, name 
FROM users;

// with calculations
SELECT name, product_count * price 
FROM products;

//<< SELECT WITH ALIASES
SELECT name AS title, product_count * price AS total_sum 
FROM products;

//>> WHERE (filtration)
=   сравнение на равенство
!=  сравнение на неравенство
<>  сравнение на неравенство
<   меньше чем
>   больше чем
<=  меньше чем или равно
>=  больше чем или равно

SELECT * FROM products 
WHERE company = 'Samsung';

SELECT * FROM products 
WHERE product_count < 3;

SELECT * FROM products 
WHERE product_count * price > 100000;


//>> NOT AND OR
// NOT - have the highest priority
// AND - have medium priority
// OR  - have the lowest priority
SELECT * FROM products 
WHERE company = 'Samsung' AND price > 50000;

SELECT * FROM products 
WHERE company = 'Samsung' OR price > 50000;

SELECT * FROM products 
WHERE NOT company = 'Samsung';


// NOT first
SELECT * FROM products
WHERE company ='Samsung' OR NOT price > 30000 AND product_count > 2;

// brackets can change priority
SELECT * FROM products
WHERE company ='Samsung' OR NOT (price > 30000 AND product_count > 2);