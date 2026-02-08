//>> СОЗДАНИЕ ШАБЛОНА ИЗ СТРОКИ
type SomeStruct struct{
    Message string
}
some_struct:=SomeStruct{"Hi"}
// строка шаблона
template_string := "Some message {{.Message}}"

// создаем шаблон с именем MessTEmplate из суем туда строку
my_template, err:= template.New("MessTEmplate").Parse(template_string)

// пишу шаблон в Writer и передаю в него данные
err = my_template.Execute(os.Stdout, &some_struct)

//>> СОЗДАНИЕ ШАБЛОНА ИЗ ФАЙЛА
// создаем шаблон из файла, имя шаблона - название файла с расширением
t,error:= template.ParseFiles("templates/index.html")
// пишу шаблон в Writer и передаю в него данные
t.Execute(os.Stdout, &some_struct)

//>> СОЗДАНИЕ ШАБЛОНОВ ИЗ НЕСКОЛЬКИХ ФАЙЛОВ
// загружаю сразу несколько файлов, имя шаблона - название файла с расширением
t, err:= template.ParseFiles("templates/layout.html", "templates/index.html")
// пишу шаблон в Writer по имени и передаю в него данные
all_t.ExecuteTemplate(os.Stdout,"index.html" ,&some_struct)

//>> ПОИСК ФАЙЛОВ ШАБЛОНОВ ПО ПАТТЕРНУ
// с этой функцией нужно быть акуратным, если шаблон не спарсится
all_t, err := template.ParseGlob("templates/*.html")
// найдем нужный
selected_templated := all_еemplates.Lookup("template.html")
selected_templated.Execute(os.Stdout ,&some_struct)

//>> ОБЪЕДИНЕНЕИЕ НЕСКОЛЬКИХ БЛОКОВ В ОДИН ШАБЛОН
// когда есть родитель и вложенные шаблоны 
// загужаем так
t, err:= template.ParseFiles("templates/layout.html", "templates/index.html")
// потом просто пишем в Writer так
t.Execute(os.Stdout, &some_struct)
