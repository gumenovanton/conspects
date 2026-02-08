//>> UPDATE

//<< UPDATE ALL
UPDATE products 
SET price = price + 3000;

//<< UPDATE ALL WHERE
UPDATE products 
SET company = 'Samsung' 
WHERE company = 'Samsunger';

// or many columns
UPDATE products 
SET company = 'Samsung', 
product_count = product_count + 3 
WHERE company = 'Samsung Inc.';