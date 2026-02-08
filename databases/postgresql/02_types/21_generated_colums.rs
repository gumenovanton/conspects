#### GENERATED COLUMS
// buisness requirements can change
// and you just alter a table and generate values that you need
// because of it generated colums is great thing
// this is just a referense to another column or columns but transformated

// it is grate for extracting parts of the json

// postgres has only stored generated columns
// not virtual

CREATE TABLE people (
    height_cm numeric,
    height_in numeric GENERATED ALWAYS AS (height_cm/2.54) STORED
);

>>>> inserting
// height_in will generated automaticaly
// but you can't insert a value in generated field
INSERT INTO people (height_cm) values (100);

>>>> antother example
// maybe you want to block some domain
CREATE TABLE users(
    email text,
    email_domain TEXT GENERATED ALWAYS AS (split_part(email, '@', 2)) STORED // split by @ and take second
);

#### RULES
// you can't use only a row value, not another table or row value
// you can't use functions like random, uuid etc, output must be allways the same, not matter how iterations was make
// you can't use another generated column
