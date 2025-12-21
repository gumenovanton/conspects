#### SEQUENCE
// create
CREATE SEQUENCE seq
AS BIGINT
START 1 // if not to set start with it will default to 0
INCREMENT 1
MINVALUE 1
MAXVALUE 1000
CACHE 10; // how many values to cache in memory before creating a new one, not required

#### FUNCTIONS
// to create a sequense
>>>> nextval('seq')
SELECT nextval('seq'); // every select will increment the sequence value - 1
SELECT nextval('seq'); // every select will increment the sequence value - 2

>>>> currval('seq')
SELECT currval('seq'); // current value of the sequence without incrementing it - always 2
//! if in other session someone does nextval('seq')
//! in this session currval('seq') will be 2
//! and if nextval('seq') is not called in this session, currval will force an erro
