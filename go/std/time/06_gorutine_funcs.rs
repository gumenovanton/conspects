//>> ФУНКЦИИ ВРЕМЕНИ ДЛЯ ГОРУТИН

//<< Sleep(duration)
// останавливает горутину на duration
time.Sleep(time.Second * 3)
fmt.Println("i woke up")

//<< After(duration)
// возвращает канал, который блокируется на указанное время, а затем возвращает Time
// можно использовать как разовое оповещение
select {
    case news := <-NEWS_CHAN:
        fmt.Println(news)
    case <-time.After(time.Hour):
        fmt.Println("Превышено время ожидания")
}

//<< Tick(duration)
// возвращает канал который периодически отправляет Time
// можно использовать как переодическое обовещение
// представляет из себя бесконечный таймер
go func() {
	for now := range time.Tick(time.Minute) {
		fmt.Println(now, statusUpdate())
	}
}()

// или так
names := []string{"Alice", "Bob", "Charlie", "Dora"}
tickChannel := time.Tick(time.Second)
index := 0
for {
	<-tickChannel
	someChannel <- names[index]
	index++
	if index == len(names) {
		index = 0
	}
}

//<< AfterFunc(duration, func)
// выполняет указанную функцию в своей собственной горутине
// результат Timer, при нажатии Stop можно отменить функцию до истечения времени
// применяется как триггер функции при ее долгом исполнении
func Foo() {
    timer = time.AfterFunc(time.Minute, func() {
        log.Println("Foo выполняется более минуты.")
    })

    // ВАЖНО если Foo закончится, но таймер будет работать и не будет остановлен
    // то горутина уснет, пока не закончится таймер
    // по этому всегда останавливаем таймер вконце
    defer timer.Stop()

    // Выполняем какую-либо долгую работу
}