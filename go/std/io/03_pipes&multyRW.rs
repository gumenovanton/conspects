//>> СПЕЦИАЛИЗИРОВАННЫЕ ФУНКЦИИ ДЛЯ ЧТЕНИЯ И ЗАПИСИ
//<< Pipe()
// то же самое что и каналы, только оперируют слайсами
// возвращает концы трубы pipeReader и pipeWriter
// в одном месте программы я могу писать в pipeWriter
// в другом месте я могу читать из pipeReader
// пока в трубе лежит не прочитанный слайс
// запись будет заблокированна
pipeReader, pipeWriter := io.Pipe()
// пишем обязательно в другой горутине, иначе deadlock
go func() {
	// обязательно закрываем
	defer pipeWriter.Close()
	pipeWriter.Write([]byte("jane"))
}()
b := make([]byte, 10)
pipeReader.Read(b)
fmt.Println(string(b)) // jane

//<< MultiReader(...readers)
// собирает данные из нескольких Reader и позволяет читать их
r1 := strings.NewReader("Kayak")
r2 := strings.NewReader("Lifejacket")
r3 := strings.NewReader("Canoe")
concatReader := io.MultiReader(r1, r2, r3)
i, _ := concatReader.Read(b)
fmt.Println(string(b[:i])) // Kayak
i, _ = concatReader.Read(b)
fmt.Println(string(b[:i])) // Lifejacket
i, _ = concatReader.Read(b)
fmt.Println(string(b[:i])) // Canoe

//<< MultiWriter(...writers)
// позволяет обьединить нескольких Writer
// писать будет одни данные во все Writer
var w1 strings.Builder
var w2 strings.Builder
var w3 strings.Builder
combinedWriter := io.MultiWriter(&w1, &w2, &w3)
combinedWriter.Write([]byte("Katty"))
fmt.Println(w1.String()) // Katty
fmt.Println(w2.String()) // Katty
fmt.Println(w3.String()) // Katty

//<< TeeReader(multiReader, writer)
// читает из нескольких Reader в один Writer
r1 = strings.NewReader("Kayak")
r2 = strings.NewReader("Lifejacket")
r3 = strings.NewReader("Canoe")
concatReader = io.MultiReader(r1, r2, r3)

// сщздаю Writer
var writer strings.Builder
teeReader := io.TeeReader(concatReader, &writer)
teeReader.Read(b)            // читаю из первого Reader в Writer
teeReader.Read(b)            // читаю из второго Reader в Writer
teeReader.Read(b)            // читаю из третьего Reader в Writer
fmt.Println(writer.String()) // KayakLifejacketCanoe

//<< LimitReader(r, limit)
// ограничивает количество данных которые можно считывать с MultiReader
r1 = strings.NewReader("Kayak")
r2 = strings.NewReader("Lifejacket")
r3 = strings.NewReader("Canoe")
concatReader = io.MultiReader(r1, r2, r3)
limited := io.LimitReader(concatReader, 13)
i, _ = limited.Read(b)
fmt.Println(string(b[:i])) // Kayak
i, _ = limited.Read(b)
fmt.Println(string(b[:i])) // Lifejack и обрежет
i, _ = limited.Read(b)
fmt.Println(string(b[:i])) // здесь будет пусто