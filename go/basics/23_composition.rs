//>> КОМПОЗИЦИЯ - ВСТРАИВАНИЕ СТРУКТУР
// полем структуры может быть другая структура или несколько
// но поля не должны конфликтовать
type Contact struct {
	Email string
	Phone string
}

func (c Contact) Print() {
	fmt.Println("Print from Contact")
}

func (c *Contact) PrintPtr() {
	fmt.Println("PrintPtr from Contact")
}

// и если я вставлю ее без указания имени поля
// все ее поля и методы продвинутся в кореную структуру
// она как бы встроится
type Person struct {
	Name string
	Age  int
	*Contact // здесь может быть и просто значение, не указатель
}

func main() {
	// но инициализировать нужно все равно так
	john := Person{
		"john",
		25,
		&Contact{
			"google@google.com",
			"+79173297729",
		},
	}
    // и я могу обращаться к полю встроенной структуры напрямую
	fmt.Println(john.Email) // google@google.com
	// и любой метод смогу вызвать так
	john.Print() // Print from Contact
	john.PrintPtr() // PrintPtr from Contact
}

//<< но если я задам поле
type Person struct {
	Name string
	Age  int
	Contact Contact 
}

// то встраивания не произойдет
// и достучаться до поля смогу только так
fmt.Println(john.Contact.email)

//>> КОМПОЗИЦИЯ С ИСПОЛЬЗОВАНИЕМ ИНТЕРФЕЙСА
// полем структуры может быть тип интерфейса
type Contacter interface {
	Print()
	PrintPtr()
}

type Contact struct {
	Email string
	Phone string
}

//! ВАЖНЫЙ НЮАНС
// если я указываю ресиверы как указатели
// то в поле интерфейс структуры я могу передать только указатель
func (c *Contact) Print() {
	c.Phone = "88888888"
	fmt.Println(c.Phone)
}

func (c *Contact) PrintPtr() {
	fmt.Println("PrintPtr from Contact")
}

type Person struct {
	Name string
	Age  int
    // здесь не слова про указатель, 
    // но значением может быть только указатель 
    // так как ресиверы в методах через указатель
	Contacter
} 

func main() {
	// здесь передаю именно адрес структуры удовлетворяющей интерфейс
	john := Person{
		"john",
		25,
		&Contact{
			"google@google.com",
			"+79173297729",
		},
	}
	// и я не смогу получить доступ к полям и методам структуры
	fmt.Println(john.Email)// ошибка
	// но смогу вызвать любой метод интерфейса
    // и там будут доступны и поля и методы структуры
	john.Print()    // 88888888
	john.PrintPtr() // PrintPtr from Contact
}

//<< нюансы ресиверов
// если ресивер по значению
// то в поле типа интерфейса я могу передать и ссылку на обьект 
// и обьект по значению
// в любом случае будут доступны только методы
// но внутреннее состояние не будет доступно
// только из методов интерфейсов