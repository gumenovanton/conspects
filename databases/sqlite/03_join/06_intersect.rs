//>> INTERSECT
// just eqoal strings for to selections
SELECT name, age
FROM employees
INTERSECT SELECT name, age 
FROM clients;