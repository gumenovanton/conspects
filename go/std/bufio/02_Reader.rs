//>> bufio.READER
//<< NewReader(r)
// возвращает буферизированный Reader c буфером 4096 байт
//<< NewReaderSize(r, size)
// возвращает буферизированный Reader c буфером size

// открывакм файл
file, _ := os.OpenFile(`1.txt`, os.O_RDONLY, 6660)
// определяем Reader но не читаем
// читать будет весь файл при первой попытке чтения из Reader
reader := bufio.NewReader(file)

//>> МЕТОДЫ СЧИТЫВАНИЯ READER
//<< Read(p []byte)
// считывает срез байтов и возвращает количество прочитанных байтов
//<< ReadByte()
// считывает один байт
//<< ReadBytes(delim byte)
// считывает срез байтов из потока, пока не встретится байт delim
//<< ReadLine()
// считывает строку в виде среза байт
//<< ReadRune()
// считывает один объект типа rune
//<< ReadSlice(delim byte)
// считывает срез байтов из потока, пока не встретится байт delim
//<< ReadString(delim byte)
// считывает строку, пока не встретится байт delim

// выводим построчно
line, _ := reader.ReadString('\n') // в этот момент в буфер прочтется весь файл
fmt.Print(line)                    // У лукоморья дуб зелёный;
line, _ = reader.ReadString('\n')
fmt.Print(line) // Златая цепь на дубе том:
line, _ = reader.ReadString('\n')
fmt.Print(line) // И днём и ночью кот учёный
line, _ = reader.ReadString('\n')
fmt.Print(line) // Всё ходит по цепи кругом;

//>> ДОП МЕТОДЫ READER
//<< Buffered()
// Этот метод возвращает int число,
// указывающее количество байтов,
// которые можно прочитать из буфера.
// то есть сколько данных осталось
// сработает только после первой операции чтения
fmt.Println(reader.Buffered()) // 1417

//<< Discard(count)
// Этот метод отбрасывает указанное количество байтов.
reader.Discard(1380)
fmt.Println(reader.Buffered()) // 37

//<< Peek(count)
// Этот метод возвращает указанное количество байтов,
// не удаляя их из буфера,
// то есть они будут возвращены последующими вызовами метода Read.
bytes, _ := reader.Peek(37)
fmt.Println(string(bytes)) // � мне сказки говорил.
line, _ = reader.ReadString('\n')
fmt.Print(line) //  мне сказки говорил.
fmt.Println()
file.Close()

//<< Reset(reader)
// Этот метод отбрасывает данные в буфере
// и выполняет последующие чтения из указанного Reader.
file, _ = os.OpenFile(`1.txt`, os.O_RDONLY, 6660)
reader.Reset(file)
fmt.Println(reader.Buffered()) // 0 не читает с файла пока не нужно
line, _ = reader.ReadString('\n')
fmt.Print(line)                // У лукоморья дуб зелёный;
fmt.Println(reader.Buffered()) // 1553

//<< Size()
// Этот метод возвращает размер буфера, выраженный int.
// fmt.Println(reader.Size()) // 4096

//>> ЧТЕНИЕ ИЗ КОНСОЛИ
reader := bufio.NewReader(os.Stdin)
fmt.Print("Введите текст: ")
text, _ := reader.ReadString('\n')
fmt.Println(text)