//>> ПОЛУЧЕНИЕ ДАННЫХ ИЗ ФОРМ ОТПРАВЛЕННЫХ МЕТОДОМ POST и кодировкой multipart/form-data
// данные могут содержаться в URL, 
// например когда форма имеет action="/form?name=sara"
// и в BODY
// так же в BODY может быть и файл

// чтобы их достать с запроса нужно его распарсить методом ParseForm()
//<< ParseMultipartForm(max)
// max - считываемое количество байт
request.ParseMultipartForm(5000)

// после этого данные попадут в струкруры запроса request.Form, request.PostForm и request.MultipartForm
// которые представляют из себя map[string][]string
// а request. представляет из себя ссылку на структуру
type Form struct {
    Value map[string][]string
    File  map[string][]*FileHeader
}
// в request.Form - попадут данные из URL, так из BODY
// в request.PostForm - попадут только значения из BODY
// а в request.MultipartForm - попадут значения из BODY и Файлы

//>> ЧТЕНИЕ ПОЛЕЙ ФОРМЫ из request.Form, request.PostForm и request.MultipartForm отправленных с enctype="application/x-www-form-urlencoded"
// ~ Form
// отсюда достанием переменные как из URL, так из BODY
fmt.Println(request.Form)            // map[age:[38] goo:[boo] name:[John]]
fmt.Println(request.Form["name"][0]) // John

// ~ PostForm
// аналог Form, но сюда попадают только переменные из BODY
fmt.Println(request.PostForm)         // map[age:[38] name:[John]]
fmt.Println(request.PostForm["name"]) // [John]

//<< MultipartForm
// аналог PostForm, но сюда попадают только переменные из BODY и файлы
fmt.Println(request.MultipartForm.Value)         // map[age:[38] name:[John]]
fmt.Println(request.MultipartForm.Value[`name`]) // [John]
fmt.Println(request.MultipartForm.File)          // map[file:[0xc0001ac060 0xc0001ac120]]

//! если в input добавить multiple, то можно добавлять несколько файлов
//! по этому в поле File по имени поля может лежать несколько значений
fmt.Println(request.MultipartForm.File[`file`][0])      // вернет FileHeader файла &{go.env map[Content-Disposition:[form-data; name="file"; filename="go.env"] Content-Type:[application/octet-stream]] 505 [] /tmp/multipart-1619109753 0 false}

//>> МЕТОДЫ FileHeader
//<< Open(), Close()
// через FileHeader можно открыть и закрыть файл для чтения 
file, _ := request.MultipartForm.File[`file`][0].Open() // открываем файл
defer file.Close() // закрыть
io.Copy(os.Stdout, file)

//<< Name, Size, Header
// посмотреть имя файла, размер и заголовки
fmt.Println(file.Name) 

//>> ХЕЛПЕРЫ ДЛЯ ЧТЕНИЕ ПОЛЕЙ ФОРМЫ из request.Form, request.PostForm и request.MultipartForm отправленных с enctype="application/x-www-form-urlencoded"
// если мне нужно только одно значение из Form, PostForm 
// или один файл из MultipartForm
// например когда я знаю что там оно точно одно
// проще использовать хелперы FormValue(), PostFormValue() и FormFile()

//<< FormValue(key)
// вызовет ParseMultipartForm() неявно, 
// то есть Form, PostForm и MultipartForm будут заполнены
// возвращает первое значение из Form формы по имени поля
fmt.Println(request.FormValue("name"))

//<< PostFormValue(key)
// вызовет ParseMultipartForm() неявно, 
// то есть Form, PostForm и MultipartForm будут заполнены
// возвращает первое значение из PostForm формы по имени поля
fmt.Println(request.PostFormValue("name"))

//<< FormFile(key)
// вызовет ParseMultipartForm() неявно, 
// то есть Form, PostForm и MultipartForm будут заполнены
// возвращает первый файл из MultipartForm формы по имени поля
// и FileHeader
file, header, _ := request.FormFile(`file`)
defer file.Close()
io.Copy(os.Stdout, file)

