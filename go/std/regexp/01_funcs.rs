//>> ПРЯМОЕ СОПОСТОВЛЕНИЕ
// в самом пакете regexp не так много ункций доступных на прямую
// здесь самые популярные
//<< MatchString(pattern, s)
// есть ли вхождение шаблона в строке
description := "A boat for one person"
match, err := regexp.MatchString("[A-z]oat", description)
fmt.Println(match, err) // true nil

//<< Match(pattern, b)
// соответствует ли шаблон байтовому срезу
match, err = regexp.Match("foo.?", []byte("seafood fool"))
fmt.Println(match, err) // true nil

//>> КОМПИЛЯЦИЯ СТРОКИ ДЛЯ ПОВТОРОНОЕ ИСПОЛЬЗОВАНИЕ
// для получения больших возможностей
// паттерн регулярного выражения нужно вначале скомпилировать
// а потом для скомпилированного паттерна очень большое количество методов
// это полезно для избежания ошибок

//<< Compile(pattern)
// компилирует паттерн для повторного использования использования
pattern, _ := regexp.Compile("[A-z]oat")
fmt.Println(pattern) // [A-z]oat

//<< MustCompile(pattern)
// то же самое, но выдает панику если паттерн не может быть скомпелирован
// неу и возвращает только одно значение
pattern = regexp.MustCompile("[A-z]oat")

//>> РАБОТА С СКОМПИЛИРОВАННЫМИ ПАТТРНАМИ
//<< MatchString(s)
// есть ли вхождение шаблона в строке
match = pattern.MatchString(description)
fmt.Println(match) // true

//<< FindStringIndex(s)
// возвращает расположение смого левого вхождения паттерна в строку
sl := pattern.FindStringIndex(description)
fmt.Println(sl) // [2 6]

attern.FindAllString//<< FindAllStringIndex(s, max)
// в:озвращает max вхождений паттерна в строку
sl2 := pattern.FindAllStringIndex(description+`boat`, 5)
fmt.Println(sl2) // [[2 6] [21 25]]

//<< FindString(s)
// возвращает первое вхождение паттерна в строку
s := pattern.FindString(description)
fmt.Println(s) // boat

//<< FindAllString(s, n)
// возвращает строку самого левого совпадения
// если n=-1 будет искать все совпадения
sa := pattern.FindAllString(description+`boat`, 5)
fmt.Println(sa) // [boat boat]

//<< Split(s, max)
// делит строку на через разделители, удовлетворяющие паттерну
sa = pattern.Split(description+`boat`, 5)
fmt.Println(sa) // [A   for one person ]

//>> РАБОТА С ГРУППАМИ
// нужно чтобы доставать несколько значений из шаблонной строки
// такой как выборка папок и прав по команде ls -l

// группа записываются в ()
// все регулярное выражение считается нулевой группой
// а все что в скобках идет по порядку от 1
description := "A boat for one person, fuckeng shit! A boat for one person"
pattern := regexp.MustCompile("A ([A-z]*) for ([A-z]*) person")

//<< FindStringSubmatch(s)
// возвращает массив, где первый элемент 0 группа
// а дальше слова соответствующие шаблону
sa := pattern.FindStringSubmatch(description)
fmt.Println(sa) // [A boat for one person boat one]

//<< FindStringSubmatchIndex(s)
// то же самое что и FindStringSubmatch но возвращает индексы
// начала и конца нулевой группы и остальных
sp := pattern.FindStringSubmatchIndex(description)
fmt.Println(sp) // [0 21 2 6 11 14]

//<< FindAllStringSubmatch(s, max)
// тио же самое, что FindStringSubmatch
// но вытащит max вхождений из повторяющихся шаблонных строк
st := pattern.FindAllStringSubmatch(description, 2)
fmt.Println(st) // [[A boat for one person boat one] [A boat for one person boat one]]

//<< FindAllStringSubmatchIndex(s, max)
// тио же самое, что FindAllStringSubmatch, только вытащит индексы
k := pattern.FindAllStringSubmatchIndex(description, 2)
fmt.Println(k) // [[0 21 2 6 11 14] [37 58 39 43 48 51]]

//<< NumSubexp()
// количество групп в паттерне
fmt.Println(pattern.NumSubexp()) // 2

//>> ПРОСТАЯ з

//>> РАБОТА С ИМЕНОВАННЫМИ ГРУППАМИ
// группу можно именовать
// и доставать значения по группам
pattern = regexp.MustCompile("A (?P<type>[A-z]*) for (?P<capacity>[A-z]*) person")

//<< SubexpIndex(name)
// индекс группы с указанным именем
fmt.Println(pattern.SubexpIndex(`type`)) // 1

//<< SubexpNames()
// все имена групп в том порядке в котором они определены
fmt.Println(pattern.SubexpNames()) // [ type capacity] - первый пустая строка, так как вся строка есть нулевая группа

subs := pattern.FindStringSubmatch(description)
fmt.Println(subs)
// так я могу вытащить полдстроки по именам
// просто итерируюсь по массивы
for _, name := range pattern.SubexpNames() {
	if name != "" {
		fmt.Println(name, "=", subs[pattern.SubexpIndex(name)])
	}
}
// type = boat
// capacity = one

//>> ЗАМЕНА ПОДСТРОК

//<< ReplaceAllString(s, template)
// заменяет совпадение по регулярному выражению на текст по шаблону
// во всех строках
// можно использовать для удаления символов по регулярному выражению

// компилируем регулярное выражение
pattern := regexp.MustCompile("A (?P<type>[A-z]*) for (?P<capacity>[A-z]*) person")
description := "Kayak. A boat for one person. Kayak. A boat for one person."

// пишем шаблон для вставки, у указанием имен групп
template := "(type: ${type}, capacity: ${capacity})"

// заменяем
replaced := pattern.ReplaceAllString(description, template)
fmt.Println(replaced) // Kayak. (type: boat, capacity: one). Kayak. (type: boat, capacity: one).

//<< ReplaceAllLiteralString(s, sub)
// простая замена вхождений, на строку
replaced = pattern.ReplaceAllLiteralString(description, `shit`)
fmt.Println(replaced) // Kayak. shit. Kayak. shit.

//<< ReplaceAllStringFunc(s, func)
// заменяет все вхождения по функции
pattern = regexp.MustCompile("A (?P<type>[A-z]*) for (?P<capacity>[A-z]*) person")
description = "Kayak. A boat for one person."
replaced = pattern.ReplaceAllStringFunc(description,
	func(s string) string {
		return "This is the replacement content"
	})
fmt.Println(replaced) // Kayak. This is the replacement content.

//>> САМЫЕ ВАЖНЫЕ 
//<< FindString(s)
