//>> ФАЙЛЫ СЕРТИФИКАТА
//<< файлы сертификата
// кидаем в папку httpsserver файл сертификата и публичного ключа

//>> ЗАПУСК СЕРВЕРА
// если работаю и с http и с https
// то ListenAndServeTLS запускаю в отдельной горутине 
// так как ListenAndServeTLS и ListenAndServe взаимоблокируются
go func () {
    err := http.ListenAndServeTLS(":5500", "certificate.cer", "certificate.key", nil)
    if (err != nil) {
        Printfln("HTTPS Error: %v", err.Error())
    }
}()

err := http.ListenAndServe(":5000", nil)
if (err != nil) {
    Printfln("Error: %v", err.Error())
}

//>> РЕДИРЕКТ НА HTTPS
// просто делаем функцию обработчик, которая правит url на https и делает redirect
func HTTPSRedirect(writer http.ResponseWriter, request *http.Request) {
	host := strings.Split(request.Host, ":")[0]
	target := "https://" + host + ":5500" + request.URL.Path
	if len(request.URL.RawQuery) > 0 {
		target += "?" + request.URL.RawQuery
	}
	http.Redirect(writer, request, target, http.StatusTemporaryRedirect)
}
func main() {
    // роеткр работает на сервер https, так как хендлер в ListenAndServeTLS - nil
	http.Handle("/message", StringHandler{"Hello, World"})

	go func() {
		http.ListenAndServeTLS(":5500", "certificate.cer", "certificate.key", nil)
	}()

    // на http nтолько один хендлер
	http.ListenAndServe(":5000", http.HandlerFunc(HTTPSRedirect))
}   