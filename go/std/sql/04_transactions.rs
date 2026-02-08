//>> ИСПОЛЬЗОВАНИЕ ТРАНЗАКЦИЙ

// транзакции это серия запросов
// которые исполнятся либо все либо не один
db, _ := sql.Open(`sqlite3`, `products.db`)

//<< Begin()
// начинает транзакцию и возвращает структуру Tx

//>> МЕТОДЫ Тх
//<< Query(query, ...args)
// делает запрос возвращая несколько записей
//<< QueryRow(query, ...args)
// делает запрос на один результат
//<< Exac(query, ...args)
// выполняет запрос не возвращающий не одного результата
//<< Prepare(query)
// сохраняет запрос в переменной
//<< Stmt(statement)
// принимает подготовленный запрос и возвращает его же но прекрепленный к транзакции
//<< Commit()
// фиксирует изменения серии транзакций
//<< Rollback()
// откатывает транзакцию

// например делаем два запроса
insertNewCategory, _ := db.Prepare("INSERT INTO Categories (Name) VALUES (?)")
changeProductCategory, _ := db.Prepare("UPDATE Products SET Category = ? WHERE Id = ?")

// начинаю транзакцию
tx, err := db.Begin()
// переменная ошибки
updatedFailed := false
if err == nil {
	// выполняю первый запрос
	catResult, err := tx.Stmt(insertNewCategory).Exec("Googs")
	if err == nil {
		// получаю id для которого был сделан первый запрос
		newID, _ := catResult.LastInsertId()
		// добавляю запрос в транзакцию
		preparedStatement := tx.Stmt(changeProductCategory)
		// меняю категорию у продукта с 3-м id
		changeResult, err := preparedStatement.Exec(newID, 3)
		if err == nil {
			changes, _ := changeResult.RowsAffected()
			if changes == 0 {
				// если я не поменял не одной записи пишу что обновления небыло
				updatedFailed = true
			}
		}
	}
}
// если были ошибоки или не произошло обновление откатываю транзакцию
if err != nil || updatedFailed {
	fmt.Printf("Aborting transaction %v", err)
	tx.Rollback()
} else {
	// иначе подтверждаю транзакцию
	tx.Commit()
}