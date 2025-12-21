# STORING BINARY DATA
// use it for storing checksums
// or small avatars, but...... better think about it
// because files is better to store in file storage

CREATE TABLE bytea_example (
    file_name TEXT,
    data BYTEA
);
