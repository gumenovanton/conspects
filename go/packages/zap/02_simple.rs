//>> simple usage
//<< dev variant
logger := zap.Must(zap.NewDevelopment())
defer logger.Sync()

logger.Info("Hello from Zap logger!")

// 2025-04-14T19:34:11.578+0500    INFO    playground/main.go:9    Hello from Zap logger!

//<< prod variant
logger := zap.Must(zap.NewProduction())
defer logger.Sync()

logger.Info("Hello from Zap logger!")

// {"level":"info","ts":1684092708.7246346,"caller":"zap/main.go:12","msg":"Hello from Zap logger!"}

//>> log levels
DEBUG (-1): for recording messages useful for debugging.
INFO (0): for messages describing normal application operations.
WARN (1): for recording messages indicating something unusual happened that may need attention before it escalates to a more severe issue.
ERROR (2): for recording unexpected error conditions in the program.
DPANIC (3): for recording severe error conditions in development. It behaves like PANIC in development and ERROR in production.
PANIC (4): calls panic() after logging an error condition.
FATAL (5): calls os.Exit(1) after logging an error condition.

//>> hiding sensitive data
//to hide sensitive data better to use Stringer implementation

type User struct {
    ID    string `json:"id"`
    Name  string `json:"name"`
    Email string `json:"email"`
}

func (u User) String() string {
    return u.ID
}

logger.Info("user login", zap.Any("user", user))
// {"level":"info","timestamp":"2023-05-17T17:05:01.081+0100","msg":"user login","user":"USR-12345"}

//>> creating child logger with some context
    childLogger := logger.With(
        zap.String("service", "userService"),
        zap.String("requestID", "abc123"),
    )

    childLogger.Info("user registration successful",
        zap.String("username", "john.doe"),
        zap.String("email", "john@example.com"),
    )

    childLogger.Info("redirecting user to admin dashboard")

{"level":"info","ts":1684164941.7644951,"caller":"zap/main.go:52","msg":"user registration successful","service":"userService","requestID":"abc123","username":"john.doe","email":"john@example.com"}
{"level":"info","ts":1684164941.764551,"caller":"zap/main.go:57","msg":"redirecting user to admin dashboard","service":"userService","requestID":"abc123"}

//>> log errors
logger.Fatal("Something went terribly wrong",
    zap.String("context", "main"),
    zap.Int("code", 500),
    zap.Error(errors.New("An error occurred")),
)

{"level":"fatal","ts":1684170760.2103574,"caller":"zap/main.go:47","msg":"Something went terribly wrong","context":"main","code":500,"error":"An error occurred","stacktrace":"main.main\n\t/home/ayo/dev/demo/zap/main.go:47\nruntime.main\n\t/usr/local/go/src/runtime/proc.go:250"}