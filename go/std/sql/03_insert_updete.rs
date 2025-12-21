//>> ВЫПОЛНЕНИЕ ЗАПРОСОВ, НЕ ТРЕБУЮЩИХ ВОЗВРАТА ЗНАЧЕНИЙ
db, _ := sql.Open(`sqlite3`, `products.db`)

//<< Exec(query, ...args)
// Eхec возвращает Result
// который имеет 2 метода для работы с результатом
// RowsAffected() - визвращает количество строк затронутых инструкцией
// LastInsertId() - возвращает int64 представляющее значение генерируемого Id
result, _ := db.Exec(`INSERT INTO Products (Name, Category, Price) VALUES (?,?,?)`, `James`, 1, 44)

fmt.Println(result.LastInsertId()) // 11
fmt.Println(result.RowsAffected()) // 1

rows, _ := db.Query(`SELECT * FROM Products`)

for rows.Next() {
    var id int
    var name string
    var category int
    var price float64
    rows.Scan(&id, &name, &category, &price)
    fmt.Println(name, category, price)
}

//>> ИСПОЛЬЗОВАНИЕ ПОДГОТОВЛЕННЫХ ОПЕРАТОРОВ
//  тупо готовлю и сохраняю запросы

//<< Prepare(query)
// возвращает подготовленнуб структуру запроса Stmt и ошибку
// с нимим можно работать пока не вызван метод Close()

selectAll, _ := db.Prepare("SELECT * FROM Products")
selectOne, _ := db.Prepare("SELECT Name, Price FROM Products WHERE Id=?")
insertNewCategory, _ := db.Prepare("INSERT INTO Categories (Name) VALUES (?)")
// changeProductCaregory, _ := db.Prepare("UPDATE Products SET Category = ? WHERE Id = ?")

//>> МЕТОДЫ Stmt
// применяются те же функции что и в запросе через bd

//<< Query(...vals)
// запросы на много строк
rows, _ := selectAll.Query()
for rows.Next() {
	var Id int
	var Name string
	var Category int
	var Price float64
	rows.Scan(&Id, &Name, &Category, &Price)
	fmt.Println(Id, Name, Category, Price)
}

//<< QueryRow(..vals)
// запросы на 1 строку
row := selectOne.QueryRow(4)
var Name string
var Price float64
row.Scan(&Name, &Price)
fmt.Println(Name, Price)

//<< Exec(..vals)
// запросы не втравку и изменение
insertNewCategory.Exec("Balls")
rows, _ = db.Query("SELECT Name FROM Categories")
for rows.Next() {
	var Name string
	rows.Scan(&Name)
	fmt.Println(Name)
}

//<< Close()
// закрытие оператора