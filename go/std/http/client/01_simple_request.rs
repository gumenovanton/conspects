//>> ПРОСТЫЕ МЕТОДЫ ОТПРАВКИ ЗАПРОСОВ
//<< Get(url)
// отправляет GET запрос на url
response, _ := http.Get("http://localhost:4000/client")
response.Write(os.Stdout)

//<< Head(url)
// то же самое что и GET запрос,
// только вернет ответ без BODY
// важно чтоб сервер обрабатывал таколй метод
response, _ = http.Head("http://localhost:4000/client")
response.Write(os.Stdout)

//<< Post(url, contentType, reader)
// отправляет POST запрос на URL c помощью reader
var builder strings.Builder
builder.WriteString(`holla`)

response, _ = http.Post("http://localhost:4000/client", "plain/text", strings.NewReader(builder.String()))

io.Copy(os.Stdout, response.Body)
defer response.Body.Close()

//<< PostForm(url, data)
// отправит данные с заголовком application/x-wwwform-urlencoded
// data представляет из себяmap[string][]string
formData := map[string][]string{
	"name":     {"Kayak "},
	"category": {"Watersports"},
	"price":    {"279"},
}
response, _ = http.PostForm("http://localhost:4000/client", formData)
fmt.Println(response.StatusCode) // 200