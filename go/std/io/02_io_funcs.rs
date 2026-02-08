//>> СЛУЖЕБНЫЕ ФУНКЦИИ ДЛЯ ЧТЕНИЯ И ЗАПИСИ
//<< Copy(w, r)
// копирует в writer w из reader r опустошая Reader
reader := strings.NewReader("This is the string i will be reading from")
io.Copy(os.Stdout, reader) // This is the string i will be reading from
fmt.Println()

//<< CopyBuffer(w, r, buffer)
// копирует в writer w из reader r используя промежуточный буфер опустошая Reader

//<< CopyN(w, r, count)
// копирует в writer w из reader r count байт опустошая Reader
reader = strings.NewReader("This is the string i will be reading from")
io.CopyN(os.Stdout, reader, 15) // This is the str
fmt.Println()

//<< ReadAll(r)
// читает все из reader в байтовый срез
reader = strings.NewReader("This is the string i will be reading from")
sl, _ := io.ReadAll(reader) // This is the str
fmt.Println(sl)             // [84 104 105 115 32 105 115 32 116 104 101 32 115 116 114 105 110 103 32 105 32 119 105 108 108 32 98 101 32 114 101 97 100 105 110 103 32 102 114 111 109]

//<< ReadAtLeast(r, byteSlice, min)
// копирует из reader r как минимум min бит в byteSlice
sl := make([]byte, 10)
reader := strings.NewReader("This is the string i will be reading from")
io.ReadAtLeast(reader, sl, 5)
fmt.Println(sl) // [84 104 105 115 32 105 115 32 116 104]

//<< ReadFull(r, byteSlice)
// заполняет byteSlice из reader r
sl = make([]byte, 10)
reader = strings.NewReader("This is the string i will be reading from")
io.ReadFull(reader, sl)
fmt.Println(sl) // [84 104 105 115 32 105 115 32 116 104]

//<< WriteString(w, str)
// записывает строку в writer
io.WriteString(os.Stdin, "This is the string for Builder\n")