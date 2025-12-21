//>> SCANNER
// подходит для парсинга файлов, ввода, построчно
// можно просто использовать Reder
// но сканнер удобнее, для чтения по строкам

//<< NewScanner()
// create scanner
lscanner := bufio.NewScanner(os.Stdin)

//<< Scan()
// scan from reader
for scanner.Scan() {		

    //<< Bytes(), Text()
    // get text or bytes
    fmt.Println(scanner.Text())
    fmt.Println(scanner.Bytes())
}

