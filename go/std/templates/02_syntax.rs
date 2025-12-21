//>> ВСТАВКА ЗНАЧЕНИЯ
// так вставим значение
{{.}}

// так вставим значение поля пеердаваемой структуры
{{.Name}}

// вызов метода без аргументов
{{.Method}}

// вызов метода c аргументами
{{.Method arg arg}}

// вызов встроенной в шаблонизатор функции
{{ func arg }}

// коментарий
{{/* a comment */}}

// строка
{{"\"output\""}}
{{`"output"`}}

//>> ОБРЕЗКА ПРОБЕЛОВ
// убрать пробелы слева
{{- .Name}}

// убрать пробелы справа
{{.Name -}}

//>> ФОРМАТИРОВАНИЕ ЗНАЧЕНИЙ
// дробное число с двумя знаками после запятой
<h1>Price: {{ printf "$%.2f" .Price }} // </h1><h1>Price: $279.00</h1>

// цепочки выражений
// здесь выполнить  функцию и отфарматировать
<h1>Discount Price: {{ .ApplyDiscount 10 | printf "$%.2f" }}// <h1>Discount Price: $269.00</h1>

//>> ОПРЕДЕЛЕНИЕ ПЕРЕМЕННЫХ
{{ define "mainTemplate" -}}
    {{ $length := len . }}
    <h1>There are {{ $length }} products in the source data.</h1>
{{- end }}

//>> ИТЕРАЦИЯ ПО СРЕЗАМ
// если срез содержит елементы
{{ range . -}}
    <h1>Name: {{ .Name }}, Category: {{ .Category }}, Price, {{- printf "$%.2f" .Price }}</h1>
{{ end }}

// вывод по умолчанию, если срез не сожержит элементов
{{ range . -}}
    <h1>Name: {{ .Name }}, Category: {{ .Category }}, Price, {{- printf "$%.2f" .Price }}</h1>
{{ else }}
    <h1>Slice is Empty</h1>
{{ end }}

//<< использование переменных
{{ range $key, $value := . -}}
    <h1>{{ $key }}: {{ printf "$%.2f" $value.Price }}</h1>
{{ end }}

//<< ВСТРОЕННЫЕ ФУНКЦИИ СРЕЗОВ
// длинна среза 
{{ len . }} 

// элемент по индексу
{{ index . 0 }}

// срез из среза
{{ range slice . 3 5 -}}
    <h1>Name: {{ .Name }}, Category: {{ .Category }}, Price, {{- printf "$%.2f" .Price }}</h1>
{{ end }}

// выход из цикла
{{break}}

// продолжение цикла
{{continue}}

//>> IF ELSE
//<< функции сравнения
// eq rg1 arg2     Эта функция возвращает true, если arg1 == arg2.
// ne arg1 arg2    Эта функция возвращает значение true, если arg1 != arg2.
// lt arg1 arg2    Эта функция возвращает значение true, если arg1 < arg2.
// le arg1 arg2    Эта функция возвращает значение true, если arg1 <= arg2.
// gt arg1 arg2    Эта функция возвращает значение true, если arg1 > arg2.
// ge arg1 arg2    Эта функция возвращает значение true, если arg1 >= arg2.
// and arg1 arg2   Эта функция возвращает значение true, если оба параметра arg1 и arg2 являются true.
// not arg1        Эта функция возвращает true, если arg1 является false, и false, если true

//<< синтаксис
{{ if lt .Price 100.00 -}}
    <h1>Name: {{ .Name }}, Category: {{ .Category }},Price, {{- printf "$%.2f" .Price }}</h1>
{{ else if gt .Price 1500.00 -}}
    <h1>Expensive Product {{ .Name }} ({{ printf "$%.2f" .Price}})</h1>
{{ else -}}
    <h1>{{"ELSE"}}</h1>
{{ end -}}

//>> WITH
// если .Price не равно nil то блок будет показан
{{ with .Price }}
    <h1>We have a price</h>
{{ end }}

//>> ОПРЕДЕЛЕНИЕ ШАБЛОНА С УКАЗАННЫМ ИМЕНЕМ
// например в одном файле я определяю шаблон с именем basicProduct и currency
// они не будут отображаться пока их не вызовешь через template
{{ define "basicProduct" -}}
    Name: {{ .Name }}, Category: {{ .Category }}, Price, {{- template "currency" .Price }}
{{- end }}

{{ define "currency" }}{{ printf "$%.2f" . }}{{ end }}

// запустить их я могу в этом же файле 
// или в другом шаблоне так
// передав ав них данные
<h1>{{- template "basicProduct" . }}</h1>
<h1>{{- template "currency" .Price }}</h1>

//>> ИСПОЛЬЗОВАНИЕ БЛОКОВ
// определяют шаблон по умолчанию, 
// который будет выведен по любому
// который заменится если шаблон загрузил из нескольких файлов
// и там есть шаблон на замену
{{ block "body" . }}
    <h2>Default content of {{.Name}}</h2>
{{ end }}

// и если я в другом файле обьявлю шаблон
// и подгружу его, он заменит блок выше
{{ define "body" }}
    <h2>Replase with this</h2>
{{ end }}