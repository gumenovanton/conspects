//>> ПОЛУЧЕНИЕ ВСЕХ ФАЙЛОВ И КАТАЛОГОВ ДИРЕКТОРИИ 
//<< ReadDir(name)
// возвращает срез DirEntry представляющий содержимое папки
dir, _ := os.Getwd()
entries, _ := os.ReadDir(dir)
fmt.Println(entries)    // [- 1.txt - go.mod - main.go]
fmt.Println(entries[0]) // - 1.txt

//>> КОМПОНЕНТЫ DirEntry
//<< Name()
// имя
fmt.Println(entries[0].Name()) // 1.txt

//<< IsDir()
// являетсмя ли директорией
fmt.Println(entries[0].IsDir()) // false

//<< Type()
// права доступа FileMod, почему то не работает
fmt.Println(entries[0].Type().Perm()) // ----------

//<< Info()
// возвращает FileInfo
fmt.Println(entries[0].Info()) // &{1.txt 0 493 {440306551 63858614399 0x54c140} {44 295590 1 33261 1000 1000 0 0 0 4096 0 {1723017599 440306551} {1723017599 440306551} {1723017610 781536775} [0 0 0]}} <nil>

//>> КОМПОНЕНТЫ FileInfo

//<< Stat(path)
// FileInfo можно вернуть и с помощью этой функции
fileInfo, err := os.Stat(`1.txt`)

//<< Name()
// имя файла
fmt.Println(fileInfo.Name()) // 1.txt

//<< Size()
// размер в байтах
fmt.Println(fileInfo.Size()) // 0

//<< Mode()
// права
fmt.Println(fileInfo.Mode()) // -rwxr-xr-x

//<< ModTime()
// дата последнего изменения
fmt.Println(fileInfo.ModTime()) // 2024-08-07 11:59:59.440306551 +0400 +04

//>> ПРОВЕРКА НА ОТСУТСТВИЕ ФАЙЛА 
//<< IsNotExist(err)
// возвращает true, если ошибка указывает что файл не существует
fmt.Println(os.IsNotExist(err)) // false

//<< IsExist(err)
// возвращает true, если ошибка указывает что файл существует
fmt.Println(os.IsExist(err)) // fals