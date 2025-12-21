//>> sync.Pool
// когда нужно постоянно выделять память и стирать
// сборщик мусора постоянно запускается и жрет ресурсы
// вместо выделения памяти
// испольуй sync.Pool
// Создание пула.

//<< создание пула
// пул создается под опрееленный тип
var dataPool = sync.Pool{
    New: func() any {
       return make([]int, 0, 10000)
    },
}

// Работа с пулом.
func processPool() {
    data := dataPool.Get().([]int)
    // Некоторая обработка данных
    for i := 0; i < 10000; i++ {
       data = append(data, i)
    }

    // Очистка.
    data = data[:0]
    dataPool.Put(data)
}