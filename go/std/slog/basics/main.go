package main

import (
	"context"
	"errors"
	"log/slog"
	"os"
)

// _ СУТЬ
// 	когда запущен сервер
// 	единственный способ понять что там происходит - это логи

// _ УРОВНИ ЛОГОВ
// 	логи жрут производительность
// 	все нужны в деве, но в проде нужны только значимые
// 	уровень логов задается числом, которое можно добавить в конфиги
//  и регулитровать какие логи, когда и куда хотим видеть
// 	в slog 4 уровня логов
// 		-4 - debug - подробная информация о состоянии приложения, при переходах состояния(только на этапе разработки)
// 		0 - info - минимальная информация о состоянии приложения, общими словами
// 		4 - warn - ошибка не в коде
// 		8 - error - ошибки в коде

// _ СУЩНОСТИ slog
// Logger - структура предоставляющая методы логирования
// Record - само сообщение логгера как единица
// Handler - интерфейс, его можно реализовать и логи будут выглядеть так как ты хочешь
// но есть и встроенные хендлеры смотри ниже

// _ ВОЗМОЖНОСТИ ПАКЕТА

func main() {
	// _ ЛОГИРОВАНИЕ БЕЗ СОЗДАНИЯ СОБСТВЕННОГО ЛОГГЕРА
	slog.Debug("Debug log") // здесь логов не будет так как минимальный уровень логов info для логгера по умолчанию
	slog.Info("Info log")   // 2024/07/03 09:56:33 INFO Info log
	slog.Warn("Warn log")   // 2024/07/03 09:56:33 WARN Warn log
	// можем передать аргументы
	slog.Error("Error log", "fuck", 12) // 2024/07/03 09:56:33 ERROR Error log fuck=12
	// можно логи выводить и так, функция Log печатает лог, получая контекст, уровень, сообщение и атрибуты
	slog.Log(context.Background(), slog.LevelDebug, "messege from Log")

	// _ СОЗДАНИЕ ЛОГОВ В СТРУКТУРИРОВАННОМ ВИДЕ встроенные хендлеры
	// ~ JsonLogger - создает хендлер формирующий лог в виде json чтроки
	handler := slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
		Level:     slog.LevelDebug, // минимальный уровень логов
		AddSource: false,           // добавляет поле source в запись лога(поля function, file, line)
	})
	// создаю новый логгер в замен стандартного
	logger := slog.New(handler)
	// в структурированный логгер можно добавлять поля
	logger.Info("Hi from json logger", "some", 16)
	// {"time":"2024-07-03T18:36:52.737079204+04:00","level":"INFO","msg":"Hi from json logger","some":16}

	// ~ NewTextHandler - создает хендлер формирующий текстовый лог
	handler2 := slog.NewTextHandler(os.Stdout, &slog.HandlerOptions{
		Level:     slog.LevelDebug, // минимальный уровень логов
		AddSource: true,            // добавляет поле source в запись лога(поля function, file, line)
	})
	// создаю новый логгер в замен стандартного
	logger2 := slog.New(handler2)
	logger2 = logger2.With("addTooAll", 12) // этот праметр будет во всех логах, всех уровней, например вписываем сервис или дата центр который оставил лог
	// в обычное сообщение я могу добавлять параметры
	logger2.Debug("Hi from text DEBUG logger", "some", 16)
	logger2.Info("Hi from text INFO logger", "some", 16)
	// time=2024-07-03T18:36:52.737+04:00 level=DEBUG source=/run/media/ang/store/dev/go/slog/log.go:60 msg="Hi from text DEBUG logger" addTooAll=12 some=16
	// time=2024-07-03T18:36:52.737+04:00 level=INFO source=/run/media/ang/store/dev/go/slog/log.go:61 msg="Hi from text INFO logger" addTooAll=12 some=16

	// _ УСТАНОВКА КАСТОМНОГО ЛОГГЕРА В КАЧЕСТВЕ ЛОГГЕРА ПО УМОЛЧАНИЮ
	// просто создаем любой логгер и ставим его по умолчанию
	slog.SetDefault(logger)
	// и он будет работать при вызове любого метода пакета slog
	slog.Info("Custom logger as Default")
	// {"time":"2024-07-03T18:36:52.737122505+04:00","level":"INFO","msg":"Custom logger as Default"}

	// _ ИСПОЛЬЗОВАНИЕ СИЛЬНО ТИПИЗИРОВАННЫХ АТРИБУТОВ
	// выше в функции логов я писал атрибуты, ключ значение
	// они слабо типизированные и возможны ошибки
	// ключи и значения можно передавать так
	// не забудь что здесь используется логер с JsonHandler по умолчанию
	slog.Info("Message from Srtong Typed Args", slog.String("name", "sam"), slog.Int("Integer", 56))
	// {"time":"2024-07-03T18:36:52.737128721+04:00","level":"INFO","msg":"Message from Srtong Typed Args","name":"sam","Integer":56}

	// _ ИСПОЛЬЗОВАНИЕ СПЕЦИАЛЬНЫХ МЕТОДОВ ДЛЯ ЛОГИРОВАНИЯ С АТРИБУТАМИ
	// добавление атрибутов любым методом выше к логу очень дорого, так как требует аллокации в памяти
	// если важна скорость, а она важна, иснужно вместо способов выше использовать спекциальнeю функцю
	// в которую передаем контекст, уровень логов, сообщение и атрибуты, И ЭТО СУПЕР БЫСТРО
	slog.LogAttrs(context.Background(), slog.LevelDebug, "LogAttrs message", slog.Int64("Int64", 65))
	// {"time":"2024-07-03T18:36:52.737135915+04:00","level":"DEBUG","msg":"LogAttrs message","Int64":65}

	// ~ групировка атрибутов
	// несколько атрибутов можно групировать вод одним ключем
	slog.LogAttrs(context.Background(), slog.LevelWarn, "Grouped LogAttrs message", slog.Group("person", slog.String("name", "John"), slog.Int("age", 16))) // {"time":"2024-07-03T18:02:51.335630621+04:00","level":"WARN","source":{"function":"main.main","file":"/run/media/ang/store/dev/go/slog/log.go","line":84},"msg":"Grouped LogAttrs message","person":{"name":"John","age":16}}
	// ! для TextHandler поля просто получат префикс
	// time=2023-02-24T12:06:20.249+01:00 level=INFO msg="image uploaded" id=23123 properties.width=4000 properties.height=3000 properties.format=jpeg

	// _ СОЗДАНИЕ И ПРИМЕНЕНИЕ ДОЧЕРНИХ ЛОГГЕРОВ c доп атрибутами
	// ~ дочерний логгер с атрибутами
	childLogger := logger.With(slog.Group("person", slog.String("name", "John"), slog.Int("age", 16)))
	// вызывая этот логгер поле logger будет в логах этого уровня
	childLogger.Info("Child Logger Message")
	// "time":"2024-07-03T18:36:52.737153026+04:00","level":"INFO","msg":"Child Logger Message","person":{"name":"John","age":16}}

	// ~ дочерний логгер с группой
	// можно групировать атрибуты логов с помощью slog.Group(выше)
	// а можно сразу создать логгер в котором все атрибуты будут под какой то группой
	logger3 := slog.New(handler).WithGroup("person")
	childLogger2 := logger3.With(slog.String("name", "Sam"), slog.Int("age", 16))
	// при этом все новые параметры будут добавлены в этку же группу
	childLogger2.Warn("childLogger2", slog.String("available_space", "900.1 MB"))
	// {"time":"2024-07-03T18:36:52.737162106+04:00","level":"WARN","msg":"childLogger2","person":{"name":"Sam","age":16,"available_space":"900.1 MB"}}

	// _ НАСТРОЙКА УРОВНЕЙ ЛОГИРОВАНИЯ
	// в slog 4 уровня логирования
	// 		-4 - debug
	// 		0 - info
	// 		4 - warn
	// 		8 - error
	// разбег в значениях чтобы я мог сконфигурировать свои уровни в дополнение к существующим

	// ~ конфигурация уровня дефолтного логгера
	slog.SetLogLoggerLevel(slog.LevelDebug)

	// ~ крнфигурация уровня логов
	// могу сделать как делаю выше, когда подключаю стандартные хендлеры, через HandlerOptions
	// могу сделать изменяемые уровни в ходе работы программы

	// для этого возвращаю ссылку текущий уровень
	logLevel := &slog.LevelVar{} // INFO по умолчанию
	// создаю структуру HandlerOptions и передаю туда уровни
	opts := &slog.HandlerOptions{
		Level: logLevel,
	}
	// и создаю хендлер
	handler3 := slog.NewJSONHandler(os.Stdout, opts)
	logger4 := slog.New(handler3)
	logger4.Info("Message")

	// и потом могу динамически менять уровень логов
	logLevel.Set(slog.LevelDebug)
	// {"time":"2024-07-03T22:20:30.994202461+04:00","level":"INFO","msg":"Message"}

	// _ КАСТОМНЫЕ УРОВНИ ЛОГИРОВАНИЯ
	// ! кастомные атрибуты доступны только в функциях Log и LogArrt
	// делаем константы уровней
	const (
		LevelTrace = slog.Level(-8)
		LevelFatal = slog.Level(12)
	)
	// создаем мапу подписей, для уровней
	var LevelNames = map[slog.Leveler]string{
		LevelTrace: "TRACE",
		LevelFatal: "FATAL",
	}
	// создаем опции хендлера
	opts2 := &slog.HandlerOptions{
		Level: LevelTrace,
		// ReplaceAttr - принимает функцию в которой сопоставляем имена атрибутов, не стоит задумываться
		ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			if a.Key == slog.LevelKey {
				level := a.Value.Any().(slog.Level)
				levelLabel, exists := LevelNames[level]
				if !exists {
					levelLabel = level.String()
				}

				a.Value = slog.StringValue(levelLabel)
			}

			return a
		},
	}
	// теперь можно использовать
	logger5 := slog.New(slog.NewJSONHandler(os.Stdout, opts2))

	ctx := context.Background()
	logger5.Log(ctx, LevelTrace, "Trace message")
	logger5.Log(ctx, LevelFatal, "Fatal level")
	// {"time":"2024-07-03T22:20:30.994215381+04:00","level":"TRACE","msg":"Trace message"}
	// {"time":"2024-07-03T22:20:30.994236752+04:00","level":"FATAL","msg":"Fatal level"}

	// _ ЛОГИРОВАНИЕ ОШИБОК
	// для типа error  не представлен хелпер для добавления атрибутов
	// типа slog.Int() или slog.String()
	// вместо этого нужно использовать slog.Any()
	err := errors.New("some error")
	logger.Error("Error logger", slog.Any("error", err))
	// {"time":"2024-07-04T12:35:06.726196109+04:00","level":"ERROR","msg":"Error logger","error":"some error"}
}
