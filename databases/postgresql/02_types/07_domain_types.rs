# DOMAIN TYPES
// wraps data type with constraint in separate type
// format - is the name of constraint, that will be showed in error message
CREATE DOMAIN us_postal_code as TEXT CONSTRAINT format CHECK (
    VALUE ~ '^\d{5}$' OR VALUE ~ '^\d{5}-\d{5}$'
);

CREATE TABLE domain_example (
    street TEXT NOT NULL,
    city TEXT NOT NULL,
    postal us_postal_code NOT NULL
);
