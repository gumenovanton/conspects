//>> СТРУКТУРА Cookie
// содержит следующие поля для конфигурирования cookie
// Name - имя куки, строка
// Value - значение, строка
// Path - путь к файлу cookie, не обязательно
// Domain - host/domain для которого поставим cookie, строка
// Expires - время истечения cookie, time.Time
// MaxAge - количество секунд до истечения cookie, int
// Secure - если true, то кука будет отправлена только по https
// HttpOnly - если true, то кука не будет доступна из JavaScript
// SameSite - политика перекрестного происхождения cookie с использованием констант
// 		- SameSiteDefaultMode - в большенстве поставит SameSiteLaxMode, не стоит использовать
// 		- SameSiteLaxMode - отправит куки если клиент переходит по ссылке на этот сайт
// 		- SameSiteStrictMode - отрпавит куки только если коиент на этом сайте
// 		- SameSiteNoneMode - куки полетят везде, используется только с параметром Strict=true

//>> УСТАНОВКА Cookie
//<< SetCookie(writer, cookie)
http.SetCookie(writer, &http.Cookie{
	Name:     "counter",
	Value:    strconv.Itoa(500),
	Secure:   true,
	HttpOnly: true,
})

//>> ЧТЕНИЕ Сookies
//<< Cookie(name)
// возвращает куку по имени
cookie, _ := request.Cookie(`counter`)
io.WriteString(writer, cookie.Value)

//<< Cookies()
// возвращает срез кук
cookies := request.Cookies()
io.WriteString(writer, cookies[0].Name)