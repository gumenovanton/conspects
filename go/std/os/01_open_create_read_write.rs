//>> ПРЯМОЕ ЧТЕНИЕ ФАЙЛА В СРЕЗ
//<< ReadFile(name)
// открывает и читает файл в срез
slice, _ := os.ReadFile(`4.txt`)
fmt.Println(string(slice)) // {"name":"Sow","price":66.25,"doscount":10}

//>> ПРЯМАЯ ЗАПИСЬ В ФАЙЛ
//<< WriteFile(name, slice, modePerms)
// создает, записывавет или переписывает файл name
// записывая slice
// c режимом modePerms
os.WriteFile(`5.txt`, []byte("holla"), 0666) // holla

//>> ОТКРЫТИЕ С ВОЗВРАЩЕНИЕМ СТРУКТУРЫ File
//<< Open(name)
// открывает файл для чтения,возвращая File
file, _ := os.Open(`3.txt`)
file.Close()

//>> ОТКРЫТИЕ И СОЗДАНИЕ 
//<< OpenFile(name, flag, modePerms)
// открывает файл с modePerms как правило это 0666
// и флагами:
// os.O_APPEND - все записывать в конец, не переписывая содержание
// os.O_CREATE - если не найдет файл создаст, без прав на линукс, нужно задавать права методами Chmod(), Chown()
// os.O_EXCL - используется в паре с O_CREATE, открытие файла будет неудачным если он существует
// os.O_RDONLY - только для чтения
// os.O_RDWR - для чтения и записи
// os.O_SYNC - открыт для синхронных IO
// os.O_TRUNC - с удалением содержимого, нужно разрешение на запись
// os.O_WRONLY - только для запись
file, _ = os.OpenFile(`3.txt`, os.O_RDONLY|O_CREATE, 0666)

//<< Create(name)
// то же самое что OpenFile с флагами O_RDWR|O_CREATE|O_TRUNC

//<< CreateTemp(dirName, fileName)
// создает и открывает файл с флагами O_RDWR, O_CREATE и O_EXCL
// в каталоге dirName
// если dirName пустая строка, то создаст файл в Temp
// добавив случайную строку вместо звездочки или к концу имени
// файл не удаляется при закрытии

//>> ЧТЕНИЕ с File
//<< Read(b)
// читает из File в срез
b := make([]byte, 100)
file.Read(b)
fmt.Println(string(b))
// first string
// second string
// third string

//<< ReadAt(slice, offset)
// читает из File в срез с отступом offset
file.ReadAt(b, 10)
fmt.Println(string(b))
// ng
// second string
// third string


//>> ЗАПИСЬ с File
file, _ := os.OpenFile(`6.txt`, os.O_RDWR|os.O_CREATE|os.O_APPEND, 0666)

//<< Write(slice)
// пишет slice в File
file.Write([]byte("holla"))

//<< WriteAt(slice, offset)
// пишет slice в File начиная с отступом offset
// с флаго O_APPEND не сработает
file.WriteAt([]byte("HOLLA\n"), 5)

//<< WriteString(str)
// записывает строку в файл
file.WriteString("WRITE_STRING")

//>> СМЕЩЕНИЯ ТОЧКИ ДЛЯ ЧТЕНИЯ И ЗАПИСИ
//<< Seek(offset, how)
// пеермещает точку считывания
// offset - сколько сдвинуться
// how:
// 		0 - смещение от начала файла
// 		1 - смещение относительно текущей позиции
// 		2 - смещение от конца файла
file.Seek(20, 0)
file.Read(b)
fmt.Println(string(b))
// string
// third string

//>> УДАЛЕНИЕ ФАЙЛА
os.Remove("example.txt")