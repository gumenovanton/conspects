#### UNIQUE
// field must be unique
CREATE TABLE products(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    number TEXT UNIQUE,
    name TEXT NOT NULL,
    price NUMERIC NOT NULL CHECK(price > 0)
);

//! if unique value is null it is unique, because null !=null, because null is unknown

>>>> unique by two columns
CREATE TABLE users(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    number TEXT,
    name TEXT NOT NULL,
    price NUMERIC NOT NULL CHECK(price > 0)

    UNIQUE(number, name) // unique by two columns
);



#### BEST PRACTICES
// unique value must be not null
// or set as
...
    product_number TEXT UNIQUE NULLS NOT DISTINCT,
...
// that means that only one null value in the hole table column is allowed
