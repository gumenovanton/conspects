//>> КОДИРОВАНИЕ И ДЕКОДИРОВАНИЕ В И ИЗ СРЕЗА
// можно кодировать и декодировать любые типы
// простые и составные
// здесь пример на структуре

type Product struct {
	Name  string  `json:"name"`
	Price float64 `json:"price"`
}

type DiscountedProduct struct {
	*Product
	Discount float64 `json:"doscount"`
}

//<< Marshal(value)
// кодирует value в JSON стез байт
structure := DiscountedProduct{&Product{"samsung", 10}, 10}
slice, _ := json.Marshal(structure)
fmt.Println(string(slice)) // {"name":"samsung","price":10,"doscount":0}

//<< Unmarshal(byteSlice, &val)
// декодирует срез байт в JSON val
str := DiscountedProduct{&Product{}, 0}
json.Unmarshal(slice, &str)
fmt.Println(str.Name)     // samsung
fmt.Println(str.Price)    // 10
fmt.Println(str.Discount) // 10