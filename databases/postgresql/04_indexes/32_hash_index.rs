#### HASH INDEX
// usted only for strict equality lookups
// becaus hashes of the value stores in an index
// optimizid to store hashes
// and it is faster then btree, but only for strict queries
CREATE INDEX email_hash ON users USING hash(email);

//! for big string values like urls, tokens etc, it is a good choice
