#### NULL
// unknown value, no value
// you can ask is the column null

// example
CREATE TABLE products(
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name text, // null by default
    price numeric // null by default
);
