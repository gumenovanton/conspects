//>> УПРАВЛЕНИЕ ФАЙЛАМИ И КАТАЛОГАМИ
//<< Chdir(dir)
// переход в директорию по пути
path, _ := os.Getwd()
fmt.Println(path) // /store/dev/projects_to_learn/playground
os.Chdir(`/home/ang`)
fmt.Println(os.Getwd()) // /home/ang <nil>
os.Chdir(path)
fmt.Println(os.Getwd()) // /store/dev/projects_to_learn/playground <nil>

//<< Mkdir(name, modePerms)
// созддать директорию
os.Mkdir(`test`, 0777)
os.Mkdir(`test2`, 0777)

//<< MkdirAll(name, modePerms)
// создает папку с родительскими каталогами
os.MkdirAll(`test/test3`, 0777)
// создаст test2 но не test3 если нет прав

//<< MkdirTemp(parentDir, name)
// создает коталог в адресе, или если он пустая строка в Temp,
// добавив случайную строку вместо звездочки или к концу имени
os.MkdirTemp(path, `temp`)

//<< Remove(name)
// удалить файл или каталог
os.Remove(`test`)

//<< RemoveAll(name)
// удалить файл или каталог со всем содержимым
os.RemoveAll(`test2`)

//<< Rename(old, new)
// переименовать файл или каталог
os.Rename(`test`, `test5`)

//<< Symlink(old, new)
// создает символьную ссылку на файл или каталог
os.Mkdir(`yo`, 0777)
os.Symlink(`yo`, `yo1`)
     