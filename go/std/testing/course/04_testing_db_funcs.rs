//>> testing func that uses db
// when i have func that prepare query
// and conncts to db to get some data
// i can't use mock because i must chech db errors on real db
// i can create local db instance, ant test my func on it

//<< how to implement it
// i can write it on the test function or in sime preparing function
// - write db connection code
// - write db preparing code(create db, create tables, add some test data)

// then connect to db again
// and test my cases
