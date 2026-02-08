//>> СУТЬ
// когда простые методы отправки
// неявно создается структура Client
// является глобальной и доступна всем пакетом
// это не очень для безопасности
// и не позволяет управлять запросом
// по этому хорошая практика создавать клиент руками

//>> СОЗДАНИЕ КЛИЕНТА
client := http.Client{}

//>> ПОЛЯ Client
// Transport - можно выбрать транспорт, но он определен по умолчанию
// CheckRedirect - для указания пользовательской политики для обработки повторяющихся перенаправлений
// Jar - возвращает CookieJar, управляющий файлами куки
// Timeout - установка таймаута запроса

//>> МЕТОДЫ Client
//<< Do(request) 
// отправляет Request возвращая Response и error
client := http.Client{}
reqURL, _ := url.Parse("https://httpbin.org/get")
	req := http.Request{
		Method: http.MethodGet,
		URL:    reqURL,
		Header: map[string][]string{
			"accept": {"application/json"},
		},
	}
res, _ := client.Do(&req)

//<< CloseIdleConnections() 
// закрывает все бездействующие HTTP-запросы, которые в настоящее время открыты и не используются
client.CloseIdleConnections()

//<< Get(url) 
// вызывает функцию Get
res, _ := client.Get("https://httpbin.org/get")

//<< Head(url) 
// вызывает функцию Head
res, _ := client.Head("https://httpbin.org/get")

//<< Post(url, contentType, reader)
// вызывает функцию Post
client := http.Client{}
var prod = Person{"Sam", "John"}

var builder strings.Builder
json.NewEncoder(&builder).Encode(prod)
res, _ := client.Post("https://httpbin.org/get", `application/json`, strings.NewReader(builder.String()))

//<< PostForm(url, data)
// вызывает функцию PostForm
client := http.Client{}
	res, _ := client.PostForm("https://httpbin.org/post", 
    map[string][]string{
		`name`: {`james`},
		`last`: {`jones`},
    }
)