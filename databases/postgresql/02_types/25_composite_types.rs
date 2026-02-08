#### COMPOSITE TYPES
// like struct in go
// used very infrequently
CREATE TYPE address AS (street TEXT, city TEXT, state TEXT, zip TEXT);

CREATE TABLE addresses (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    address address
);

SELECT ROW('123 Main St', 'Anytown', 'CA', '12345')::address;
// or
SELECT ('123 Main St', 'Anytown', 'CA', '12345')::address;

#### SELECTION
// separated values
SELECT id, (address).street, (address).city, (address).state, (address).zip FROM addresses;
