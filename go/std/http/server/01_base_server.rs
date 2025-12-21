//>> СОЗДАНИЕ СЕРВЕРА
// ~ создаем хандлер
// хендлер создаем как структуру с методом хендлером для каждого пути
// как правило в себе содержит ссылку на контроллер
// и состояние в которое можно например положить шаблоны
type StringHandler struct {
	message string
}

// сам хендлер должен принимать
// writer http.ResponseWriter куда пишем ответ
// и request *http.Request - откуда берем данные запроса
func (sh StringHandler) ServeHTTP(writer http.ResponseWriter, request *http.Request) {
	io.WriteString(writer, sh.message)
}

//<< запуск сервера
// по http использую ListenAndServe(addr, handler)
// по https использую ListenAndServeTLS(addr, cert, key, handler)
// ListenAndServe принимает все что реализует интерфейс Handler
// в
http.ListenAndServe(":5000", StringHandler{message: "Hello, Антоха!!!"})