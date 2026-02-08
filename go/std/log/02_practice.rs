//>> ПРАКТИКА ЛОГИРОВАНИЯ С ПОМОЩЬЮ ПАКЕТА log
// логирование с помощью пакета log происходит через прохождение следующих этапов

//! МНЕ НЕ НРАВИТСЯ ЭТОТ ПОДХОД, ТАК КАК СЛИШКОМ МНОГО ИНТЕРФЕЙСОВ

// ~ создание интерфейса логгера и перечислений уровней логов
// создаю тип уровней логов
type LogLevel int

// задаю сами уровни
const (
	Trace LogLevel = iota
	Debug
	Information
	Warning
	Fatal
	None
)

// создаю интерфейс логгера с набором функций, которые описывают каждый уровень
type Logger interface {
	Trace(string)
	Tracef(string, ...interface{})
	Debug(string)
	Debugf(string, ...interface{})
	Info(string)
	Infof(string, ...interface{})
	Warn(string)
	Warnf(string, ...interface{})
	Panic(string)
	Panicf(string, ...interface{})
}

// ~ создание логгера по умолчанию и реализация методов для него
// теперь я могу стоздать любой логгер, который реализует этот интерфейс
// но могу делать так

// создаю структуру которая хранит
// - минимльный уровень логов, для дева он может быть более емкий, а в проде только фатальные ошибки
// - мапа логгеров
// - разрешение на вызов паники

type DefaultLogger struct {
	minLevel     LogLevel
	loggers      map[LogLevel]*log.Logger
	triggerPanic bool
}

// геттер возвращающий текущий минимальный уровень логов
func (l *DefaultLogger) MinLogLevel() LogLevel {
	return l.minLevel
}

// базавая функция записи логов, она вызывается во всех методах логгера
func (l *DefaultLogger) write(level LogLevel, message string) {
	if l.minLevel <= level {
		l.loggers[level].Output(2, message)
	}
}

// запись Trace логов
func (l *DefaultLogger) Trace(msg string) {
	l.write(Trace, msg)
}

// форматированнная запись Trace логов
func (l *DefaultLogger) Tracef(template string, vals ...interface{}) {
	l.write(Trace, fmt.Sprintf(template, vals...))
}

// запись Debug логов
func (l *DefaultLogger) Debug(msg string) {
	l.write(Debug, msg)
}

// форматированная запись Debug логов
func (l *DefaultLogger) Debugf(template string, vals ...interface{}) {
	l.write(Debug, fmt.Sprintf(template, vals...))
}

// запись Info логов
func (l *DefaultLogger) Info(msg string) {
	l.write(Information, msg)
}

// форматированная запись Info логов
func (l *DefaultLogger) Infof(template string, vals ...interface{}) {
	l.write(Information, fmt.Sprintf(template, vals...))
}

// запись Warn логов
func (l *DefaultLogger) Warn(msg string) {
	l.write(Warning, msg)
}

// форматированная запись Warn логов
func (l *DefaultLogger) Warnf(template string, vals ...interface{}) {
	l.write(Warning, fmt.Sprintf(template, vals...))
}

// запись Panic логов
func (l *DefaultLogger) Panic(msg string) {
	l.write(Fatal, msg)
	if l.triggerPanic {
		panic(msg)
	}
}

// форматированная запись Panic логов
func (l *DefaultLogger) Panicf(template string, vals ...interface{}) {
	formattedMsg := fmt.Sprintf(template, vals...)
	l.write(Fatal, formattedMsg)
	if l.triggerPanic {
		panic(formattedMsg)
	}
}

// ~ создание логгера по умолчанию
// это логгер который будет писать в консоль
//! аналогично можно создать логгер для прода, указав нужные места назначения
//! или преедав их через конфиги
func NewDefaultLogger(level LogLevel) Logger {
	flags := log.Lmsgprefix | log.Ltime
	return &DefaultLogger{
		minLevel: level,
		loggers: map[LogLevel]*log.Logger{
			Trace:       log.New(os.Stdout, "TRACE ", flags),
			Debug:       log.New(os.Stdout, "DEBUG ", flags),
			Information: log.New(os.Stdout, "INFO ", flags),
			Warning:     log.New(os.Stdout, "WARN ", flags),
			Fatal:       log.New(os.Stdout, "FATAL ", flags),
		},
		triggerPanic: true,
	}
}