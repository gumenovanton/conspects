//>> ПРЕОБРАЗОВАНИЕ РЕГИСТРА
//<< ToLower(s)
// возвращает строку в нижнем регистре
fmt.Println(strings.ToLower("сТРОКА")) // строка

//<< ToUpper(s)
// возвращает строку в верхнем регистре
fmt.Println(strings.ToUpper("Строка")) // СТРОКА

//>> ИЗМЕНЕНИЕ РЕГИСТРА ОТДЕЛЬНЫХ СИМВОЛОВ пакет unicode
// в юникоде буквы могут быть
// большими
// маленькими
// заглавными

//<< IsLower(rune)
// руна в маленьком регистре?
fmt.Println(unicode.IsLower('С')) // false

//<< ToLower
// руну в маленький регистр
fmt.Println(string(unicode.ToLower('С'))) // c

//<< IsUpper
// руна в большом регистре?
fmt.Println(unicode.IsUpper('ж')) // false

//<< ToUpper
// руну в большой регистр
fmt.Println(string(unicode.ToUpper('ж'))) // Ж

//<< IsTitle
// руна заглавная?
fmt.Println(unicode.IsTitle('Ё')) // false

//<< ToTitle
// руну в заглавную
fmt.Println(string(unicode.ToTitle('ё'))) // Ё