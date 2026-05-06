HOW TO PROFILE ALLOCS
// показывает всю картину по всему приложению
// add allocs handler and allocs profiler
mux.Handle("/debug/pprof/allocs", pprof.Handler("allocs"))

HOW TO RUN
& PPROF_TMPDIR=$PWD/pprof go tool pprof -http :8082 -seconds 20 http://127.0.0.1:8080/debug/pprof/allocs
// на стрелках написано какой ресурс мы смотрим
// в данном случае это allocs
