//>> ПРОВЕРКА СТРОК
//<< Count(s, sub)
// сколько раз подстрока в строке
fmt.Println(strings.Count("My Name is Anton Anton", "Anton")) // 2

//<< Index(s, sub)
// первое вхождение подстроки в строке
// или -1 если вхождения нет
fmt.Println(strings.Index("My Name is Anton Anton", "Anton")) // 11

//<< LastIndex(s, sub)
// последнее вхождение подстроки в строке
// или -1 если вхождения нет
fmt.Println(strings.LastIndex("My Name is Anton Anton", "Anton")) // 17

//<< IndexAny(s, sub)
// первое вхождение либого сивмола подстроки в строке
// или -1 если вхождения нет
fmt.Println(strings.IndexAny("My Name is Anton Anton", "At")) // 11

//<< LastIndexAny(s, sub)
// последнее вхождение либого сивмола подстроки в строке
// или -1 если вхождения нет
fmt.Println(strings.LastIndexAny("My Name is Anton Anton", "At")) // 19

//<< IndexByte(s, b)
// первое вхождение байта в строке
// или -1 если вхождения нет
fmt.Println(strings.IndexByte("My Name is Anton Anton", 5)) // -1

//<< LastIndexByte(s, b)
// последнее вхождение байта в строке
// или -1 если вхождения нет
fmt.Println(strings.LastIndexByte("My Name is Anton Anton", 5)) // -1

//<< IndexFunc(s,func)
//<< LastIndexFunc(s,func)
// акрвое и последнее вхождение символа, удовлетворяющего результатам функции
description := "A boat for one person"
isLetterB := func(r rune) bool {
	return r == 'B' || r == 'b'
}
fmt.Println("IndexFunc:", strings.IndexFunc(description, isLetterB)) // IndexFunc: 2,
