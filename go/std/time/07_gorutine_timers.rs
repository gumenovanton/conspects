//>> ТАЙМЕРЫ
//<< структура time.Timer
// представляет собой запущеный таймер
// содержит
// С - канал который вернет timeTime
// Stop - останавливает таймер
// Reset - останавливает и запускает с новым значением

//<< NewTimer(duration)
// создает и запускает time.Timer
// здесь даю время на выполнение горутины
timer := time.NewTimer(time.Minute * 10)
go func() {
	time.Sleep(time.Second * 2)
	// тк горутина отработала, сбрасываю таймер
	timer.Reset(time.Second)
}()
// жду горутину
<-timer.C
fmt.Println("Горутина отработала")

//<< NewTicker(duration)
// аналог Tick
// но возвращает не канал а таймер
// что позволяет останавливать и переопределять повторяющийся таймер
names := []string{"Alice", "Bob", "Charlie", "Dora"}
ticker := time.NewTicker(time.Second / 10)
index := 0
for {
	<-ticker.C
	someChannel <- names[index]
	index++
	if index == len(names) {
		ticker.Stop()
		close(someChannel)
		break
	}
}