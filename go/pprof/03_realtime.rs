HOW TO PROFILE REALTIME
const addr = "0.0.0.0:8000"

func main() {
	ctx, cancel := signal.NotifyContext(context.Background(), syscall.SIGTERM, syscall.SIGINT)
	defer cancel()

	logger := slog.New(slog.NewTextHandler(os.Stdout, &slog.HandlerOptions{
		Level: slog.LevelInfo,
	}))

	s := &server{
		logger: logger,
	}

	mux := http.NewServeMux()
	mux.HandleFunc("POST /v1/action", s.endPoint)
	// end point for frofiler
	mux.HandleFunc("/degub/pprof/profile", pprof.Profile)

	server := &http.Server{
		Addr:    addr,
		Handler: mux,
	}

	go func() {
		defer cancel()
		if err := server.ListenAndServe(); err != nil {
			logger.LogAttrs(
				ctx,
				slog.LevelError,
				"server serve error",
				slog.Any("error", err),
			)
		}
	}()

	<-ctx.Done()
	shutdownCtx, cancel := context.WithTimeout(context.Background(), time.Minute)
	defer cancel()

	err := server.Shutdown(shutdownCtx)
	if err != nil {
		logger.LogAttrs(
			ctx,
			slog.LevelError,
			"server shutdown error",
			slog.Any("error", err),
		)
		return
	}
}

type server struct {
	logger *slog.Logger
}

func (rr *server) endPoint(w http.ResponseWriter, r *http.Request) {
	rr.logger.Info("request")

	f1()
	f2()
	f3()
	f4()

	w.WriteHeader(http.StatusOK)
}

//go:noinline
func f1() int {
	a := 0
	for range 1000000 {
		a++
	}
	return a
}

//go:noinline
func f2() int {
	a := 0
	for range 1000000 {
		a++
	}
	return a
}

//go:noinline
func f3() int {
	a := 0
	if rand.N(100) == 3 {
		for range int(10e8) {
			a++
		}
	}

	for range 1000000 {
		a++
	}
	return a
}

//go:noinline
func f4() int {
	a := 0
	for range 1000000 {
		a++
	}
	return a
}

HOW TO
// i need to run my app
// give the load
// and run
& PPROF_TMPDIR=$PWD/pprof go tool pprof -http :8082 -seconds 20 http://127.0.0.1:8080/debug/pprof/profile
// PPROF_TMPDIR=$PWD/pprof - dir to store pprof file
// 8082 - where to run full web version
// 20 - how much seconds i want to profile
// http... - where to get profiles

WHAT TO WATCH
// better to watch
Flame Graph
// here i can see the resources usage for a functions
