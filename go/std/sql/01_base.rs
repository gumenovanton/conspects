//>> ПОНЯТИЯ  
// работа с базами данных происходит при взаимодействии двух сущностей
// пакет database/sql пердставляет абстрактный функционал
// для работы с sql базами данных
// а пакеты драйверы обеспечивают совместимость с конкретной бд

//<< список драйверов тут 
https://go.dev/wiki/SQLDrivers

//>> ОТКРЫТИЕ БАЗЫ ДАННЫХ
// чтобы открыть базу данных нужно значть
// имя используемого драйвера подключенного к проекту
fmt.Println(sql.Drivers()) // [postgres sqlite3]

//<< Open(driver, connectionString)
// driver - имя используемого драйвера
// connectionString - строка подключения
// возвращает sql.DB
db, err := sql.Open("sqlite3", "products.db")
if err == nil {
	fmt.Println("Opened database")
}

//>> ЗАКРЫТИЕ БАЗЫ ДАННЫХ
defer db.Close()

