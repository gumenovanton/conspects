//>> СОЗДАНИЕ MIDDLEWARE
// просто функция принимающая тип HandleFunc
// и возвращающая тип HandleFunc
// тоесть передаем в нее обычный хендлер
// и в возвращаемой функции делаем свои вещи

//<< создаю Middleware
func middleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Println("Message from MW", r.Host)
		next(w, r)
	}
}

//<< создаю Хендлер
func indexHandler(w http.ResponseWriter, r *http.Request) {
	log.Println("Message From Hamdler", r.Host)
	io.Copy(w, r.Body)
}

//<< запускаю сервер 
// вешаю на путь миддварь
mux := http.NewServeMux()
mux.HandleFunc("/", middleware(indexHandler))
http.ListenAndServe(":3000", mux)