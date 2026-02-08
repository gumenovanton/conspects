// _ ФУНКЦИИ ПАКЕТА log БАЗОВАЯ ПЕЧАТЬ В Logger
	// по умолчанию логи выводятся в консоль

	//<< log.Output(depth, string)
	// базовая функция пакета
	// выводит строку в Logger
	// depth ставить 2
	// принимает строку а не интерфейс, по этому самая эффективная
	log.Output(2, "Output") // 2024/08/26 17:32:35 Output

	//>> ХЕЛПЕРЫ ДЛЯ ПЕЧАТИ В Logger
	//<< log.Print(...vals), log.Println(...vals), log.Printf(template, ...vals)
	// простой вывод в logger
	log.Print("Message") // 2024/08/26 17:06:01 Message
	// простой вывод в logger, с добавлением перехода на новую строку
	log.Println("Message") // 2024/08/26 17:06:01 Message
	// форматированный вывод в logger
	log.Printf("Log text: %s", "Message") // 2024/08/26 17:11:28 Log text: Message

	//<< log.Fatal(...vals), log.Fatalln(...vals), log.Fatalf(template, ...vals)
	// вывод в Logger и завершение приложения
	// log.Fatal("Fatal Error")

	// вывод в Logger со знаком новой строки и завершение приложения
	// log.Fatalln("Message")

	// форматизированный вывод в Logger и завершение приложения
	// log.Fatalf("Log text: %s", "Message")

	//<< log.Panic(...vals), log.Panicln(...vals), log.Panicf(template, ...vals)
	// вывод в Logger и вызывает панику
	// log.Panic("Fatal Error")

	// вывод в Logger со знаком новой строки и вызывает панику
	// log.Panicln("Message")

	// форматизированный вывод в Logger и вызывает панику
	// log.Panicf("Log text: %s", "Message")

	//>> ФОРМИРОВАНИЕ СТИЛЯ СООБЩЕНИЯ
	// по умолчанию в сообщение лога включен флаг даты и времени
	// я могу добавлять флаги
	// могу добавлять и смотреть префиксы

	//<< ПРЕФИКС
	// установка
	log.SetPrefix("INFO ")
	// просмотр
	fmt.Println(log.Prefix())                // INFO
	log.Output(2, "Log Message With Prefix") // INFO 2024/08/26 17:45:05 Log Message With Prefix

	//<< ФЛАГИ
	// Ldate - дата
	// Ltime - время
	// Lmicroseconds - микросекунды
	// Llongfile - полный путь файла и номер строки где был вызван лог в коде
	// Lshortfile - только имя файла и номер строки где был вызван лог в коде
	// LUTC - поставить UTC вместо локального часового пояса
	// Lmsgprefix - преемещает префикс к сообщению
	// LstdFlags - значение по умолчанию, включает Ldate и Ltime

	//<< УСТАНОВКА ФЛАГОВ
	// установка
	log.SetFlags(log.Lshortfile | log.Ltime)
	// вызывает значения флагов, они int
	fmt.Println(log.Flags())                // 18
	log.Output(2, "Log Message With Flags") // proc.go:272: INFO Log Message With Flags

	//>> УСТАНОВКА Writer ДЛЯ ЛОГИРОВАННИЯ
	// по умолчанию логи пишуться в консоль
	// я могу перенаправить их например в файл
	file, _ := os.OpenFile("logs.txt", os.O_CREATE|os.O_APPEND|os.O_RDWR, 0666)
	// устанавливаю назначение
	log.SetOutput(file)
	err := log.Output(2, "Log in File")
	fmt.Println(err)

	//>> СОЗДАНИЕ СОБСТВЕННОГО ЛОГГЕРА
	// логгер по умолчанию нет смысла использовать
	// у меня могут быть разные логгеры
	// для каждого уровня логов я могу задать префикс и назначение логов
	// по этому не стоит, так же и для безопасности, использовать стандартный логгер
	// а нужно создавать свой

	//<< New(writer, prefix, flags)
	// возвращает логгер
	file, _ := os.OpenFile("logs.txt", os.O_CREATE|os.O_APPEND|os.O_RDWR, 0666)
	logger := log.New(file, "INFO ", log.Ltime)
	logger.Output(2, "My Logger Message")