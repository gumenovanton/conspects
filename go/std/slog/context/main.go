package main

import (
	"context"
	"log/slog"
	"os"
)

// _ ИСПОЛЬЗОВАНИЕ ПАКЕТА context
// я могу печатать логи передавая в них контекст
// с помощью методов
// 		DebugContext(ctx context.Context, msg string, args ...any)
// 		InfoContext(ctx context.Context, msg string, args ...any)
// 		WarnContext(ctx context.Context, msg string, args ...any)
// 		ErrorContext(ctx context.Context, msg string, args ...any)
// все что мы положим в контекст
// поподет в лог
// ! применяется когда хотим передать например id чтобы вывести его во всех логах
// чтобы это реализовать, нужно реализовать свой хендлер
// и переопределив метод Handler в нем засунуть атрибуты из контекста в сообщение
// например здесь я храню в контексте слайс slog атрибутов

// тип ключа для передачи в контекст
type ctxKey string

// константа - имя поля передаваемое в контекст
const (
	slogFields ctxKey = "slog_fields"
)

// создаю свой хендлер
type ContextHandler struct {
	slog.Handler
}

// реализую функцию хендлер
func (h ContextHandler) Handle(ctx context.Context, r slog.Record) error {
	// если есть такое поле, которое хранит []slog.Attr
	// добавляю в сообщение атрибут
	if attrs, ok := ctx.Value(slogFields).([]slog.Attr); ok {
		for _, v := range attrs {
			r.AddAttrs(v)
		}
	}
	// запускаю родительский хендлер
	return h.Handler.Handle(ctx, r)
}

// это просто хелпер, добавляющий атрибут в полученный контекст
func AppendCtx(parent context.Context, attr slog.Attr) context.Context {
	// если первый параметр nil создаю контекст
	if parent == nil {
		parent = context.Background()
	}

	// если в контексте есть поле значение которого слайс []slog.Attr
	// добавляю в этот слайс атрибут и возвращаю новый контекст
	if v, ok := parent.Value(slogFields).([]slog.Attr); ok {
		v = append(v, attr)
		return context.WithValue(parent, slogFields, v)
	}

	// если нет создаю такой слайс, кидаю в него атрибут и создаю контекст с атрибутом
	v := []slog.Attr{}
	v = append(v, attr)
	return context.WithValue(parent, slogFields, v)
}

func main() {
	// создаю хендлер
	h := &ContextHandler{slog.NewJSONHandler(os.Stdout, nil)}
	// создаю логгер
	logger := slog.New(h)
	// делаю контекст со значением
	ctx := AppendCtx(context.Background(), slog.String("ctx_val1", "123"))
	// кидаю лог
	logger.InfoContext(ctx, "Logger With 1 Context Val1", slog.String("logger_arg", "998"))
	// {"time":"2024-07-04T09:39:43.072544375+04:00","level":"INFO","msg":"Logger With 1 Context Val1","logger_arg":"998","ctx_val1":"123"}

	// кидаю еще одно значение в контекст
	ctx = AppendCtx(ctx, slog.String("ctx_val2", "123"))
	// кидаю лог и в нем будет уже два значения из контекста
	logger.InfoContext(ctx, "Logger With 2 Context Val", slog.String("logger_arg", "998"))
	// {"time":"2024-07-04T09:39:43.072564277+04:00","level":"INFO","msg":"Logger With 2 Context Val","logger_arg":"998","ctx_val1":"123","ctx_val2":"123"}
}
