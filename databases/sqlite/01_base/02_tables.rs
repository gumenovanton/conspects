//>> CREATE TABLE
CREATE TABLE Users
(
    Id INTEGER,
    Name TEXT,
    Age INTEGER
);

//if not exists
CREATE TABLE IF NOT EXISTS Users
(
    Id INTEGER,
    Name TEXT,
    Age INTEGER
);

//>> DELETE TABLE
DROP TABLE users;

// if exists
DROP TABLE IF EXISTS users;

//>> CONSTRAINTS
//<< PRIMARY KEY
// key must be unique
CREATE TABLE users
(
    id INTEGER PRIMARY KEY,
    name TEXT,
    age INTEGER
);

// alternative
CREATE TABLE users
(
    id INTEGER,
    name TEXT,
    age INTEGER,
    PRIMARY KEY(id)
);

// composed 
CREATE TABLE users
(
    id INTEGER,
    name TEXT,
    age INTEGER,
    PRIMARY KEY(id, name)
);

//>> AUTOINCREMENT
CREATE TABLE users
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    age INTEGER
);

//>> UNIQUE
CREATE TABLE users
(f
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    age INTEGER,
    email TEXT UNIQUE
);

//>> NOT NULL
CREATE TABLE users
(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    age INTEGER
);

//>> DEFAULT
CREATE TABLE users
(
    id INTEGER PRIMARY KEY,
    name TEXT,
    age INTEGER DEFAULT 18
);

//>> CHECK
// checks velue on insert
CREATE TABLE users
(
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL CHECK(name !=''),
    age INTEGER NOT NULL CHECK(age >0 AND age < 100)
);

//>> CONSTRAINT OPERATOR
CREATE TABLE users
(
    id INTEGER,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    age INTEGER NOT NULL,
    CONSTRAINT users_pk PRIMARY KEY(id),
    CONSTRAINT user_email_uq UNIQUE(email),
    CONSTRAINT user_age_chk CHECK(age >0 AND age < 100)
);
