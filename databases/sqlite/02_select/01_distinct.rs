//>> DISTINNCT
// allows to get unique records

// DISTINCT for one column
REPLACE INTO users (name, age) VALUES ('Tom', 22);

// DISTINCT fon many columns
SELECT DISTINCT company, product_count FROM products;