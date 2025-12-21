//>> МЕТОДЫ
// это функции привязанные к структуре

//<< обьявление
type Product struct {
    name, category string
    price float64
}

// при инициализации нужно указать получатея(receiver)
// в качестве получателя(receiver) указываем ссылку на структуру
// к которой привязываем метод
// если указать значение, то метод будет работать с копией
// просто всегда ставь указатель
// можно указать параметры и возвращаемые значения
func (product *Product) printDetails(){
    fmt.Println("Name:", product.name, "Category:", product.category, "Price", product.price)
}

//<< вызов метода
products := Product {"Kayak", "Watersports", 275}
product.printDetails()

//>> ОПРЕДЕЛЕНИЕ МЕТОДАВ ДЛЯ ВСЕВДАНИМА
// метод можно прилепить к любому типу в пределах пакета
type ProductList []Product
func (products *ProductList) calcCategoryTotals() map[string]float64 {
}

//>> МЕТОДЫ СТРУКТУР ПРИ ВСТРАИВАНИИ
// при встраивании стуктура так же унаследует все методы вложенной структуры
type Contact struct {
}

func (cont *Contact) print() {
	fmt.Println("contact")
}

type Person struct {
	Contact
}

func main() {
    // сработает как для переменной
	john := Person{Contact: Contact{}}
	john.print()
    // так и для указателя
	johnPtr := &Person{Contact: Contact{}}
	johnPtr.print()
}


