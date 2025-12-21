#### JSON
// type to store JSON data
// stored as text under the hood
CREATE TABLE json_data (
    id SERIAL PRIMARY KEY,
    data JSON
);

#### JSONB
// type to store JSON data
// much less storage space than JSON
// stored as blob under the hood
// has rules
    // duplicate keys are not allowed, and will be ignored
    // all keys will be stored in ordered manner
    // no double spaces in text
CREATE TABLE json_data (
    id SERIAL PRIMARY KEY,
    data JSONB
);

>>>> HOW TO EXTRACT VALUES SEPARATELY
// with quotes
SELECT '"string": "hello, world"'::jsonb->'string';

// without quotes
SELECT '"string": "hello, world"'::jsonb->>'string';

// array element
SELECT '"array": [1, 2, 3]'::jsonb->'array'->0;

// object key without quotes
SELECT '"object": {"key1": "value1", "key2": "value2"}'::jsonb->'object'->>'key1';

#### BEST PRACTICE
// do not use json type to store values that will be selected in queries separately
// create separate columns for it
// use jsonb
