HOW TO PROFILE ALLOCS
// показывает всю картину по всему приложению
// add trace handler and trace profiler
mux.Handle("/debug/pprof/trace", pprof.Trace)

HOW TO RUN
curl http://localhost:8080/debug/pprof/trace\?seconds\=20 -o trace.out
go tool trace -http ":8082" trace.out
