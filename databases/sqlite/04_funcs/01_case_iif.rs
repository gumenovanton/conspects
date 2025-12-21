//>> CASE
// allow to get val of another val
// return 3 cols in this rows
SELECT name, items_count, 
CASE
	WHEN items_count = 1 
		THEN 'Товар заканчивается'
	WHEN items_count = 2 
		THEN 'Мало товара'
	WHEN items_count = 3 
		THEN 'Есть в наличии'
	ELSE 'Много товара'
END AS category
FROM products;

//>> IIF
//  simple condition
SELECT name, company, items_count,
    IIF(items_count > 2, 'Много товара', 'Мало товара') AS status
FROM products;

// many
SELECT name, company, items_count,
    IIF(items_count == 1, 'Товар заканчивается', 
        IIF(items_count==2, 'Мало товара', 
            IIF(items_count==3, 'Есть в наличии', 'Много товара'))) AS status
FROM products;