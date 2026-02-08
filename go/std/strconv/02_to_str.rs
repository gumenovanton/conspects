//>> ПАРСИНГ В СТРОКУ
// ~ Itoa(int) - int в строку
// возвращает значение и ошибку
fmt.Println(strconv.Itoa(11)) // 11

//<< FormatBool(val) - bool в строку
// возвращает значение и ошибку
fmt.Println(strconv.FormatBool(true)) // true

//<< FormatInt(val, base) - int64 в строку
// возвращает значение в нужной системе счисления и ошибку
fmt.Println(strconv.FormatInt(10, 2)) // 1010

//<< FormatUint(val, base) - uint64 в строку
// возвращает значение в нужной системе счисления и ошибку
fmt.Println(strconv.FormatUint(10, 8)) // 12

//<< FormatFloat(val, format, precision, size) - float в строку
// возвращает значение и ошибку
// format - формат числа
// 		f - ddd.ddd без экспоненты
// 		e,E - с экспонентой
// 		g,G - без экспоненты для маленьких значений, с экспонентой для больших
// precision - количество знаков после запятой
// 		-1 - без знаков после запятой
fmt.Println(strconv.FormatFloat(12.225566, 'f', 2, 64)) // 12.22

