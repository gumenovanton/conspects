//>> ВАЖНО
// все эти функции работают с float64

//>> ЗНАКИ
//<< Abs(val)
// абсолютное число, число без знака
fmt.Println(math.Abs(17.35))  // 17.35
fmt.Println(math.Abs(-17.35)) // 17.35

//<< Copysign(x,y)
// копирование знаков для x от y
fmt.Println(math.Copysign(17.35, -7)) // -17.35
fmt.Println(math.Copysign(-17.35, 7)) // 17.35

//>> ОКРУГЛЕНИЕ ЗНАЧЕНИЙ
//<< Ceil(val)
// округление до большего
fmt.Println(math.Ceil(15.55)) // 16
fmt.Println(math.Ceil(15.45)) // 16

//<< Floor(val)
// округление до меньшего
fmt.Println(math.Floor(15.55)) // 15
fmt.Println(math.Ceil(15.45))  // 16

//<< Round(val)
// округление до ближайшего, средние значения в большую
fmt.Println(math.Round(15.55)) // 16
fmt.Println(math.Round(15.50)) // 16

//<< RoundToEven(val)
// округление до ближайшего, средние значения в четную сторону
fmt.Println(math.RoundToEven(14.55)) // 15
fmt.Println(math.RoundToEven(14.50)) // 14
fmt.Println(math.RoundToEven(13.50)) // 14

//>> БОЛЬШОЕ И МЕНЬШЕЕ
//<< Max(x,y)
fmt.Println(math.Max(3, 4)) // 4

//<< Min(x,y)
fmt.Println(math.Min(3, 4)) // 3

//>> ВОЗВЕДЕНИЕ В СТЕПЕНЬ
//<< Pow(x, y)
// х в степень у
fmt.Println(math.Pow(3, 4)) // 81

//>> КОНСТАНТЫ МАКСИМАЛЬНЫХ ЗНАЧЕНИЙ
fmt.Println(math.MaxInt8)                //127
fmt.Println(math.MinInt8)                //-128
fmt.Println(math.MaxInt16)               //32767
fmt.Println(math.MinInt16)               //-32768
fmt.Println(math.MaxInt32)               //2147483647
fmt.Println(math.MinInt32)               //-2147483648
fmt.Println(math.MaxInt64)               //9223372036854775807
fmt.Println(math.MinInt64)               //-9223372036854775808
fmt.Println(math.MaxUint8)               //255
fmt.Println(math.MaxUint16)              //65535
fmt.Println(math.MaxUint32)              //4294967295
// почему то тут ошибка переполнения
// fmt.Println(math.MaxUint64)				// 18446744073709551615
fmt.Println(math.MaxFloat32)             //3.4028234663852886e+38
fmt.Println(math.MaxFloat64)             //1.7976931348623157e+308
fmt.Println(math.SmallestNonzeroFloat32) //1.401298464324817e-45
fmt.Println(math.SmallestNonzeroFloat64) //5e-324