package main

// _ СОЗДАНИЕ ПОЛЬЗОВАТЕЛЬСКИХ ОБРАБОТЧИКОВ
// чтобы это сделать, нужно создать структуру, которая должна реализовывать интерфейс Handler
// 	type Handler interface {
// 		Enabled(context.Context, Level) bool
// 		Handle(context.Context, r Record) error
// 		WithAttrs(attrs []Attr) Handler
// 		WithGroup(name string) Handler
// 	}

// Enabled() определяет, следует ли обрабатывать или выбрасывать запись журнала в зависимости от ее уровня. Для принятия решения также может использоваться context.
// Handle() обрабатывает каждую запись журнала, отправленную обработчику. Она вызывается только в том случае, если Enabled() возвращает true.
// WithAttrs() создает новый обработчик из существующего и добавляет в него указанные атрибуты.
// WithGroup() создает новый обработчик из существующего и добавляет в него указанное имя группы так, чтобы это имя квалифицировало последующие атрибуты.

// _ ЗАПУСТИ И УВИДЕШЬ ВЫВОД
import (
	"context"
	"encoding/json"
	"io"
	"log"
	"log/slog"
	"os"

	"github.com/fatih/color"
)

type PrettyHandlerOptions struct {
	SlogOpts slog.HandlerOptions
}

type PrettyHandler struct {
	slog.Handler
	l *log.Logger
}

func (h *PrettyHandler) Handle(ctx context.Context, r slog.Record) error {
	level := r.Level.String() + ":"

	switch r.Level {
	case slog.LevelDebug:
		level = color.MagentaString(level)
	case slog.LevelInfo:
		level = color.BlueString(level)
	case slog.LevelWarn:
		level = color.YellowString(level)
	case slog.LevelError:
		level = color.RedString(level)
	}

	fields := make(map[string]interface{}, r.NumAttrs())
	r.Attrs(func(a slog.Attr) bool {
		fields[a.Key] = a.Value.Any()

		return true
	})

	b, err := json.MarshalIndent(fields, "", "  ")
	if err != nil {
		return err
	}

	timeStr := r.Time.Format("[15:05:05.000]")
	msg := color.CyanString(r.Message)

	h.l.Println(timeStr, level, msg, color.WhiteString(string(b)))

	return nil
}

func NewPrettyHandler(
	out io.Writer,
	opts PrettyHandlerOptions,
) *PrettyHandler {
	h := &PrettyHandler{
		Handler: slog.NewJSONHandler(out, &opts.SlogOpts),
		l:       log.New(out, "", 0),
	}

	return h
}

func main() {
	opts := PrettyHandlerOptions{
		SlogOpts: slog.HandlerOptions{
			Level: slog.LevelDebug,
		},
	}
	handler := NewPrettyHandler(os.Stdout, opts)
	logger := slog.New(handler)
	logger.Debug(
		"executing database query",
		slog.String("query", "SELECT * FROM users"),
	)
	logger.Info("image upload successful", slog.String("image_id", "39ud88"))
	logger.Warn(
		"storage is 90% full",
		slog.String("available_space", "900.1 MB"),
	)
	logger.Error(
		"An error occurred while processing the request",
		slog.String("url", "https://example.com"),
	)
}
