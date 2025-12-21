//>> СОЗДАНИЕ ЭНКОДЕРА В WRITER
//<< NewEncoder(writer)
// кодирует JSON в указанный writer.
// возвращает Encoder, который используется для записи в Writer
file, _ := os.OpenFile(`4.txt`, os.O_RDWR|os.O_APPEND, 6660)
encoder := json.NewEncoder(file)

//>> МЕТОДЫ Encoder
//<< Encode(val)
// кодирует указанное значение как JSON и записывает его в Writer.
//<< SetEscapeHTML(on)
// принимает bool аргумент, который, если он равен true, кодирует символы,
// экранирование которых в HTML было бы опасным. По умолчанию эти символы экранируются.
//<< SetIndent(prefix, indent)
// Этот метод задает префикс и отступ, которые применяются к имени каждого поля в
// выходных данных JSON.
encoder.Encode(true)    // true
encoder.Encode("hello") // "hello"
encoder.Encode(99.99)   // 99.99
encoder.Encode(200)     // 200
encoder.Encode('Ё')     // 1025
// слайс
encoder.Encode([]string{"Kayak", "Lifejacket", "Soccer Ball"}) // ["Kayak","Lifejacket","Soccer Ball"]
// масив
encoder.Encode([3]int{10, 20, 30}) // [10,20,30]
// мапа
encoder.Encode(map[string]float64{"Kayak": 279, "Lifejacket": 49.95}) // {"Kayak":279,"Lifejacket":49.95}
	
//>> КОДИРОВАНИЕ СТРУКТУР
// в структурах можно писать теги 
// которые показывают как нужно отображать имена в JSON
type Product struct {
	Name  string  `json:"name"`
	Price float64 `json:"price"`
}

type DiscountedProduct struct {
	*Product
	Discount float64 `json:"doscount"`
}

dProd := DiscountedProduct{
    &Product{"Sow", 66.25},
    10,
}

encoder.Encode(dProd) // {"name":"Sow","price":66.25,"doscount":10}

//<< JSON теги
	// `json:"-"` - пропуск поля
	// `json:"product,omitempty"` - пропуск не назначеных полей, по умолчанию им присвоится null
	// `json:",string"` - значение поля любого типа записать как строку

//>> СОЗДАНИЕ ПОЛНОСТЬЮ НАСТРАЕВАЕМЫХ КОДИРОВОК
// могу кодировать структуру как хочу
// смотри PRO_GO стр 669