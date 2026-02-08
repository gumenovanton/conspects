//>> ПАТТЕРН НАБЛЮДАТЕЛЬ
// когда есть данные при обновлении которых
// все кто от них зависят должны обновиться

//>> РЕАЗИЗАЦИЯ
//<< создаем интерфейс Consumer 
// потребитель, тот кто подписывается

type Consumer interface {
    // обновление состояния потребителя, 
    // получаем имя издатель
    // чтобы можно было как то регулиировать работу метода Update
	Update(pubName string)
    // отдаем имя потребителя, 
    // чтобы издатель имел ключ для сохранения получателя
    // ну и чтобы как то регулировать функцию оповещени издптеля 
	GetName() string 
}

//<< создаем интерфейс Publisher
// издатель, тот кто расшаривает данные

type Publisher interface {
    // подписываю получателя
	Subsctybe(Consumer)
    // отписываю получателя
	UnSubscribe(Consumer)
    // уведомляю всех подписчиков
	NotifyListeners()
}

//<< создаю струтуру издателя
// и реализую интерфейс Publisher

type Sensors struct {
	Name      string
	LoadCell  int64
	Encoder   int64
	Consumers map[string]Consumer
}

func (s Sensors) Subsctybe(consumer Consumer) {
	s.Consumers[consumer.GetName()] = consumer
}

func (s Sensors) UnSubsctybe(consumer Consumer) {
	delete(s.Consumers, consumer.GetName())
}

func (s Sensors) NotifyListeners() {
	for _, consumer := range s.Consumers {
		consumer.Update(s.Name)
	}
}

// при изменении состояния 
// нужно просто вызвать NotifyListeners

//<< создаю структуру получателя


