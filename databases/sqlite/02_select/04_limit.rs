//>> LIMIT
SELECT * FROM users
LIMIT 3;

// skip first 2 and return next 3
SELECT * FROM users
LIMIT 2, 3;

// set offset with OFFSET
SELECT * FROM users
LIMIT 3 OFFSET 2;

