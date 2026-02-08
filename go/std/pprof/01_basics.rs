//>> PROFILING
// when i get profile(memory usage, cpu params) of work of my app and analize it

//>> how to get profile
// i can use 2 methods
//<< 1 - write benchmark and run it with flags to create profile
//<< 2 - run pprof methods from code

//>> 1 - write banchmark and create profile with flags
// example func
func doSomething()  {
}

// benchmark
func BethmarkDoSomething(b *testing.B)  {
    for i:=0; i<b.N; i++{
        doSomething()
    }
}

//<< memory stats 
// to watch how much memory were used and how much heap alloc i got, i can run 
// but its not profiling
go test -bench . -benchmem

//<< get profiles
// get profile of cpu and memory usage
go test -bench . -benchmem -cpuprofile=cpu.pro -memprofile=mem.pro

//<< watch profile
// this command will run pprof cli
go tool pprof cpu.pro
// or 
go tool pprof mem.pro
