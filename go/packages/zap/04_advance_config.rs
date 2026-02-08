//>> advance config
func createLogger() *zap.Logger {
    // console destination
    stdout := zapcore.AddSync(os.Stdout)

    // file destination with lumberjack to rotate logs. They do not get to large
    file := zapcore.AddSync(&lumberjack.Logger{
        Filename:   "logs/app.log",
        MaxSize:    10, // megabytes
        MaxBackups: 3,
        MaxAge:     7, // days
    })

    level := zap.NewAtomicLevelAt(zap.InfoLevel)

    productionCfg := zap.NewProductionEncoderConfig()
    productionCfg.TimeKey = "timestamp"
    productionCfg.EncodeTime = zapcore.ISO8601TimeEncoder

    developmentCfg := zap.NewDevelopmentEncoderConfig()
    developmentCfg.EncodeLevel = zapcore.CapitalColorLevelEncoder

    consoleEncoder := zapcore.NewConsoleEncoder(developmentCfg)
    fileEncoder := zapcore.NewJSONEncoder(productionCfg)

    // core from 2 cores
    core := zapcore.NewTee(
        zapcore.NewCore(consoleEncoder, stdout, level),
        zapcore.NewCore(fileEncoder, file, level),
    )

    // create logger
    return zap.New(core)
}

func main() {
    logger := createLogger()

    defer logger.Sync()

    logger.Info("Hello from Zap!")
}

2023-05-15T16:15:05.466+0100    INFO    Hello from Zap!
{"level":"info","timestamp":"2023-05-15T16:15:05.466+0100","msg":"Hello from Zap!"}