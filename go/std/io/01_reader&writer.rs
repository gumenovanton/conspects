//>> READER & WRITER
// интерфейсы пакета io
// из любого типа реализующего Reader можно читать,
// его так и мпринято называть Reader
// в любой тип реализующий Writer можно писать,
// его так и мпринято называть Writer

//>> READER
// содержит только 1 метод Read(byteSlice)
// читающий в срез битов
// возвращает количество прочитанных байт и err
// если источник превышает размер слайса
// то Read читает по кускам равным слайсу

//<< Чтение из любого Reader(файла, потока...) на примере чтения из строки
// создаю Reader из строки
reader := strings.NewReader("Kayak")
// создаю срез байт
b := make([]byte, 2)
// читаю из Reader
for {
	count, err := reader.Read(b)
	if count > 0 {
		fmt.Printf("Read %v bytes: %v\n", count, string(b[0:count]))
	}
	// если конец выходим из цикла чтения
	// (чтобы этого не делать можно использовать for range)
	if err == io.EOF {
		break
	}
}
// Read 2 bytes: Ka
// Read 2 bytes: ya
// Read 1 bytes: k

//>> WRITER
// содержит только 1 метод Write(byteSlice)
// пишущий в Writer слайс байт
// возвращает количество записанных байт и err

//<< Запись в любой Writer(файл, поток...) на примере записи в строку
// создаю Writer
var builder strings.Builder
builder.Write([]byte("Hi "))
builder.Write([]byte("Anton!"))
// собираю строку
fmt.Println(builder.String()) // Hi Anton!