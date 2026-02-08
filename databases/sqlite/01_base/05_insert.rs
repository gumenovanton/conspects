//>> INSERT
INSERT INTO users (name, age) 
VALUES ('Tom', 37);

// if insert all fields, u do not need to use names
INSERT INTO users 
VALUES (2, 'Bob', 41);

// u can skip fields(if did not set NOT NULL) where val is null
// and u can skip fields with DEFAULT constraint if u have no value
INSERT INTO users (name) 
VALUES ('Sam');

// or set NULL value directly
INSERT INTO users (name, age) 
VALUES (NULL, NULL);

//>> INSERT MANY VALUES IN A TIME
INSERT INTO users (name, age) VALUES
('Tom', 37),
('Bob', 41),
('Sam', 28);