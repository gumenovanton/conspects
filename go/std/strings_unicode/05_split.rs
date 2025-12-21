// _ РАЗДЕЛЕНИЕ СТРОК
//<< Fields(s)
// Возвращает слайс слов через пробел
fmt.Println(strings.Fields("Hi John, I am Teddy")) // [Hi John, I am Teddy]

//<< FieldsFunc(s, func)
// Возвращает слайс слов с разделителем по результату функции
isLetter_ := func(r rune) bool {
	return r == '_' || r == ','
}
fmt.Println(strings.FieldsFunc("Hi_John_,_I_am_Teddy", isLetter_)) // [Hi John I am Teddy]

//<< Split(s, sub)
// Возвращает слайс слов через разделитель подстроку
fmt.Println(strings.Split("Hi_John_,_I_am_Teddy", "_")) // [Hi John , I am Teddy]

//<< SplitN(s, sub, max)
// Возвращает слайс слов через разделитель подстроку, но разделит на max частей
fmt.Println(strings.SplitN("Hi_John_,_I_am_Teddy", "_", 3)) // [Hi John ,_I_am_Teddy]

//<< SplitAfter(s, sub)
// Возвращает слайс слов через разделитель подстроку, включая подстроку
fmt.Println(strings.SplitAfter("Hi_John_,_I_am_Teddy", "_")) // [Hi_ John_ ,_ I_ am_ Teddy]

//<< SplitAfterN(s, sub, max)
// Возвращает слайс слов через разделитель подстроку, включая подстроку, но разделит на max частей
fmt.Println(strings.SplitAfterN("Hi_John_,_I_am_Teddy", "_", 3)) // [Hi_ John_ ,_I_am_Teddy]