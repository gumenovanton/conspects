//>> FUNCS
//<< Valid()
//- is the byte array a valid UTF  
bytes := []byte("Hello, 世界!")
valid := utf8.Valid(bytes)
fmt.Println(valid) // Выводит true

//<< RuneCountInString()
//- count runes
str := "Привет, мир!"
count := utf8.RuneCountInString(str)
fmt.Println(count) // Выводит 12

//<< DecodeRune()
//- first rune in byte array 
bytes := []byte("Привет, мир!")
runeValue, _ := utf8.DecodeRune(bytes)
fmt.Printf("%c\n", runeValue) // Выводит "П"

//<< EncodeRune()
//- rune to byte arr
runeValue := 'п'
bytes := make([]byte, utf8.UTFMax)
encodedBytes := utf8.EncodeRune(bytes, runeValue)
fmt.Println(encodedBytes) // Выводит 2
fmt.Println(bytes[:encodedBytes]) // Выводит [208 191]
		
//<< RuneLen()
//- rune len
runeValue := '世'
length := utf8.RuneLen(runeValue)
fmt.Println(length) // Выводит 4

//<< RuneStart()
//- is the byte a start of rune
bytes := []byte("Hello, 世界!")
start := utf8.RuneStart(bytes[7])
fmt.Println(start) // Выводит true

//<< RuneError()
//- rune Error
runeError := utf8.RuneError
fmt.Printf("%U\n", runeError) // Выводит "U+FFFD"