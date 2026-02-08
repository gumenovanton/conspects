//>> ГЕНЕРАЦИЯ СЛУЧАЙНЫХ ЧИСЕЛ
// ~ Float32()
// Эта функция генерирует случайное значение float32 в диапазоне от 0 до 1.
fmt.Println(rand.Float32()) // 0.054110765

// ~ Float64()
// Эта функция генерирует случайное значение float64 в диапазоне от 0 до 1.
fmt.Println(rand.Float64()) // 0.49840272233587635

// ~ Int()
// Эта функция генерирует случайное int значение.
fmt.Println(rand.Int()) // 2397469377378314106

//<< Intn(max)
// Эта функция генерирует случайное int число меньше указанного значения, как описано после таблицы.
fmt.Println(rand.Intn(5)) // 4

//<< Uint32()
// Эта функция генерирует случайное значение uint32.
fmt.Println(rand.Uint32()) // 1899793135

//<< Uint64()
// Эта функция генерирует случайное значение uint64.
fmt.Println(rand.Uint64()) // 15234742453201746732

// ~ Shuffle(count, func)
// Эта функция используется для рандомизации порядка элементов, как описано после таблицы.
var names = []string{"Alice", "Bob", "Charlie", "Dora", "Edith"}
rand.Shuffle(len(names), func(first, second int) {
	names[first], names[second] = names[second], names[first]
})
fmt.Println(names) // [Bob Charlie Dora Edith Alice]