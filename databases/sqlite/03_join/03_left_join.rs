//>> LEFT JOIN 
// выведет все результаты как в простом JOIN
// но к ним добавит остальные записи изглавной таблице 
// даже если для которых нет записей в присоединяемой таблице
// заменив отстствуюзие знаяения на NULL
// обрати внимание какая таблица главная
SELECT name, created_at, items_count, price 
FROM customers LEFT JOIN orders 
ON orders.customer_id = customers.id;

//<< COMBINE
// products adds to orders at first
// than adds customers
SELECT customers.name, orders.created_at, 
       products.name, products.company
FROM orders 
JOIN products ON orders.product_id = products.id AND products.price > 45000
LEFT JOIN customers ON orders.customer_id = customers.id
ORDER BY orders.created_at;