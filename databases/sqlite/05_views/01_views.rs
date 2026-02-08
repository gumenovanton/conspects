//>> VIEWS
//представления

//>> CREATE VIEW
CREATE VIEW IF NOT EXISTS orders_products_customers AS
SELECT orders.created_at AS order_date, 
        customers.name AS customer,
        products.name As product  
FROM orders 
INNER JOIN products ON orders.product_id = products.id
INNER JOIN customers ON orders.customer_id = customers.id;

//>> SELECT VIEW
SELECT * FROM orders_products_customers;

//>> DROP VIEW
DROP VIEW IF EXISTS orders_products_customers;