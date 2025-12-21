//>> 2 - get profile in code

// to get profile in code i need do 4 steps
// 1 - create routes and add pprof endpoints in to it
// 2 - start server
// 3 - load server 
// 4 - ping endpoint i need to write profile in file

//>> get cpu profile example
// with chi router
//<< 1 - create pprof routes 
r.Group(func(r chi.Router) {
    r.Get("/profiling/pprof/", pprof.Index)
    r.Get("/profiling/pprof/cmdline", pprof.Cmdline)
    r.Get("/profiling/pprof/profile", pprof.Profile)
    r.Get("/profiling/pprof/symbol", pprof.Symbol)
    r.Get("/profiling/pprof/trace", pprof.Trace)
    r.Get("/profiling/pprof/block", pprof.Handler("block").ServeHTTP)
    r.Get("/profiling/pprof/goroutine", pprof.Handler("goroutine").ServeHTTP)
    r.Get("/profiling/pprof/threadcreate", pprof.Handler("threadcreate").ServeHTTP)
    r.Get("/profiling/pprof/allocs", pprof.Handler("allocs").ServeHTTP)
    r.Get("/profiling/pprof/heap", pprof.Handler("heap").ServeHTTP)
    r.Get("/profiling/pprof/mutex", pprof.Handler("mutex").ServeHTTP)
})

//<< 2 - start server

//<< 3 - load server
// here with apacheBench (post - is file with json body)
ab -t 20 -n 100000000 -c 5 -T 'application/json' -p ./post 'http://localhost:8080/api/address/search'

//<< 4 - get cpu profile in file
// for 10 seconds 
curl http://localhost:8080/profiling/ggrof/profile?seconds=10 > profile

//<< 5 - and analize profile
go tool pprof profile

//>> i can get all profiles i want
//<< trace
curl http://localhost:8080/profiling/ggrof/trace?seconds=10 > tracing
// to watch trace i need to run
go tool trace tracing

//<< memory profile
curl http://localhost:8080/profiling/ggrof/heap?seconds=10 > profile
// watch it like cpu profile 