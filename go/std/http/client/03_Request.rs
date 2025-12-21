//>> СТРУКТУРА Request
// основные поля описаны в ../server/02_Request.rs
// в структуре два замороченных поля
// URL и Body, которые по факту структуры
// по этому создаются тоже отдельно

//>> СОЗДАНИЕ URL
// самый простой способ создать url.URL
reqURL, _ := url.Parse("http://localhost:5000/echo")    

//>> СОЗДАНИЕ Body
// поле Body содержит ReadCloser
// то есть любой тип который реализует Reader и Closer
// Если у меня есть метод Reader который не реализует Closer
// ReadCloser можно создать из Reader функцией io.NopCloser(reader), 
// которая возвращает структуру nopCloser которая в себе содержит reader
// и реализует методы Read() и Close()

//! так устроено BODY, 
//! потому что по факту это может быть, даже файл     

type Product struct {
    Name, Category string
    Price float64
}
var prod = Product{ "Kayak", "Watersports", 279 }

var builder strings.Builder
json.NewEncoder(&builder).Encode(prod)

body:=io.NopCloser(strings.NewReader(builder.String()))

//>> СОЗДАНИЕ Request, литеральный синтаксис
req := http.Request{
    Method: http.MethodGet,
    URL:    reqURL,
    Header: map[string][]string{
        "Content-Type": {"application.json"},
    },
    Body: io.NopCloser(strings.NewReader(builder.String())),
}

//>> КОНСТРУКТОРЫ
//<< NewRequest(method, url, reader)
// создает Request без контекста
req, err := http.NewRequest(http.MethodPost, "http://localhost:5000/echo", io.NopCloser(strings.NewReader(builder.String())))

//<< NewRequestWithContext(context, method, url, reader)
// создает Request с контекстом
req, err := http.NewRequestWithContext(context.Background(), http.MethodPost, "http://localhost:5000/echo", io.NopCloser(strings.NewReader(builder.String())))
