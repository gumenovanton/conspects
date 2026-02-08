#### IDENTITY
// bigint the best type for a primary key
// i need to set it as identity
// and if i try to insert a value into an id field, it will cause an error
// and this is the main reason that the bigint with identity is the best type for primary key
// and this is the most secure way to generate a unique identifier
CREATE TABLE id_example (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY, // it will be increment automatically
    name text
);

// this is a sequence under the hood
// to get a name of the sequence
SELECT pg_get_serial_sequence('id_example', 'id');
