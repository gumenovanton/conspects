//>> НЕЯВНОЕ СОЕДИНЕНИЕ 
// here orders have foreign key to customers 
SELECT * FROM orders, customers
WHERE orders.customer_id = customers.id;

// tree tables
SELECT customers.name, products.name, orders.created_at 
FROM orders, customers, products
WHERE orders.customer_id = customers.id AND orders.product_id=products.id;

