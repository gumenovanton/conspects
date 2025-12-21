//>> WORKER POOL
// когда у меня есть много задач я не могу решать их последовательно
// например сбросить статус сотни польщователей

// так же я не могу выполнять их все паралельно
// так как под каждую задачу я должен создать горутину 
// и каждая заюерет память, и если память нужна большая
// то так можно потерять всю память

// выход, создать workerPool
// на все процесы создать ограниченное количество горутин
// пусть 5
// после завершения задачи worker берется за следующую


func PooledWork(allData []model.SimpleData) {
	var wg sync.WaitGroup
	workerPoolSize := 100// на сколько задач

	// канал задач
	dataCh := make(chan model.SimpleData, workerPoolSize)

	// создаю горутины 5 нтук
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			// горутина ждет задачу
			// как получила делает, берет следующую
			for data := range dataCh {
				basic.Process(data)
			}
		}()
	}

	// где то кидаю задачи
	for i, _ := range allData {
		dataCh <- allData[i]
	}
	// закрываю канал
	close(dataCh)
	wg.Wait()
}


// Process
worker.PooledWork(allData)


//<< или можно раскидать все по структурам
// какаято таска
ype Task struct {
	ID     int
	Data   string
	Status bool
}

// пул
type WorkerPool struct {
	Queue        chan *Task
	WorkersCount int
	Result       chan *Task
}

// создание пула по количеству задач
func NewWorkerPool(workers int) *WorkerPool {
	return &WorkerPool{
		make(chan *Task, workers),
		workers,
		make(chan *Task),
	}
}
//созданеи горутин
func (wp *WorkerPool) Start() {
	for i := 0; i < wp.WorkersCount; i++ {
		go func() {
			wp.worker()
		}()
	}
}

// сам обработчик
func (wp *WorkerPool) worker() {
	task := <-wp.Queue
	log.Printf("Executing task %d: %s\n", task.ID, task.Data)
	task.Status = true
	wp.Result <- task
}

//добавление таски
func (wp *WorkerPool) AddTask(task *Task) {
	wp.Queue <- task
}

// ожидание
func (wp *WorkerPool) Wait(timer int) {
	<-time.NewTimer(time.Second * time.Duration(timer)).C
}

func main() {
	pool := NewWorkerPool(5)
	pool.Start()

	tasks := []*Task{
		{ID: 1, Data: "Task 1"},
		{ID: 2, Data: "Task 2"},
		{ID: 3, Data: "Task 3"},
	}

	go func() {
		for _, task := range tasks {
			pool.AddTask(task)
		}
	}()

	go func() {
		for v := range pool.Result {
			log.Printf("Task %d completed: %s status: %v\n", v.ID, v.Data, v.Status)
		}
	}()

	// Waiting for all tasks to complete
	pool.Wait(1)
}