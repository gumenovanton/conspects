//>> РАБОТА С ПУТЯМИ
//<< ключевые пути
// текущая
fmt.Println(os.Getwd()) // /store/dev/projects_to_learn/playground <nil>
// домашняя
fmt.Println(os.UserHomeDir()) // /home/ang <nil>
// кэш
fmt.Println(os.UserCacheDir()) // /home/ang/.cache <nil>
// конфиги
fmt.Println(os.UserConfigDir()) // /home/ang/.config <nil>
// темп
fmt.Println(os.TempDir()) // /tmp

//>> ФУНКЦИИ ДЛЯ РАБОТЫ С ПУТЯМИ
wPath, _ := os.Getwd()

//<< Abs(path)
// возвращает абсолютный путь из относительного
fmt.Println(filepath.Abs(wPath)) // /store/dev/projects_to_learn/playground <nil>

//<< IsAbs(path)
// true если путь абсолютный
fmt.Println(filepath.IsAbs(wPath)) // true

//<< Base(path)
// последний элемент пути
fmt.Println(filepath.Base(wPath)) // playground

//<< Clean(path)
// отчищает от повторяющихся разделителей и относительных ссылок
fmt.Println(filepath.Clean(wPath)) // /store/dev/projects_to_learn/playground

//<< Dir(path)
// все элементы пути кроме последнего
fmt.Println(filepath.Dir(wPath)) // /store/dev/projects_to_learn

//<< EvalSymlinks(path)
// возвращает путь по символической ссылке
fmt.Println(filepath.EvalSymlinks(wPath)) // /store/dev/projects_to_learn/playground <nil>

//<< Ext(path)
// расширение файла
newPath := filepath.Join(wPath, `home`, `ang.txt`)
fmt.Println(filepath.Ext(newPath)) // .txt

//<< FromSlash(path)
// заменяет каждую косую черту символом разделителя для используемой платформы
fmt.Println(filepath.FromSlash(wPath)) // /store/dev/projects_to_learn/playground

//<< ToSlash(path)
// заменяет разделитель платформы косой чертой
fmt.Println(filepath.ToSlash(wPath)) // /store/dev/projects_to_learn/playground

//<< Join(...elements)
// объединяет несколько элементов, используя файловый разделитель платформы
fmt.Println(filepath.Join(wPath, `home`, `ang`)) // /store/dev/projects_to_learn/playground/home/ang

//<< Match(pattern, path)
// true, усли путь соответствует шаблону
// паттерн по правилам регулярных выражений
fmt.Println(filepath.Match("/home/ang/*", "/home/ang/file.txt")) // true <nil>

//<< Rel(base, path)
// относительный путь от базы
fmt.Println(filepath.Rel(`/home`, `/home/ang/file.txt`)) // ang/file.txt

//<< Split(path)
// возвращает директорию и имя файла по отдельности
// если путь папки, то отделит конечную папку от остального пути
// вернув два значения
fmt.Println(filepath.Split(`home/file.txt`)) //home/ file.txt

//<< SplitList(path)
// путь по компанентам в срез
fmt.Println(filepath.SplitList("/a/b/c:/usr/bin")) // [/a/b/c /usr/bin]

//<< VolumeName(path)
// возвращает компонент тома или пустую строку если тома не
fmt.Println(filepath.VolumeName("C:\foo\bar")) // C: на виндоу