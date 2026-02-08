// _ ОБРЕЗКА СТРОК
// ~ TrimSpace(s)
// обрезка начальных и конечных пробелов
fmt.Println(strings.TrimSpace(" Hi John, I am Teddy  ")) // Hi John, I am Teddy

// ~ Trim(s, set)
// обрезка начальных и конечных символов из набора
fmt.Println(strings.Trim(" Hi John, I am Teddy  ", " Hy")) // i John, I am Tedd

// ~ TrimLeft(s, set)
// обрезка начальных символов из набора
fmt.Println(strings.TrimLeft(" Hi John, I am Teddy  ", " Hy")) // i John, I am Teddy

// ~ TrimRight(s, set)
// обрезка конечных символов из набора
fmt.Println(strings.TrimRight(" Hi John, I am Teddy  ", " Hy")) // Hi John, I am Tedd

// ~ TrimPrefix(s, prefix)
// обрезка префикса
fmt.Println(strings.TrimPrefix(" Hi John, I am Teddy  ", " Hi ")) // John, I am Teddy

// ~ TrimSuffix(s, suffix)
// обрезка суфикса
fmt.Println(strings.TrimSuffix(" Hi John, I am Teddy  ", " Teddy  ")) // Hi John, I am

// ~ TrimFunc(s, func)
// обрезка начальных и конечных символов по функции
trimFunc := func(r rune) bool {
	return r == '_'
}
fmt.Println(strings.TrimFunc("_Hi John, I am Teddy_", trimFunc)) // Hi John, I am Teddy

// ~ TrimLeftFunc(s, func)
// обрезка начальных и конечных символов по функции
fmt.Println(strings.TrimLeftFunc("_Hi John, I am Teddy_", trimFunc)) // Hi John, I am Teddy_

// ~ TrimRightFunc(s, func)
// обрезка начальных и конечных символов по функции
fmt.Println(strings.TrimRightFunc("_Hi John, I am Teddy_", trimFunc)) // _Hi John, I am Teddy