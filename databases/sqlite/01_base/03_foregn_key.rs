//>> FOREIGN KEY

//<< UNNAMED FOREIGN KEY
CREATE TABLE companies
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
CREATE TABLE users
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER,
    company_id INTEGER,
    FOREIGN KEY (company_id)  REFERENCES companies (id)
);

//<< NAMED FOREIGN KEY
CREATE TABLE users
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER,
    company_id INTEGER,
    CONSTRAINT users_companies_fk //NAME
    FOREIGN KEY (company_id)  REFERENCES companies (id)
);

//>> ON DELETE и ON UPDATE
//<< CASCADE
// автоматически удаляет или изменяет строки из зависимой таблицы 
// при удалении или изменении связанных строк в главной таблице.

//<< SET NULL 
// при удалении или обновлении связанной строки из главной 
// таблицы устанавливает для столбца внешнего ключа значение NULL. 
// (В этом случае столбец внешнего ключа должен поддерживать установку NULL)

//<< RESTRICT
// отклоняет удаление или изменение строк в главной таблице 
// при наличии связанных строк в зависимой таблице.

//<< NO ACTION 
// то же самое, что и RESTRICT.

//<< SET DEFAULT
// при удалении связанной строки из главной таблицы устанавливает 
// для столбца внешнего ключа значение по умолчанию, 
// которое задается с помощью атрибуты DEFAULT.
CREATE TABLE users
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    age INTEGER,
    company_id INTEGER,
    FOREIGN KEY (company_id)  REFERENCES companies (id) ON DELETE SET NULL
);