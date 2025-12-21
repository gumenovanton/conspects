//>> GRPUP BY
// return 1 record for group
// for every group u can use funcs
SELECT company, COUNT(*) AS models_count
FROM products
GROUP BY company;

// many fields
SELECT company, product_count, COUNT(*) AS models_count
FROM products
GROUP BY company, product_count;

// with order by, order by must be after GRPUP BY
SELECT company, COUNT(*) AS models_count
FROM products
WHERE price > 40000
GROUP BY company
ORDER BY models_count DESC;

//>> HAVING
// filter groups

// all groups where number of records more than one
SELECT company, COUNT(*) AS models_count
FROM products
GROUP BY company
HAVING COUNT(*) > 1;

// with order by, order by must be after HAVING
SELECT company, COUNT(*) AS models, SUM(product_count) AS units
FROM products
WHERE price * product_count > 80000
GROUP BY company
HAVING SUM(product_count) > 2
ORDER BY units DESC;