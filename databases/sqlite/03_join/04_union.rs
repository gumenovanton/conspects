//>> UNION 
// name & age must be in both tables
SELECT name, age 
FROM clients
UNION SELECT name, age FROM employees;

// if not it will be an error
SELECT name, age , account_sum
FROM clients
UNION SELECT name, age FROM employees;

//<< ALL
// with union all equal records will be delired from responce
// to return all of them use ALL 
SELECT name, age 
FROM clients
UNION ALL SELECT name, age FROM employees;

//>> UNOIN FROM ONE TABLE
// here discount calcuculates from sum of spended money
SELECT name, age, account_sum + account_sum * 0.1 AS total_sum 
FROM clients WHERE account_sum < 3000
UNION SELECT name, age, account_sum + account_sum * 0.3 AS total_sum 
FROM clients WHERE account_sum >= 3000;