//>> compare banchmarks
// to compare benchmarks i need to install benchstat util

// then i must write old test data to file
go test -bench . -benchmem -count=8 > bench_old
// then i refactor my code
// then i must write new test dato to file
go test -bench . -benchmem -count=8 > bench_new
// and compare it
benchstat bench_old bench_new