# CARACTER TYPES
// the worst performens
// do not use this type
create table char_example (
    abbr char(5)
);

insert into char_example (abbr) values ('tetet');

# VARCHAR
// character varying
// be very ocuracy with lengs
// do not use it, better to use check constraints
create table varchar_example (
    last_name character varying(30)
);

# TEXT
// use it for big texts
create table text_example (
    description  text
);

# BEST PRACTICES
// use text with constraints instead varchar
