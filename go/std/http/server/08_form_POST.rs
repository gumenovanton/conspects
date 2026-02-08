//>> ПОЛУЧЕНИЕ ДАННЫХ ИЗ ФОРМ ОТПРАВЛЕННЫХ МЕТОДОМ POST и кодировкой application/x-www-form-urlencoded
// данные могут содержаться в URL, 
// например когда форма имеет action="/form?name=sara"
// и в BODY

// чтобы их достать с запроса нужно его распарсить методом ParseForm()
//<< ParseForm()
request.ParseForm()

// после этого данные попадут в струкруры запроса request.Form и request.PostForm
// которые представляют из себя map[string][]string
// из которых данные можно доставать по ключу(имя поля) и индексу
// так как значений по ключу может быть несколько
// при этом
// в request.Form - попадут данные как из URL, так из BODY
// а в request.PostForm - попадут только значения из BODY

//>> ЧТЕНИЕ ПОЛЕЙ ФОРМЫ из request.Form и request.PostForm отправленных с enctype="application/x-www-form-urlencoded"
//<< Form
fmt.Println(request.Form) // map[name:[John sara]]
fmt.Println(request.Form["name"][0])// John

//<< PostForm
// аналог Form, но сюда попадают только переменные из BODY
fmt.Println(request.PostForm) // map[name:[John]]
fmt.Println(request.PostForm["name"]) // John

//>> ХЕЛПЕРЫ ДЛЯ ЧТЕНИЕ ПОЛЕЙ ФОРМЫ из request.Form и request.PostForm отправленных с enctype="application/x-www-form-urlencoded"
// если мне нужно только одно значение из Form и PostForm
// например когда я знаю что там оно точно одно
// проще использовать хелперы FormValue() и PostFormValue()

//<< FormValue(key)
// вызовет ParseForm() неявно, то есть Form и PostForm будут заполнены
// возвращает первое значение из Form формы по имени поля
fmt.Println(request.FormValue("name"))

//<< PostFormValue(key)
// вызовет ParseForm() неявно, то есть Form и PostForm будут заполнены
// возвращает первое значение из PostForm формы по имени поля
fmt.Println(request.PostFormValue("name"))
