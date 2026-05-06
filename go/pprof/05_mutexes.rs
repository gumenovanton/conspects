HOW TO PROFILE LOCKS
// enable mutex profiling
runtime.SetMutexiProfileFraction(1)
// add mutex handler and mutex profiler
mux.Handle("/debug/pprof/mutex", pprof.Handler("mutex"))

HOW TO RUN
& PPROF_TMPDIR=$PWD/pprof go tool pprof -http :8082 -seconds 20 http://127.0.0.1:8080/debug/pprof/mutex
// на стрелках написано какой ресурс мы смотрим
// в данном случае это мьютексы
