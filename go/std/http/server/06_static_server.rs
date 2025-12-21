//>> ОБРАБОТЧИК СТАТИЧЕСКИХ ФАЙЛОВ
// создаем хендлер с каталогом откуда берем файлы
fsHandler := http.FileServer(http.Dir("./static"))
// обрежем путь, убрав /files, чтобы внутри FileServer
// сразу получать доступ к файлам
// по url - http://localhost:5000/files/index.html
// достанет файл по пути static/index.html
http.Handle("/files/", http.StripPrefix("/files", fsHandler))
http.ListenAndServe(":5000", nil)

//<< отчистка пути 
// в путь к файлу может быть заложена атака
// которая сканирует папки
// чтобы это не допустить
// FileServer убирает все . и .. из пути

//>> РАЗДАЧА ОДНОГО ФАЙЛА
// если я хочу по какому то пути отдать какой-то один файл
// использую ServeFile
func downloadHandler(w http.ResponseWriter, r *http.Request) {
    http.ServeFile(w, r, "./ui/static/file.zip")
}

//<< отчистка пути
// ServeFile не чистит путь, и при обработке запроса
// самому нужно побеспокоится о безопасности
// его нужно самому чистить с помощью
filepath.Clean()