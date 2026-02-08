//>> INNER JOIN
//is not necessary to use INNER key word

// two tables
SELECT orders.created_at, orders.items_count, products.name
FROM orders
JOIN products ON products.id = orders.product_id;

// tree tables
SELECT orders.created_at, customers.name, products.name
FROM orders
JOIN products ON products.id = orders.product_id
JOIN customers ON customers.id=orders.customer_id;

// with filter and sort
SELECT orders.created_at, customers.name, products.name
FROM orders
JOIN products ON products.id = orders.product_id
JOIN customers ON customers.id=orders.customer_id
WHERE products.price > 45000
ORDER BY customers.name;
