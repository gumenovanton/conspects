//>> УПРАВЛЕНИЕ ЗНАЧЕНИЯМИ ВРЕМЕНИ
london, _ := time.LoadLocation("Europe/London")
saratov, _ := time.LoadLocation("Europe/Saratov")

time1 := time.Now()
time2 := time.Now()

fmt.Println()
fmt.Println(time1) // 2024-08-04 18:42:15.244024872 +0400 +04 m=+0.000178243
fmt.Println(time2) // 2024-08-04 18:42:15.244027177 +0400 +04 m=+0.000180547

//<< Add(duration)
// добавляет time.Duration ко времени
fmt.Println(time1.Add(time.Hour)) // 2024-08-04 19:42:15.244024872 +0400 +04 m=+3600.000178243

//<< AddDate(y, m, d)
// добавляет число, месяц, год ко времени
fmt.Println(time1.AddDate(1, 1, 1)) // 2025-09-05 18:42:15.244024872 +0400 +04

//<< After(time)
// true если time1 после time2
fmt.Println(time1.After(time2)) // false

//<< Before(time)
// true если time1 до time2
fmt.Println(time1.Before(time2)) // true

//<< Equal(time)
// true если time1 = time2
fmt.Println(time1.Equal(time2)) // false

//<< IsZero()
// true если time1 1 января 1 года 00,00,00 UTC
fmt.Println(time1.IsZero()) // false

//<< In()
// time1 в локации
fmt.Println(time1.In(london))  // 2024-08-04 15:42:15.244024872 +0100 BST
fmt.Println(time1.In(saratov)) // 2024-08-04 18:42:15.244024872 +0400 +04

//<< Location()
// Location для time1
fmt.Println(time1.Location()) // Local

//<< Round(duration)
// округляет time1 до duration
fmt.Println(time1.Round(time.Hour)) // 2024-08-04 19:00:00 +0400 +04

//<< Truncate(duration)
// округляет time1 до duration к нижнему значению
fmt.Println(time1.Truncate(time.Hour)) // 2024-08-04 18:00:00 +0400 +04

//<< Sub(time)
fmt.Println(time1.Sub(time2)) // -2.304µs