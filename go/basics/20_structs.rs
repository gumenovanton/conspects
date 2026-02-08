//>> СТРУКТУРЫ
// пользовательский тип данных обьединяющий в себе несколько типов данных

//<< обьявление
type Product struct {
    name, category string
    price float64
}

//<< инициализация
// если я не инициализирую какое то поле
// то это поле будет инициализировано значением по умолчанию

kayak := Product {
    name: "Kayak",
    category: "Watersports",
    price: 275,
}

//<< позиционная инициализация
// имена полей можно не указывать при сохранении порядка
var kayak = Product { "Kayak", "Watersports", 275.00 }

//<< доступ к полям
fmt.Println(kayak.name, kayak.category, kayak.price)

//>> СОЗДАНИЕ УКАЗАТЕЛЯ НА СТРУКТУРУ С ПОМОЩЬЮ NEW
// так можно создать указатель на структуру с нулевыми значениями
var lifejacket = new(Product)
// это то же самое что и 
var lifejacket = &Product{}

//>> СРАВНЕНИЕ СТРУКТУР
// структуры можно сравнивать если можно сравнить их поля
// если одно из полей например срез, то сравнивать уже нельзя

//>> ЯВНОЕ ПРИОБРАЗОВАНИЕ СТРУКТУР
// возможно только если имена полей 
// их порядок и типы одинаковые 
prod := Product { name: "Kayak", category: "Watersports", price: 275.00 }
item := Item { name: "Kayak", category: "Watersports", price: 275.00 }
fmt.Println(prod == Product(item))

//>> АНОНИМНЫЕ СТРУКТУРЫ
// структуры без имени
// можно например пеердавать в функции
func writeName(val struct {name, category string price float64}) {
    fmt.Println("Name:", val.name)
}

//>> ДОСТУП К ПОЛЯМ УКАЗАТЕЛЕЙ НА СТРУКТУРУ
// делаем просто по значению 
// го сам разименует 
func calcTax(product *Product) {
    if (product.price > 100) {
        product.price += product.price * 0.2
    }
}
    
//>> КОНСТРУКТОР СТРУКТУРЫ
// в го принято писать функцию new
// которая возвращает указатель на структуру
// чтобы не дублировать значение созданной в теле функции структуры
// при возврате значения
//! так делают конструкторы, 
//! потому что они могут инициализировать даже приватные поля 
func NewProduct(name, category string, price float64) *Product {
    return &Product{name, category, price}
}

//>> ПОЛЯ УКАЗАТЕЛИ
// может быть указателем
// но если это поле не проинициализировать
// при попытки обращения к нему будет ошибка
// так как указателем будет nil 
type Product struct {
    name, category string
    price float64
    *Supplier
}
// делать инициализацию нужно хотябы так
var prod Product = Product{ Supplier: &Supplier{}}



