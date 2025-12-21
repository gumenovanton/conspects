//>> ЗАПРОСЫ К БАЗЕ ДАННЫХ НА ПОЛУЧЕНИЕ ДАННЫХ
//<< методы запросов
// Query(query, ...args) - возвращает Rows c результатами
// QueryRow(query, ...args) - возвращает Row c первым результатом
// Exec(query, ...args) - выполняет запрос не возвращающий данные, возвращает Result

//<< доступ к данным Rows
// Next() - переход к следующему значению
// NextResultSet() - переход к следующему набору значений
// Scan(...targets) - сканирование результатов в переменные
// Close() - закрытие перебора значений, когда остальные результаты не нужны

//<< Query(query, ...args)
db, _ := sql.Open("sqlite3", "products.db")
rows, _ := db.Query("SELECT * FROM Products")

//<< Next()
// перебираю результаты
for rows.Next() {
	var id, category int
	var name string
	var price float64
	//<< Scan(...targets)
	// пишу результат в переменные
	// чувствителен к типу
	rows.Scan(&id, &name, &category, &price)
	fmt.Printf("Row: %v %v %v %v\n", id, name, category, price)
}

//>> ПРАВИЛА СОПОСТАВЛЕНИЯ ТИПОВ
// SQL cтроки, числовые и логические значения могут быть сопоставлены с их аналогами в Go
// Числовые и логические типы SQL можно сканировать в строки Go
// Строки SQL могут быть просканированы в числовые типы Go
// Значения времени SQL можно сканировать в строки Go или значения *time.Time.

//>> СКАНИРОВАНИЕ ЗНАЧЕНИЙ В СТРУКТУРУ

type Product struct {
	Id       int
	Name     string
	Category int
	Price    float64
}

products := []Product{}
for rows.Next() {
	p := Product{}
	rows.Scan(&p.Id, &p.Name, &p.Category, &p.Price)
	products = append(products, p)
}
fmt.Println(products) // [{1 Kayak 1 279} {2 Lifejacket 1 48.95} {3 Soccer Ball 2 19.5} {4 Corner Flags 2 34.95}]

//>> ЗАПОЛНИТЕЛИ
// это значения которые можно использовать в теле запроса

type Category struct {
	Id   int
	Name string
}
type Product struct {
	Id   int
	Name string
	Category
	Price float64
}

db, _ := sql.Open("sqlite3", "products.db")
defer db.Close()

// заполнитель
categoryName := "Soccer"
rows, _ := db.Query(`
	SELECT Products.Id, Products.Name, Products.Price,
	Categories.Id as Cat_Id, Categories.Name as CatName
	FROM Products, Categories WHERE Products.Category = Categories.Id
    AND Categories.Name = ?`, categoryName)

products := []Product{}

for rows.Next() {
	p := Product{}
	rows.Scan(&p.Id, &p.Name, &p.Price, &p.Category.Id, &p.Category.Name)
	products = append(products, p)
}
fmt.Println(products) // [{3 Soccer Ball {2 Soccer} 19.5} {4 Corner Flags {2 Soccer} 34.95}]

//>> ПОЛУЧЕНИЕ ОТДЕЛЬНЫХ ЗАПИСЕЙ
//<< QueryRow(query, ...args)
// применяется когда нужно достать одну запись
db, _ := sql.Open(`sqlite3`, `products.db`)

row := db.QueryRow(`
SELECT Products.Id, Products.Name, Products.Price,
Categories.Id as Cat_Id, Categories.Name as CatName
FROM Products, Categories
WHERE Products.Category = Categories.Id
AND Products.Id = ?`, 1)
p := Product{}

row.Scan(&p.Id, &p.Name, &p.Price, &p.Category.Id, &p.Category.Name)

fmt.Println(p) // {1 Kayak {1 Watersports} 279}