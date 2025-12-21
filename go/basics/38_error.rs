//>> ERROR
// Тип TRROR в го это интерфейс

//>> СОЗДАНИЕ КАСТОМНОЙ ОШИБКИ
// кастомная ошибка это всегда структура со строкой
// которая реализует интерфейс Error
// это и будет ошибка
// ее я смогу пихать в любой метод который требует ошибки
// например в panic
type CategoryError struct {
    requestedCategory string
}
// и реализуем интерфейс Error
func (e *CategoryError) Error() string {
    return "Category " + e.requestedCategory + " does not
    exist"
}

//>> ОШИБКИ В КАНАЛАХ
// как правило лучше добавлять поле ошибки в структуру сообщения
// и так возвращать ее из канала
// и работать с ней при получении
type ChannelMessage struct {
    Category string
    Total float64
    *CategoryError
}

//>> ПРОСТОЕ СОЗДАНИЕ ОШИБКИ
// создать и вернуть ошибку я могу так
// она будет иметь тип error и ее можно пихать и возвращать везде где нужен тип error
err = errors.New("Cannot find category")

// или так
err = fmt.Errorf("Cannot find category: %v", category)

//>> PANIC
// завершает работу приложения с выводом ошибки в поток ошибок
// програмист сам решает завершить приложение ошибкой или нет
panic(message.CategoryError)

// когда вызывается паника
// поток прикращает работу и вызываются все defer
// вверх по стеку

//>> ВОССТАНОВЛЕНИЕ ПОСЛЕ ПАНИКИ
// чтобы не дать панике опдняться вверх по стеку
// нужно через defer вызвать встроенную функцию recovery 

//! НЕЛЬЗЯ ВОССТАНОВИТЬСЯ ПОСЛЕ STACKOVERFLOW И КОНКУРЕНТНОЙ ЗАПИСИ В МАПУ

// пусть у меня есть функция которая может запаниковать
func iWillPanic() {
    // в defer делаем recovery
	defer func() {
		if arg := recover(); arg != nil {
			if err, ok := arg.(error); ok {
				fmt.Println("Error:", err.Error())
			} else if str, ok := arg.(string); ok {
				fmt.Println("Message:", str)
			} else {
				fmt.Println("Panic recovered")
			}
		}
	}()
    // и здесь вызываем панику
    // после этой строки функция улетит в конец для вызова defer
	panic(errors.New("ooo shit"))
    // код здесь никогда не исполнится
    fmt.Println("This will never run")
}

func main() {
    // здесь мы паникуем
    // после паники внутри iWillPanic код сразу выполнит defer
    // вострановится
	iWillPanic()
    // и программма продолжит работать с этой строки
	fmt.Println("I am working")
}

//<< ВАЖНОЕ ЗАМЕЧАНИЕ 
// если все это поместить в main 
// программа улетит на defer
// и должна сразу вернуться во внешний блок
// но его нет
func main() {
	defer func() {
		if arg := recover(); arg != nil {
			fmt.Println("Panic recovered")
		}
	}()
	panic(errors.New("ooo shit"))// так как здесь функция перескакивает на defer
	fmt.Println("This will never run")
}

//>> ВОСССТАНОВЛЕНИЕ ПОСЛЕ ПАНИКИ В ГОРУТИНАХ
// в горутинах та же история что выше в main
// по этому весь код после паники не выполнится
// и если я хочу продолжить выполнение в горутине
// например закрыть канал например
// делать это нужно в rercovery в теле функции горутины
defer func() {
    if arg := recover(); arg != nil {
        fmt.Println(arg)
        close(outChan)
    }
}()