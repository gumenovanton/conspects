#### SERIAL
// autoincrementing integer
// not recomended to use it for primary keys
// this is a sequence off integfer under the hood
//! there is possibility to out of range of the value

CREATE TABLE serial_example (
    id SERIAL PRIMARY KEY
);


#### BIGSERIAL
// autoincrementing bigint
CREATE TABLE serial_example (
    id BIGSERIAL PRIMARY KEY
);
