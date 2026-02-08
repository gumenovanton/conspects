//>> pprof cli
// this is cli to watch profiles
// to run profile via pprof cli
go tool pprof cpu.pro
// or run it on web browser
go tool pprof -http=:8080 cpu.pro

//<< how to watch

//>> commands
//<< top - showes top 10 hardest funcs and displays 
flat - operation time in ms
flat% - operation time in % of all app work
sum% - total sum of % of work
cum - operation time with all subprocesses
cum% - operation time with all subprocesses in %

//<< web - shows tree of funcs in web brawser

//<< list - shows time of runing every row of code
// but i must run in from directory where code was compiled
list doSomething

