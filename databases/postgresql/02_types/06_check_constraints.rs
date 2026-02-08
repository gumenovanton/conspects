# CHECK CONSTRAINTS

## column constraints
// work for column and use only onw value
// if i want number greater then 0
// or i want text lengh=5
// CONSTRAINT - creates a name to constraint, that will be sowed in an error message
// CHECK - sets a constraint
CREATE TABLE check_example (
    price numeric CONSTRAINT prise_must_be_positive CHECK (price > 0),
    abbr text CHECK (LENGTH(abbr)=5) // constraint without a name
);


## table constraints
// every column constraints can be set as table constraints
// but if a constraint gets two columns of the table, it should be set as table constraint, it is a good practice
CREATE TABLE check_example (
    price numeric CONSTRAINT prise_must_be_positive CHECK (price > 0),
    descount_price numeric CHECK (discount_price > 0),
    abbr text,
    CHECK (LENGTH(abbr)=5), // column_constraint as table constraint, all the same
    CHECK (price > discount_price) // the better way to set this constraint is to set it as a table constraint
);

# BEST PRACTICES
// use more data check in the app instead an db
