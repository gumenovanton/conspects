//>> ЧТЕНИЕ ИЗ КОНСОЛИ
//<< Scan(...vals)
// сканирует ввод через пробел или после нажатия Enter
// если цифры не парсятся выведет значение по умолчанию
var name string
var category string
var price float64
fmt.Print("Enter text to scan: ")
n, _ := fmt.Scan(&name, &category, &price) // goo htht 99
fmt.Println(n, name, category, price)      // 3 goo htht 99

//<< Scanln(...vals)
// то же самое но закончит ввод если нажать Enter
n, _ := fmt.Scanln(&name, &category, &price)
fmt.Println(n, name, category, price)

//<< Scanf(template,...vals)
// вытащит значения из введенной строки по шаблону
n, _ := fmt.Scanf("name:%s category:%s price:%f", &name, &category, &price) // name:bx1 category:dron price:66.88
fmt.Println(n, name, category, price)                                       // 3 bx1 dron 66.88

//>> ЧТЕНИЕ ИЗ ФАЙЛА
//<< Fscan(reader,...vals)
// считывает значения из файла через пробел
file, _ := os.OpenFile(`1.txt`, os.O_RDONLY, 0666)
n, _ := fmt.Fscan(file, &name, &category, &price) // bx1 dron 66.88 в файле
fmt.Println(n, name, category, price)             // 3 bx1 dron 66.88

//<< Fscanln(reader,...vals)
// делает то же самое но как увидит переход на новую строку остановит сканирование
ile, _ := os.OpenFile(`1.txt`, os.O_RDONLY, 0666)
n, _ := fmt.Fscanln(file, &name, &category, &price) // bx1 dron \n66.88 в файле
fmt.Println(n, name, category, price)               // 2 bx1 dron 0

//<< Fscanf(reader,template,...vals)
// считывает с файла по шаблону
n, _ := fmt.Fscanf(file, "name:%s category:%s price:%f", &name, &category, &price) // name:bx1 category:dron price:66.88
fmt.Println(n, name, category, price)                                              // 3 bx1 dron 66.88                                             // 3 bx1 dron 66.88

//>> ЧТЕНИЕ ИЗ СТРОКИ
//<< Sscan(str,...vals)
// сканирует значения через пробел из строки в переменные
n, _ := fmt.Sscan("black tshirt 55.66", &name, &category, &price)
fmt.Println(n, name, category, price)

//<< Sscanf(str,template,...vals)
// достает значения из строки по шаблону
n, _ := fmt.Sscanf("name:bx1 category:dron price:66.88", "name:%s category:%s price:%f", &name, &category, &price)
fmt.Println(n, name, category, price) // 3 bx1 dron 66.88

// ~ Sscanln(str,template,...vals)
// то же самое что и Sscanf но остановит сканирование при переносе строки
n, _ := fmt.Sscanf("name:bx1 category:dron price:\n66.88", "name:%s category:%s price:%f", &name, &category, &price)
fmt.Println(n, name, category, price) // 2 bx1 dron 0

