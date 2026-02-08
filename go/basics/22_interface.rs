//>> ИНТЕРФЕЙСЫ

//>> ИСПОЛЬЗОВАНИЕ ИНТЕРФЕЙСА В ФУНКЦИЯХ
// когда нужна функция принимающая не важно какой тип
// главное предоставляющий определенный функционал
// для этого я 

// обьявляю интерфейс
type Vehicle interface{
    // описываю какой функионал буду использовать в функции
    move()
}

// создаю функцию которая будет реализовывать это функционал
// и указываю что значение должно быть типа интерфейса
func drive(vehicle Vehicle){
    vehicle.move()
}

// создаю структуры
type Car struct{ }
type Aircraft struct{}
 
// для каждой имплементирую интерфейс
func (c Car) move(){
    fmt.Println("Автомобиль едет")
}
func (a Aircraft) move(){
    fmt.Println("Самолет летит")
}
 
func main() {
     
    tesla := Car{}
    boing := Aircraft{}
    // и в функцию я могу пеердать любой тип реализующий этот интерфкейс
    drive(tesla)
    drive(boing)

    //! ЕСЛИ В ПОЛУЧАТЕЛЕ МЕТОДОВ СТРУКТУР БУДУТ УКАЗАТЕЛИ unc (c *Car) move()
    //! ТО В drive НЕ ПОЛУЧИТСЯ ПРЕДАТЬ ЗНАЧЕНИЕ
    //! ТОЛЬКО УКАЗАТЕЛЬ
    drive(&boing)
}

//>> ИСПОЛЬЗОВАНИЕ ИНТЕРФЕЙСА В СЛАЙСАХ
// так же я могу создать коллекции типа интрефейса
// и положить туда все что его реализует
// могу перебирать и вызывать функционал
arr:=[]Vehicle{Car{}, Aircraft{}}

//>> ИМПЛЕМЕНТАЦИЯ ИЛИ СООТВЕТСТВИЕ ИНТЕРФЕЙСУ
// чтобы структура соотвестсвовала интерфейсу, 
// нужно просто реализовать для нее все методы интерфейса 

//>> СРАВНЕНИЯ ИНТЕРФЕЙСОВ
// два значения интерфейса равны, 
// если они имеют одинаковый динамический тип и все их поля равны.

//>> УТВЕРЖЖЕНИЕ ТИПА ИНТЕРФЕЙСА ИЛИ TYPE CHECK
// expense.(Service)
// это типа мы говорим
// в переменной expense у нас лежит тип Service
// полезно для сравнений
// потому что утверждение типа может возвращать 
// значение и флаг соответствия  

// проверять тип интерфейса можно так
expenses := []Expense {
    Service {"Boat Cover", 12, 89.50, []string{} },
    Service {"Paddle Protect", 12, 8, []string{} },
    &Product { "Kayak", "Watersports", 275 },
}
for _, expense := range expenses {
    if s, ok := expense.(Service); ok {
        fmt.Println("Service:", s.description, "Price:",
        s.monthlyFee * float64(s.durationMonths))
    } else {
        fmt.Println("Expense:", expense.getName(),
        "Cost:", expense.getCost(true))
    }
}

// или так
for _, expense := range expenses {
    switch value := expense.(type) {
        case Service:
            fmt.Println("Service:", value.description, "Price:", value.monthlyFee *float64(value.durationMonths))
        case *Product:
            fmt.Println("Product:", value.name, "Price:", value.price)
        default:
            fmt.Println("Expense:", expense.getName(), "Cost:", expense.getCost(true))
    }
}

// я могу не присваивать значение в переменную если не планирую ее использование
for _, expense := range expenses {
    switch expense.(type) {
        case Service:
            // do some
        case *Product:
            // do some
        default:
            // do some
    }
}
//>> ИСПОЛЬЗОВАНИЕ ПУСТОГО ИНТЕРФЕЙСА
// пустой интерфейс говорит, что значение может быть любого типа
// но все это работает довольно долго
// и не стоит так делать
data := []interface{} {
    expense,
    Product { "Lifejacket", "Watersports", 48.95 },
    Service {"Boat Cover", 12, 89.50, []string{} },
    Person { "Alice", "London"},
    &Person { "Bob", "New York"},
    "This is a string",
    100,
    true,
}

//<< пустой интерфейс
var l interface{} = interface{}(nil)

//>> ИНТЕРФЕЙСЫ ТИПОВ
// создаются для дженериков и описывают возможные типы
// которые использует дженерик
type Number interface{
    int64 | float64
}

//<< основанные на типе производные типы
// знак тильда означает, что я могу передать как int64
// так и любой производный тип от float64
type Number interface{
    ~int64
}