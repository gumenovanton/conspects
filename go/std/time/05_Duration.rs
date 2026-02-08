//>> КОНСТАНТЫ DURATION
// time.Hour
// time.Minute
// time.Second
// time.Millisecond
// time.Microsecond
// time.Nanosecond

//>> DURATION В ЕДЕНИЦАХ
dur := time.Hour + (30 * time.Minute)
fmt.Println()
fmt.Println(dur.Hours())             // 1.5
fmt.Println(dur.Seconds())           // 5400
fmt.Println(dur.Milliseconds())      // 5400000
fmt.Println(dur.Microseconds())      // 5400000000
fmt.Println(dur.Nanoseconds())       // 5400000000000
fmt.Println(dur.Round(time.Hour))    // 2h0m0s
fmt.Println(dur.Truncate(time.Hour)) // 1h0m0s

//>> DURATION ОТНОСТИЕЛЬНО ВРЕМЕНИ
timeNow := time.Now()
fmt.Println(time.Since(timeNow.Add(-555665))) // 788.312µs
fmt.Println(time.Until(timeNow.Add(-555665))) // -794.109µs

//>> DURATION ИЗ СТРОК
// h - Эта единица обозначает часы.
// m - Эта единица обозначает минуты.
// s - Эта единица обозначает секунды.
// ms - Эта единица обозначает миллисекунды.
// us - или μs Эта единица обозначает микросекунды.
// ns - Эта единица обозначает наносекунды.

dur, _ = time.ParseDuration("1h30m")
fmt.Println(dur) // 1h30m0s