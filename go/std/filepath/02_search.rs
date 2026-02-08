//>> ПОИСК ФАЙЛОВ ПО ПАТТЕРНУ
//<< Glob(pathPatten)
// озвращает string срез содержащий совпадающие пути
sl, err := filepath.Glob("/store/dev/*")
fmt.Println(sl, err) // [/store/dev/knowlege /store/dev/projects /store/dev/projects_to_learn] <nil>

//>> ОБРАБОТКА ВСЕХ ФАЙЛОВ В КАТАЛОГЕ
//<< WalkDir(directory, func)
// делает действие для каждого обьекта в папке
// принимает функцию,
// которая принимает путь, DirEntry и dirErr

func callback(path string, dir os.DirEntry, dirErr error) (err error) {
	info, _ := dir.Info()
	fmt.Printf("Path %v, Size: %v\n", path, info.Size())
	return
}
	
dir, _ := os.Getwd()
filepath.WalkDir(dir, callback)
// Path /store/dev/projects_to_learn/playground, Size: 36
// Path /store/dev/projects_to_learn/playground/1.txt, Size: 0
// Path /store/dev/projects_to_learn/playground/go.mod, Size: 29
// Path /store/dev/projects_to_learn/playground/main.go, Size: 317
