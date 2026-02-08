#### EXPLAIN
// to understand how the query done

#### STRUCTURE
EXPLAIN SELECT * FROM users LIMIT 10;
// Limit (cost=0.00..0.24 rows=10 width=76)
//  -> Seq Scan on users (cost=0.00..23580.08 rows=989908 width=76)

// i should right it from inside to outside

>>>> plan as json
// i can output explain as json
EXPLAIN (format json) SELECT * FROM users LIMIT 10;

#### TYPES OF SCAN
// the pg desided how to scan a tables
// in physical disc order
// or go to an index

>>>> types of scan
// sequential scan - scan all table in physical order - the slowest method
// bitmap index scan (works together with hash scan, when qwery like email<'b') - go to index, find values that need(email<'b') and get with HASH SCAN scan them in the physical order, faster then seq scan
// index scan (works with queries like email='aaron@example.com') - scan index and read value from memory, the fastest way
// index only scan (works with queries like select email forum users where email='aaron@example.com' ) - scan the index, takes value from index and do not go to memory, the fastest way

#### INFO
// cost - unknown value, just costs units, minimum cost is the best, just for optimizing
// cost=0.00..0.24 - start cost .. end cost

// rows - max value of rows may be affected

// with - how wide in bites every message

#### ANALIZE
// to show more detales
EXPLAIN ANALIZE SELECT * FROM users LIMIT 10;
// it shows
// execution time in milliseconds
// planning time in milliseconds
// actual tame of every row
// loops count of every row - each node could be run many time
