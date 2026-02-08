package main

import (
	"log/slog"
	"os"
)

// _ СОКРЫТИЕ ЧУВСТВИТЕЛЬНЫХ ПОЛЕЙ
// если я оперирую какой то сущьностью, например User
// и я передам ее в лог, все поля попадут в строку логов
// я могу изменить вывод как мне хочется
// для этого нужно реализовать интерфейс
// type LogValuer interface {
//     LogValue() Value
// }

// например моя структура
type User struct {
	ID        string `json:"id"`
	FirstName string `json:"first_name"`
	LastName  string `json:"last_name"`
	Email     string `json:"email"`
	Password  string `json:"password"`
}

// реализую интерфейс
func (u User) LogValue() slog.Value {
	// при передачи переменной типа User в логи хочу видеть только id
	return slog.StringValue(u.ID)
}

func main() {
	handler := slog.NewJSONHandler(os.Stdout, nil)
	logger := slog.New(handler)

	u := &User{
		ID:        "user-12234",
		FirstName: "Jan",
		LastName:  "Doe",
		Email:     "jan@example.com",
		Password:  "pass-12334",
	}
	// вывожу в логи
	logger.Info("info", "user", u)
	// ~ так выглядит вывод с реализацией интерфейса
	// 	{
	// 	  "time": "2024-07-04T12:50:00.777814763+04:00",
	// 	  "level": "INFO",
	// 	  "msg": "info",
	// 	  "user": "user-12234"
	// 	}
	// ~ а так вывод если реализацию интерфейса закоментировать
	// 	{
	// 		"time": "2024-07-04T12:49:05.058250939+04:00",
	// 		"level": "INFO",
	// 		"msg": "info",
	// 		"user": {
	// 		  "id": "user-12234",
	// 		  "first_name": "Jan",
	// 		  "last_name": "Doe",
	// 		  "email": "jan@example.com",
	// 		  "password": "pass-12334"
	// 		}
	// 	}
}
