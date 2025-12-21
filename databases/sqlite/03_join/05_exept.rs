//>> EXEPT
// differance between 2 selection
SELECT name, age
FROM clients
EXCEPT SELECT name, age 
FROM employees;