//>> ПАРСИНГ ЗНАЧЕНИЙ ИЗ СТРОК
//<< ParseBool(str) - парсинг строки в bool
// возвращает значение и ошибку
fmt.Println(strconv.ParseBool("true")) // true nil
fmt.Println(strconv.ParseBool("TRUE")) // true nil
fmt.Println(strconv.ParseBool("True")) // true nil
fmt.Println(strconv.ParseBool("T"))    // true nil
fmt.Println(strconv.ParseBool("t"))    // true nil
fmt.Println(strconv.ParseBool("1"))    // true nil

fmt.Println(strconv.ParseBool("false")) // false nil
fmt.Println(strconv.ParseBool("FALSE")) // false nil
fmt.Println(strconv.ParseBool("False")) // false nil
fmt.Println(strconv.ParseBool("F"))     // false nil
fmt.Println(strconv.ParseBool("f"))     // false nil
fmt.Println(strconv.ParseBool("0"))     // false nil

//<< ParseFloat(str,size) - строку в нужный float
// возвращает значение и ошибку
// size - в какую разрядность положить(32,64)
fmt.Println(strconv.ParseFloat("1.1", 32)) // 1.100000023841858 <nil>
fmt.Println(strconv.ParseFloat("1.1", 64)) // 1.1 <nil>

//<< ParseInt(str, base, size), ParseUint(str, base, size) - строку в нужный int
// возвращает значение и ошибку
// base - в какой системе число(2,8,10,16)
// size - в какую разрядность положить(0,8,10,16,32,64)
fmt.Println(strconv.ParseInt("1100", 2, 0))     // 12 <nil>
fmt.Println(strconv.ParseInt("14", 8, 8))       // 12 <nil>
fmt.Println(strconv.ParseUint("c", 16, 8))      // 12 <nil>
fmt.Println(strconv.ParseUint("12", 10, 64))    // 12 <nil>

//<< при конвертации чисел с префиксом base = 0
fmt.Println(strconv.ParseInt("0b00001100", 0, 8))   // 12 <nil>
fmt.Println(strconv.ParseInt("0b1100", 0, 8))       // 12 <nil>
fmt.Println(strconv.ParseInt("0o14", 0, 8))         // 12 <nil>
fmt.Println(strconv.ParseInt("0xc", 0, 8))          // 12 <nil>

// ~ Atoi(str) - строку в int
// возвращает значение и ошибку
fmt.Println(strconv.Atoi("11")) // 11 <nil>

//! переполнение выдаст ошибку
fmt.Println(strconv.Atoi("111111111111111111111111111111111111111111")) // error

