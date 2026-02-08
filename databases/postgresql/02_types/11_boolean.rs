# BOOLEAN
// true and false
// takes one byte

CREATE TABLE boolean_example (
    id SERIAL PRIMARY KEY,
    status BOOLEAN
);

## bolean colation
INSERT INTO boolean_example (status) VALUES
(TRUE),
(FALSE),
('t'),
('f'),
('true'),
('false'),
('1'),
// (1), // error
('0'),
('on'),
('off'),
('yes'),
('no'),
(NULL); // NULL means "unknown"
