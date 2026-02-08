//>> IF
// каждая {} имеет свою область видимости
l := 5
if l == 5 {
	fmt.Println(l)
} else if l == 4 {
	fmt.Println(l)
} else {
	fmt.Println(l)
}

//>> IF c ИНИЦИАЛИЗАЦИЕЙ
// при использловании с инициализацией прееменные доступны только внутри if
if l,err := strconv.Atoi("5"); l==5{
	fmt.Println(l)
} else if l == 4 {
	fmt.Println(l)
} else {
	fmt.Println(err)
}
