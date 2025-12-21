//>> rase condition detecting
// to detect rase conditions i need to use flag -race
// and i can use it in build and in run commands
go test -race

//! sometimes its not working
// of cource its do not catch race conditions 
// when you manipulating data from external sources like db

//<< how to test
// i just create variable
// create case when i now how to realy value must be changed
// change it with func to test
// end compare with the want valiable