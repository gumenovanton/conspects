//>> ОТПРАВКА ДАННЫХ ФОРМЫ
// на клиенте я могу эмулировать отправку формы
// делается это через создание Writer 
// с помощью функции multipart.NewWriter
// пакета "mime/multipart"

// затем производится запись полей
// 

//<< создание Writer NewWriter(writer)
// делаем буфер бит
var buffer bytes.Buffer
formWriter := multipart.NewWriter(&buffer)

//<< создание поля и запись в него CreateFormField(fieldname)
fieldWriter, _ := formWriter.CreateFormField("name")
// пишем любым методом
io.WriteString(fieldWriter, "Alice")

fieldWriter, _ = formWriter.CreateFormField("city")
io.WriteString(fieldWriter, "New York")

//<< создание поля с отправкой файла и запись в него CreateFormFile(fieldname, filename)
fileWriter, _ := formWriter.CreateFormFile("codeFile", "printer.go")
fileData, _ := os.ReadFile("./printer.go")
fileWriter.Write(fileData)

//<< закрытие Writer Close()
// обозначает конец формы
//! ЗДЕСЬ НЕ ИСПОЛЬЗУЙ defer
formWriter.Close()

//<< создание запроса
req, _ := http.NewRequest(http.MethodPost, "http://localhost:5000/form", &buffer)

//<< создание заголовка с разделителем FormDataContentType()
req.Header["Content-Type"] = []string{formWriter.FormDataContentType()}
// отправка запроса
res, _ := http.DefaultClient.Do(req)