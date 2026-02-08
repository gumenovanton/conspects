#### FOREIGN KEY CONSTRAINTS
>>>> field level constraint
// first table
CREATE TABLE states (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT
)

// table with foreign key
CREATE TABLE cities (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    state_id BIGINT REFERENCES states(id) NOT NULL, // data types must match
    name TEXT NOT NULL
);

//! if i try to insert a city with unknown state id
//! it will fail with an error, because postgres will check the referential integrity
//! and the foreign key fields must be unique

>>>> table level constraint
CREATE TABLE cities (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    state_id BIGINT NOT NULL, // data types must match
    name TEXT NOT NULL,
    FOREIGN KEY (state_id) REFERENCES states(id)
    // default value looks like this
    FOREIGN KEY (state_id) REFERENCES states(id) ON DELETE NO ACTION // you can't delete a child without deletion a parent
    FOREIGN KEY (state_id) REFERENCES states(id) ON DELETE RESTRICT // the same thing as no action
    // or delete cascade all values
    //! do not use cascade deletion
    FOREIGN KEY (state_id) REFERENCES states(id) ON DELETE CASCADE ON UPDATE CASCADE
);

#### BEST PRACTICES
// foreign key costrain inforces a referential integrity
// and you should use it
// do not use cascade on delete
