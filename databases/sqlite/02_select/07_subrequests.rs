//>> SUBREQUESTS
SELECT * FROM users
WHERE age > (SELECT AVG(age) FROM users);

// selection data from another table
SELECT name, age, 
        (SELECT name FROM companies 
        WHERE companies.id = users.company_id) AS company
FROM users;

// ones more
SELECT name,
    age,
    (SELECT AVG(age) FROM users AS subuser 
    WHERE subuser.company_id=user.company_id) AS avg_age
FROM users AS user
WHERE age > 
    (SELECT AVG(age) FROM users AS subuser 
    WHERE subuser.company_id=user.company_id);

//>> IN BETWEEN
//<< IN
SELECT * FROM users
WHERE company_id IN (SELECT id FROM companies 
                WHERE name='Microsoft' OR name='Google');

//<< BETWEEN
SELECT * FROM users
WHERE age BETWEEN
    (SELECT AVG(age) FROM users) AND
    (SELECT MAX(age) FROM users);

//>> INSERT
INSERT INTO users (name, age, company_id) VALUES
('Tom', 37, (SELECT id FROM companies WHERE name='Microsoft'));

//>> UPDATE
// in filter 
UPDATE users
SET age = age + 3
WHERE company_id =(SELECT id FROM companies WHERE name='Microsoft');

// in val
UPDATE users
SET age = (SELECT MAX(age) FROM users)
WHERE id=1;

//>> DELETE
DELETE FROM users
WHERE company_id=(SELECT id FROM companies WHERE name='Microsoft');

