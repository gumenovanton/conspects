//>> compare profiles
// to compare profiles i need to get profile two times
// before and after refactoring
// and just run
go tool pprof -http=:8080 -diff_base old_prof new_prof