//>> ИЗМЕНЕНИЕ СТРОК
//<< Replace(s, old, new, n)
// заменяет n вхождений old на new в строке s
fmt.Println(strings.Replace("Sam is the Sam's futher", "Sam", "John", 1)) // John is the Sam's futher

//<< ReplaceAll(s, old, new, n)
// заменяет все вхождения old на new в строке s
fmt.Println(strings.ReplaceAll("Sam is the Sam's futher", "Sam", "John")) // John is the John's futher

//<< Map(func, s)
// генерирует строку для каждого символа в строке s
// если функция генерирует отрицательное значение, то символ отбрасывается
text := "It was a boat. A small boat."
mapper := func(r rune) rune {
	if r == 'b' {
		return 'c'
	}
	return r
}
mapped := strings.Map(mapper, text)
fmt.Println("Mapped:", mapped) // Mapped: It was a coat. A small coat.

//<< Replacer(s)
// возвращает новую строку с заменой парами
text = "It was a boat. A small boat."
// "boat" на "kayak", "small" на "huge"
replacer := strings.NewReplacer("boat", "kayak", "small", "huge")
replaced := replacer.Replace(text)
fmt.Println("Replaced:", replaced) // Replaced: It was a kayak. A huge kayak.

//<< WriteString(writer, s)
// заменяет так же парами но пишет в writer
file, _ := os.Create("1.txt")
replacer.WriteString(file, text)   // запишет измененую строку в файл