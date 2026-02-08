#### INDEX NAMING
// indexes lives in db, not in schima, be carefull
// the better choice for naming is
// tablename_column(s)_unique
// like
CREATE INDEX users_email_idx ON users(email);
