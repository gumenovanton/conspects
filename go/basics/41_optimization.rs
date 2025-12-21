//>> ОПТИМИЗАЦИЯ ПРАВИЛА

//<< cложение строк
// самым быстрым является сложение строк типа строка = строка1 + строка2 + строка3
func sumStringInOneString() string {
    var str string
    name := "Simba"
    action := "Change Password"
    code := "41251"

    str = str +
       "Name: " + name +
       "Action: " + action +
       "Code: " + code

    return str

// на втором мест запись строк в string.Builder и последующая сборка
func sumStringWithBuilder() string {
    var sb strings.Builder
    name := "Simba"
    action := "Change Password"
    code := "41251"

    sb.WriteString("Name: ")
    sb.WriteString(name)
    sb.WriteString("Action: ")
    sb.WriteString(action)
    sb.WriteString("Code: ")
    sb.WriteString(code)

    return sb.String()
}

// на третем мете формат строка = строка + часть, строка = строка + часть2
func sumStringInMoreStrings() string {
    var str string
    name := "Simba"
    action := "Change Password"
    code := "41251"

    str = str + "Name: "
    str = str + name
    str = str + "Action: "
    str = str + action
    str = str + "Code: "
    str = str + code

    return str
}

//<< переалокации
// когда нужно постоянно выделять память и стирать
// сборщик мусора постоянно запускается и жрет ресурсы
// вместо выделения памяти
// испольуй sync.Pool
// Создание пула.
var dataPool = sync.Pool{
    New: func() any {
       return make([]int, 0, 10000)
    },
}

// Работа с пулом.
func processPool() {
    data := dataPool.Get().([]int)
    // Некоторая обработка данных
    for i := 0; i < 10000; i++ {
       data = append(data, i)
    }

    // Очистка.
    data = data[:0]
    dataPool.Put(data)
}