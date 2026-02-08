#### CREATE INDEX

>>>> rules to create indexes on single column
// best principal but not fully true is to create indexes on field that used after WHERE word is selection
// and the more field is unique the better choice for indexing this field is
// always test indexes with demo data and with your queries
CREATE INDEX bday ON users USING btree(birthday);

>>>> rules to create indexes on multiple columns
// 1 - from left to right no skipping (fields order is important)
// the most frequently used field must be the first field in the index
// that means if you have a query like SELECT * FROM users WHERE firstname = 'John' AND lastname = 'Doe' AND birthday = '1990-01-01'
// it's better to create an index on (firstname, lastname, birthday) than on (lastname, firstname, birthday)
// and if i SELECT * FROM users WHERE firstname = 'John' the index will be used
// and if i SELECT * FROM users WHERE firstname = 'John' AND lastname = 'Doe' the index will be used
// but if i SELECT * FROM users WHERE lastname = 'Doe' the index will not be used
// but if i SELECT * FROM users WHERE lastname = 'Doe' AND firstname = 'John' the index will not be used
CREATE INDEX multi ON users USING btree(firstname, lastname, birthday);

// 2 - stops at the first range
// if i SELECT * FROM users WHERE firstname = 'John' AND lastname = 'Doe' AND birthday < '1990-01-01'
// index search will stop at the first range
// and i should use range at the end of the index


#### COMBINING INDEXES
// if you search three field in different combinations
// create three indexes and pg will combine them automatically
CREATE INDEX first ON users USING btree(firstname);
CREATE INDEX last ON users USING btree(lastname);
CREATE INDEX birth ON users USING btree(birthday);
// and i can combine queries by needs and pg will search with them
// without scanning the table


#### COVERING
// when i put in index all data i need
// and i select it from the index without going to the table
// used very rarely
// but great in some cases

// for example if i very often need first name by last name
// like SELECT firstname, lastname FROM users WHERE lastname = 'Doe' ORDER BY firstname
// i create index
CREATE INDEX name ON users(firstname, lastname);
// and when i do this query
// pg get all from index, and do not go to table

>>>> how to add some data to index without using it in index
// INCLUDE word add it to the leaf node
CREATE INDEX name ON users(firstname, lastname) INCLUDE (id);

//! but do not do it with big data


#### PARTIAL INDEX
// index on part of values
// for example an index on rows where is_pro = true
CREATE INDEX name ON users(email) WHERE is_pro = true;

// and this index will work only if write WHERE is_pro = true part
SELECT * FROM users WHERE email = 'aaron@example.com' AND is_pro = true;

>>>> unique partial index
// works only if value is unique
CREATE UNIQUE INDEX email ON users(email) WHERE deleted_at IS NULL;


#### INDEX ORDERING
// if i have a single column index
// pg can scan it on ascending and descending order

// if i have a multi column index
// i need to create index in proper way
// becaus if you select with equal order
SELECT * FROM users ORDER BY birthday ASC, created_at ASC;
// or
SELECT * FROM users ORDER BY birthday DESC, created_at DESC;
// here is no problem

// but if you select with different order
SELECT * FROM users ORDER BY birthday ASC, created_at DESC;
// pg do not use index

// and if i need index on some specific order
// i should create index on all columns in order
CREATE INDEX birthdate_created_an ON users(birthdate ASC, created_at DESC);
// or
CREATE INDEX birthdate_created_an ON users(birthdate DESC, created_at ASC);


#### ORDERING NULLS IS INDEXES
// by default nulls are larger than other values
// but we can change it
CREATE INDEX birthdate_created_an ON users(birthdate NULLS FIRST, created_at DESC NULLS LAST);


#### FUNCTIONAL INDEX
// similar to generated columns
// but in index
CREATE INDEX domain on users((split_part(email, '@', 2)));

// and query should be like this
SELECT * FROM users WHERE split_part(email, '@', 2) = 'example.com';
// and pg will use index

// but if i change query to
SELECT * FROM users WHERE split_part(email, '@', 1) = 'example.com';
// pg will not use index

>>>> case insensitive index
CREATE INDEX email_lower ON users(lower(email));
//! query must be like this
SELECT * FROM users WHERE lower(email) = 'aaron@example.com';
// but it will not return any rows
SELECT * FROM users WHERE lower(email) = 'AARON@example.com';
