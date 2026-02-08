//>> СТРУКТУРА Response
fmt.Println(response.StatusCode) // 200
fmt.Println(response.Status) // 200 OK
fmt.Println(response.Proto) // HTTP/1.1
fmt.Println(response.Header) // map[Content-Length:[45] Content-Type:[text/plain; charset=utf-8] Date:[Sun, 11 Aug 2024 11:17:05 GMT]]
fmt.Println(response.Trailer) // map[]
fmt.Println(response.ContentLength) // 45 в int64.
fmt.Println(response.TransferEncoding) // []
fmt.Println(response.Close) // false
fmt.Println(response.Uncompressed) // false
fmt.Println(response.Request) // &{GET http://localhost:4000/client HTTP/1.1 1 1 map[] <nil> <nil> 0 [] false localhost:4000 map[] map[] <nil> map[]   <nil> <nil> <nil> {{}} <nil> [] map[]}
fmt.Println(response.TLS) // <nil>
fmt.Println(response.Cookies()) // []
fmt.Println(response.Location()) // <nil> http: no Location header in response

//<< Write(writer)
// Этот метод записывает сводку ответа на указанный Writer.
response.Write(os.Stdout)
// HTTP/1.1 200 OK
// Content-Length: 45
// Content-Type: text/plain; charset=utf-8
// Date: Sun, 11 Aug 2024 11:40:15 GMT
//
// {"id":1,"name":"Anton","phone":"9173297729"}

//<< Body
// возвращает ReadCloser, который является Reader,
// определяющим метод Close и обеспечивающим доступ к телу ответа.
data, _ := io.ReadAll(response.Body)
fmt.Println(string(data)) // {"id":1,"name":"Anton","phone":"9173297729"}
// обязательно закрываем
defer response.Body.Close()
// можно записать в консоль
os.Stdout.Write(data) // {"id":1,"name":"Anton","phone":"9173297729"}
