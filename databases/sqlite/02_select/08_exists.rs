//>> EXISTS
// to check does the query return at least one row
SELECT * FROM companies
WHERE EXISTS 
(SELECT * FROM users WHERE users.company_id = companies.id);

//<< NOT EXISTS
SELECT * FROM companies
WHERE NOT EXISTS 
(SELECT * FROM users WHERE users.company_id = companies.id);