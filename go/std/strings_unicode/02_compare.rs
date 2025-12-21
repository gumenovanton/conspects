//>> СРАВНЕНИЕ СТРОК
//<< Contains(str, substr)
// содержит ли строка подстроку
fmt.Println(strings.Contains("John is god", "ohn")) // true

//<< ContainsAny(str, substr)
// содержит ли строка любой из символов в подстроке
fmt.Println(strings.ContainsAny("John is god", "Moo")) // true

//<< ContainsRune(str, rune)
// содержит ли строка руну
fmt.Println(strings.ContainsRune("€48.95", '€')) // true

//<< EqualFold(str1, str2)
// сравнение строк без учета регистра
fmt.Println(strings.EqualFold("Строка", "сТроКа")) // true

//<< HasPrefix(str1, prefix)
// Начинается ли строка с префикса
fmt.Println(strings.HasPrefix("Строка", "Ст")) // true

//<< HasSuffix(str1, sufix)
// Заканчивается ли строка суфиксом
fmt.Println(strings.HasSuffix("Строка", "ока")) // true
