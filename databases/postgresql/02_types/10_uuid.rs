# UUID as a data type
// the best type to store generated uuid
// this is not the text
// this takes only 16 bytes
// if i use text instead uuid, it will take 40 bytes

CREATE TABLE uuid_example (
    uuid_value UUID
);

## how to generate uuid
select gen_random_uuid()

// if i want have timestamp in uuid, i should us uuid v7, available in pg extentions
