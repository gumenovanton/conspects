//>> СОРТИРОВКА СРЕЗОВ
//<< Float64s(slice)
// сортирует float64 срез
floats := []float64{279, 48.95, 19.50}
fmt.Println(floats) // [279 48.95 19.5]
sort.Float64s(floats)
fmt.Println(floats) // [19.5 48.95 279]

//<< Float64sAreSorted(slice)
// можно ли сортировать срез
fmt.Println(sort.Float64sAreSorted(floats)) // true

//<< Ints(slice)
// сортирует int срез
ints := []int{9, 4, 2, -1, 10}
fmt.Println(ints) // [9 4 2 -1 10]
sort.Ints(ints)
fmt.Println(ints) // [-1 2 4 9 10]

//<< IntsAreSorted(slice)
// можно ли сортировать срез
fmt.Println(sort.IntsAreSorted(ints)) // true

//<< Strings(slice)
// сортирует string срез
strings := []string{"Kayak", "Lifejacket", "Stadium"}
fmt.Println(strings)
if !sort.StringsAreSorted(strings) {
	sort.Strings(strings)
	fmt.Println(strings)
} else {
	fmt.Println(strings)
}

//<< StringsAreSorted(slice)
// можно ли сортировать срез
fmt.Println(sort.StringsAreSorted(strings)) // true

//>> СОРТИРОВКА СЛАЙСОВ ПОЛЬЗОВАТЕЛЬСКИХ ТИПОВ
// чтобы сортировать слайсов обьектов 
// нужно реализовать интерфейс с названием Interface
// а именно его три метода
// Len() - количество элементов которые будут отсортированны
// Less(i,j) - возвратит екгу если i должен стоять перед j
// Swap(i,j) - меняет местами элементы с указанными индексами

// после реализации этих методов структуры можно сортировать
type Product struct {
	Name  string
	Price float64
}
// нужно обьявить тип последовательности
type ProductSlice []Product

func ProductSlices(p []Product) {
	sort.Sort(ProductSlice(p))
}
func ProductSlicesAreSorted(p []Product) {
	sort.IsSorted(ProductSlice(p))
}
func (products ProductSlice) Len() int {
	return len(products)
}
func (products ProductSlice) Less(i, j int) bool {
	return products[i].Price < products[j].Price // здесь определяем по какому значению сортируем
}
func (products ProductSlice) Swap(i, j int) {
	products[i], products[j] = products[j], products[i]
}

func main() {
	products := []Product{
		{"Kayak", 279},
		{"Lifejacket", 49.95},
		{"Soccer Ball", 19.50},
	}
	ProductSlices(products)
	fmt.Println(products) // [{Soccer Ball 19.5} {Lifejacket 49.95} {Kayak 279}]

}