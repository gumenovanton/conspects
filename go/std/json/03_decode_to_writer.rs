//>> СОЗДАНИЕ ДЕКОДЕРА В READER
//<< NewDecoder(reader)
// декодирует json из reader
// возвращает Decoder, который используется для считывания данных из Reader
reader := strings.NewReader(`true "Hello" 99.99 200 99.99 200`)
decoder := json.NewDecoder(reader)

//>> МЕТОДЫ
//<< Decode(value)
// декодирует в value в тип переменной
var boolean bool
var str string
var num float64
var num2 int
decoder.Decode(&boolean)
decoder.Decode(&str)
decoder.Decode(&num)
decoder.Decode(&num2)                    // декодируется в int
fmt.Printf("%T: %v\n", boolean, boolean) // bool: true
fmt.Printf("%T: %v\n", str, str)         // string: Hello
fmt.Printf("%T: %v\n", num, num)         // float64: 99.99
fmt.Printf("%T: %v\n", num2, num2)       // int: 200

//<< DisallowUnknownFields()
// вызов данного метода даст ошибку
// если в JSON есть поля отсутствующие поля
// которых нет в структуре
// по умолчанию они просто игнорируются

//<< UseNumber()
// если декодировать в интерфейс. то все числа будут по умолчанию Float64
// можно кдекодировать в json.Number, вместо Float64 по умолчанию
decoder.UseNumber()
// и полученное значение из декодера можно сконвертить
// либо в Int64, либо в Float64, либо в строку
// но это по сути нах не нужно

var num3 json.Number
var num4 json.Number
decoder.Decode(&num3)
decoder.Decode(&num4)
n3 := num3.String()
n4, _ := num4.Int64()
fmt.Println(n3, n4) // 99.99 200

//>> ДЕКОДИРОВАНИЕ СОСТАВНЫХ ТИПОВ
//<< массивы
reader = strings.NewReader(`[10,20,30]`)
ints := make([]int, 3)
decoder = json.NewDecoder(reader)
decoder.Decode(&ints)
fmt.Println(ints) // [10 20 30]

//<< мапа
reader = strings.NewReader(`{"Kayak" : 279, "Lifejacket" : 49}`)
intsMap := make(map[string]int)
decoder = json.NewDecoder(reader)
decoder.Decode(&intsMap)
fmt.Println(intsMap) // map[Kayak:279 Lifejacket:49]

//<< структура
type Product struct {
	Name  string  `json:"name"`
	Price float64 `json:"price"`
}

type DiscountedProduct struct {
	*Product
	Discount float64 `json:"doscount"`
}

reader = strings.NewReader(`{"name":"Sow","price":66.25,"doscount":10}`)
prod := DiscountedProduct{&Product{}, 0}
decoder = json.NewDecoder(reader)
decoder.Decode(&prod)
fmt.Println(prod.Name)     // Sow
fmt.Println(prod.Price)    // 66.25
fmt.Println(prod.Discount) // 10

//<< JSON теги
	// `json:"-"` - пропуск поля
	// `json:"product,omitempty"` - пропуск не назначеных полей, по умолчанию им присвоится null
	// `json:",string"` - значение поля любого типа записать как строку, обрати внимание на запятую это второй параметр


//>> СОЗДАНИЕ ПОЛНОСТЬЮ НАСТРАЕВАЕМЫХ ДЕКОДИРОВОК
// могу декодировать структуру как хочу
// смотри PRO_GO стр 712