//>> ОРГАНИЗАЦИЯ ТЕСТОВ ПРОИЗВОДИТЕЛЬНОСТИ
// бенчмарки лежат в папке пакета
// имеют тот же пакет, соответственно имеют доступ ко всем функциям пакета
// имена файлов тестов заканчиваются на _test.go, типа benchmark_test.go
// бенчмарки - это функция с именем начинающимся на Benchmark и далее какое то имя, типа BenchmarkSort
// и принимают структуру типа testing.B
// в которой и реализованы все методы тестирования произволдительности

//>> ПОЛЯ СТРУКТУРЫ В
// N - определяет сколько раз запускать код подлежащий измерению

// _ ПРИМЕР БЕНЧМАРКА
func BenchmarkSort(b *testing.B) {
	size := 250
	data := make([]int, size)
	// здесь вызывается тестируемый код
	// делвается всегда в цикле, N раз
	// N всегда будет иметь какое то разное значение
	for i := 0; i < b.N; i++ {
		for j := 0; j < size; j++ {
			data[j] = rand.Int()
		}
		sortAndTotal(data)
	}
}

//>> ЗАПУСК БЕНЧМАРКОВ
//<< запуск всех бенчмарков без тестов
// go test -bench . -run notest
// goos: linux
// goarch: amd64
// pkg: playground
// cpu: AMD Ryzen 5 4600H with Radeon Graphics
// BenchmarkSort-12          104197             19666 ns/op
// PASS
// ok      playground      2.177s

// 12 - количество ядер
// 104197 - N
// 19666 ns/op - наносекунд на один цикл

//>> ОТКЛЮЧЕНИЕ НЕНУЖНОГО КОДА
// в тесте выше помимо тестированя функции sortAndTotal
// я генерирую случайный массив
// это занимает время, но это не то что мне нужно включать в подсчет
// этот код я могу исключить из расчета

func BenchmarkSort2(b *testing.B) {
	size := 250
	data := make([]int, size)
	//<< сброс таймера в начале нужного участка
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		//<< остановка таймера при ненужных действиях
		b.StopTimer()
		for j := 0; j < size; j++ {
			data[j] = rand.Int()
		}
		//<< запуск таймера где нужно
		b.StartTimer()
		sortAndTotal(data)
	}
}

// _ ВЫПОЛНЕНИЕ БЕНЧМАРКА ВНУТРИ БЕНЧМАРКА
// в одном тесте я могу запустить несколько тестов
func BenchmarkSort3(b *testing.B) {
	sizes := []int{10, 100, 250}
	for _, size := range sizes {
		// здесь будет запущено три теста,
		// и для каждого будут выведены результаты
		b.Run(fmt.Sprintf("Array Size %v", size), func(subB *testing.B) {
			data := make([]int, size)
			subB.ResetTimer()
			for i := 0; i < subB.N; i++ {
				subB.StopTimer()
				for j := 0; j < size; j++ {
					data[j] = rand.Int()
				}
				subB.StartTimer()
				sortAndTotal(data)
			}
		})
	}
}

// результаты будут примерно такими
// ...
// BenchmarkSort3/Array_Size_10-12                   865695              1319 ns/op
// BenchmarkSort3/Array_Size_100-12                  256398              4877 ns/op
// BenchmarkSort3/Array_Size_250-12                   99921             11086 ns/op
// ...