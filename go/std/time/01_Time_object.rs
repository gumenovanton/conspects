//>> СОЗДАНИЕ ОБЬЕКТА Time
//<< Now()
// вернет Time представляющая текущий момент
fmt.Println(time.Now()) // 2024-08-04 17:22:50.348158971 +0400 +04 m=+0.000024164

//<< Date(y, m, d, h ,min, sec, nsec ,loc)
// создает обьект Time
fmt.Println(time.Date(1995, time.June, 9, 0, 0, 0, 0, time.Local)) // 1995-06-09 00:00:00 +0400 +04

//<< Unix(sec, nsec)
// создает обьект Time из значений секунд и наносекунд с 1 января 1970 года
fmt.Println(time.Unix(1433228090, 0)) // 2015-06-02 09:54:50 +0300 +03

//>> ДОСТУП К КОМПОНЕНТАМ ОБЬЕКТА Time
timeNow := time.Now()

fmt.Println(timeNow.Date())       // 2024 August 4
fmt.Println(timeNow.Clock())      // 17 29 5
fmt.Println(timeNow.Year())       // 2024
fmt.Println(timeNow.YearDay())    // 217
fmt.Println(timeNow.Month())      // August
fmt.Println(timeNow.Day())        // 4
fmt.Println(timeNow.Weekday())    // Sunday
fmt.Println(timeNow.Hour())       // 17
fmt.Println(timeNow.Minute())     // 29
fmt.Println(timeNow.Second())     // 5
fmt.Println(timeNow.Nanosecond()) // 209760071