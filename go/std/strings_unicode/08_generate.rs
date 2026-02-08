//>> ПОСТРОЕНИЕ И ГЕНЕРАЦИЯ СТРОК
//<< Join(slice, sep)
// слайс в строку через разделитель
fmt.Println(strings.Join([]string{"I", "Am", "MAN"}, "_")) // I_Am_MAN
//<< Repeat(s, count)
// вернет строку, повторяя строку s count раз
fmt.Println(strings.Repeat("MAN", 7)) // MANMANMANMANMANMANMAN

//>> СТРОИТЕЛЬНЫЕ СТРОКИ
// набор функций типа strings.Builder
// для построения строк

// создаю writer
var builder strings.Builder

// Формирую строку
builder.WriteString("Hi")
builder.WriteRune('_')
builder.WriteString("Anton")
// Если надо сбросил
builder.Reset()

// Формирую новую строку
builder.WriteString("Hi")    // добавил строку
builder.WriteRune('_')       // добавил _
builder.WriteString("Anton") // опять строку
builder.WriteByte(55)        // добавил байт
fmt.Println(builder.Len())   // 7 - сколько байт занято
fmt.Println(builder.Cap())   // 16 - сколько выделено
builder.Grow(8)              // добавил 16 байт
fmt.Println(builder.Cap())   // 40 - хз как увеличивает

// возвращаю итоговую строку из компоновщика
fmt.Println("String:", builder.String()) // String: Hi_Anton7