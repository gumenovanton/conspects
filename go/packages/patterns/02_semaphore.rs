//>> ПАТТЕРН СИМАФОР
// когда у меня есть много задач я не могу решать их последовательно
// например сбросить статус сотни польщователей

// так же я не могу выполнять их все паралельно
// так как под каждую задачу я должен создать горутину 
// и каждая заюерет память, и если память нужна большая
// то так можно потерять всю память

// выход, создать буферезированный канал 
// размером на необхдимое количество горутин
// создать цикл создающий по одной горутине на одну задачу
// но преред созданием горутины положить в канал пустую структуру
// а в каждой задаче вконце получать значение и канала, освобождая его
// таким образом после заполнения канала, 
// заблокиеруется основная горутина
// а после завершения первой цикл возобновится
// и создасться еще одна
// таким образом прерберуться все таски
// но активными будут только столько горутин
// какая емкость канала

//! СУТЬ ОГРАНИЧЕНИЕ ГОРУТИН КАНАЛОМ
//! СИМАФОР ЭТО КАНАЛ

func worker(id int, semaphore chan struct{}, results chan<- string) {
	// делаю что-то с данными и кидаю их в результат
    // так как эта горутина закончила работу
    // освобождаю место в канале
	results <- "any data"
	<-semaphore 
}

func main() {
    // задаю количество тасков
	numWorkers := 10
    // создаю семофор с таким размером буфера, скольким горутином я позволю работать одновременно
	semaphore := make(chan struct{}, 3)
    // канал для результатов
	results := make(chan string, numWorkers)

    // открываю горутину для каждой задачи
	for i := 1; i <= numWorkers; i++ {
		semaphore <- struct{}{} // Занятие места в канале
		// в итоге когда канал забит цикл залочится и не будет создавать горутины
		go worker(i, semaphore, results)
	}
    // вывод результатаов
	for i := 1; i <= numWorkers; i++ {
		// тут освобождаю горутины, давая создаваться другим горутинам
		result := <-results
		fmt.Println(result)
	}

	close(semaphore)
	close(results)
}

//<< ЛИБО С ПОМОЩЬЮ СТРУКТУР
type Semaphore struct {
	C chan struct{}
}

// уменшаю количество свободных мест в канале
func (s *Semaphore) Acquire() {
	s.C <- struct{}{}
}

// увеличиваю количество свободных мест в канале
func (s *Semaphore) Release() {
	<-s.C
}

type resultWithError struct {
	User users.User
	Err  error
}

func DeactivateUsers(usrs []users.User, gCount int) ([]users.User, error) {
	// создаем семафор и передаем ему канал с размером буфера равным ограничению на количество одновременно выполняемых горутин
	sem := Semaphore{
		C: make(chan struct{}, gCount),
	}

	// канал для сбора результатов
	outputCh := make(chan resultWithError, len(usrs))
	// канал для оповещения горутин, что мы больше не ждем их выполнения
	sgnlCh := make(chan struct{})

	output := make([]users.User, 0, len(usrs))

	for _, v := range usrs {
		go func(usr users.User) {
			sem.Acquire()
			defer sem.Release()

			err := usr.Deactivate()

			// если ловим закрытие сигнального канала, то завершаем функцию
			select {
			case outputCh <- resultWithError{
				User: usr,
				Err:  err,
			}:
			case <-sgnlCh:
				return
			}
		}(v)
	}

	// ждем и собираем результаты
	// либо мы получим все, либо выйдет ошибка, по которой мы перестанем читать
	for i := len(usrs); i > 0; i-- {
		res := <-outputCh
		if res.Err != nil {
			close(sgnlCh)
			return nil, fmt.Errorf("an error occurred: %w", res.Err)
		}

		output = append(output, res.User)
	}

	return output, nil
}