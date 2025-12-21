//>> ТЕОРИЯ
// Форма может быть отправлена тремя свособами
// GET запросом 
// POST запросом без указания enctype(по умолчанию будет задан application/x-www-form-urlencoded)
// POST запрос с enctype="multipart/form-data"

//<< GET запросом
// данные будут вложены в сам URL 
// парами ключ значение через & 
// типа 
// /submit?name=Ivan&surname=Ivanov.

//<< POST c enctype="application/x-www-form-urlencoded"
// данные будут вложены в BODY и будут иметь примерно такой вид
first_name=sausheong&last_name=chang

//<< POST c enctype="multipart/form-data"
// данные будут вложены в BODY и будут иметь примерно такой вид
------WebKitFormBoundaryMPNjKpeO9cLiocMw
Content-Disposition: form-data; name="first_name"

sau sheong
------WebKitFormBoundaryMPNjKpeO9cLiocMw
Content-Disposition: form-data; name="last_name"

chang
------WebKitFormBoundaryMPNjKpeO9cLiocMw--

//>> ПОЛУЧЕНИЕ ДАННЫХ ИЗ ФОРМ ОТПРАВЛЕННЫХ МЕТОДОМ GET
// поля нужно просто достать из Query,
fmt.Println(request.URL.Query().Get(`name`)) // John
fmt.Println(request.URL.Query().Get(`age`))  // 33
fmt.Println(request.URL.Query())             // map[age:[33] name:[John]]
fmt.Println(request.URL.RawQuery)            // name=John&age=33

