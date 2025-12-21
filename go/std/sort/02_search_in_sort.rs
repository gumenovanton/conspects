//>> ПОИСК В ОТСОРТИРОВАННЫХ ДАННЫХ
//<< SearchInts(slice, val)
// ищет в int срезе, возвращает индекс найденного или позицию куда вставить
ints := []int{1, 2, 3, 4, 5}
fmt.Println(sort.SearchInts(ints, 2)) // 1

//<< SearchFloat64s(slice, val)
// ищет в float64 срезе, возвращает индекс найденного или позицию куда вставить
floats := []float64{1, 2, 3, 4, 5}
fmt.Println(sort.SearchFloat64s(floats, 4)) // 3

//<< SearchStrings(slice, val)
// ищет в string срезе, возвращает индекс найденного или позицию куда вставить
strings := []string{"a", "bb", "Ja", "l", "pd"}
sort.Strings(strings)
fmt.Println(strings)                          // [Ja a bb l pd]
fmt.Println(sort.SearchStrings(strings, "a")) // 1

//<< Search(count, testFunc)
// ищет в массиве любого типа по результату функции
// count - сколько элементов смотреть
a := []int{1, 3, 6, 10, 15, 21, 28, 36, 45, 55}
i := sort.Search(len(a), func(i int) bool { return a[i] >= 6 })
fmt.Println(i) // 2