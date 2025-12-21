//>> creating logger with config
func New() *zap.Logger {
    // create encoder conf
    // dev conf just uppercase loglevel and short keys in json
	encoderConfig := zap.NewDevelopmentEncoderConfig()
	encoderConfig := zap.NewProductionEncoderConfig()
    // set timestamp instead of milliseconds
	encoderConfig.EncodeTime = zapcore.ISO8601TimeEncoder

	config := zap.Config{
		Level:             zap.NewAtomicLevelAt(zap.DebugLevel), // loglevel
		Development:       true, // puts in dev, for dpanic, and do stacktrace more humanreadable 
		DisableCaller:     false, // dont show caller message
		DisableStacktrace: false, // disable stacktrace
		Sampling:          nil, // sampling policy, to log not every log
		Encoding:          "console", // output destination, valid json, console
		EncoderConfig:     encoderConfig, 
		OutputPaths: []string{  // output destinations
			"stdout",
		},
		ErrorOutputPaths: []string{
			"stderr", // error destinations
		},
		InitialFields: map[string]interface{}{
			"init message to every log message": "zaplog", // initial message for every log
		},
	}
	return zap.Must(config.Build())
}