HOW TO PROFILE HEAP
// показывает данные между итерациями GC
// add heap handler and heap profiler
mux.Handle("/debug/pprof/heap", pprof.Handler("heap"))

HOW TO RUN
& PPROF_TMPDIR=$PWD/pprof go tool pprof -http :8082 -seconds 20 http://127.0.0.1:8080/debug/pprof/heap
// на стрелках написано какой ресурс мы смотрим
// в данном случае это heap
