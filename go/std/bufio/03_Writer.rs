//>> bufio.WRITER
//<< NewWriter(w)
// Эта функция возвращает буферизованный Writer
// с размером буфера по умолчанию
// (который на момент записи составляет 4096 байт).
//<< NewWriterSize(w, size)
// Эта функция возвращает буферизованный Writer
// с указанным размером буфера.
file, _ := os.OpenFile(`3.txt`, os.O_RDWR|os.O_CREATE, 6660)
writer := bufio.NewWriter(file)

//>> МЕТОДЫ ЗАПИСИ WRITER
//<< Write()
// записывает срез байтов
//<< WriteByte()
// записывает один байт
//<< WriteRune()
// записывает один объект типа rune
//<< WriteString()
// записывает строку
writer.WriteString("first string\n")
writer.WriteString("second string\n")
writer.WriteString("third string\n")

//>> СЛУЖЕБНЫЕ ФУНКЦИИ
//<< Buffered()
// сколько байт занято
fmt.Println(writer.Buffered())

//<< Available()
// сколько байт доступно
fmt.Println(writer.Available())

//<< Size()
// размер буфера
fmt.Println(writer.Size())

//>> ЗАПИСЬ БУФЕРА В ФАЙЛ
//<< Flush()
// запишет из буфера в файл
writer.Flush()

//>> СБРОС БУФЕРА
//<< Reset(writer)
// сбросит данные буфера и привяжет его к Writer
writer.Reset(file)