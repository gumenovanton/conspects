//>> РАБОТА С ОТВЕТАМИ В ХЕНДЛЕРЕ

//<< Установка Хедеров
// Go генерирует автоматические заголовки:
// Date
// Content-Type
// Content-Length

// для JSON Content-Type = plane/text
// по этому нужно менять на application/json 

// установка хедера
// при этом го сам первые символы сделает с большой а остальные с маленькой
writer.Header().Set("Content-Type", "application/json")

// добавление хедера, когда нужно добавить несколько одного имени
writer.Header().Add("Allow", "REST")

// удалить хедер, не удаляет автоматические хедеры
writer.Header().Del("Allow", "REST")
// чтобы удалить автоматические хедеры нужно обнулить их вручную 
writer.Header()["Date"] = nil

// получить значение хедера
writer.Header().Get("Allow", "REST")

// вывести срез всех хедеров со значением по ключу
writer.Header().Values("Cash-Control")

//<< Установка статуса ответа
// статусы ответа можно писать константами
writer.WriteHeader(http.StatusOK)
// или так
writer.WriteHeader(200)

//<< Запись в BODY
writer.Write([]byte(sh.message))

//>> УДОБНЫЕ ФУНКЦИИ ОТВЕТА
// их нужно использовать в теле Хандлера

//<< Возврать текстовой ошибки
// эта функция заменяет пару WriteHeader со статусом ошибки 
// и Write с сообщением об ошибке
http.Error(writer, "Че то не так", 500)

//<< Возврат ошибки NotFound
// возвразщает текстовую ошибку, которая отбразиться в браузере как 404 page not found
http.NotFound(writer, request)

//<< Редирект
// редирект на path с кодом ошибки
// если редирект с /, выведет код ошибки как ссылку на /auth
http.Redirect(writer, request, `auth`, 500)

//<< Отправка файла
// отправит файл и отобразит в браузере
http.ServeFile(writer, request, `1.jpg`)

// можно отправить html
http.ServeFile(writer, request, `index.html`)

//>> ОТПРАВКА JSON В ОТВЕТ
// чтобы отправить ответ как json нужно задать соответствующий хедер
writer.Header().Set("Content-Type", "application/json")
// и создаеть Encoder и закодировать данные 
// и они запишуться в ответ
json.NewEncoder(writer).Encode(Products)
