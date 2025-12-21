//>> FOR
//<< бесконечный цикл
for{
    continue    // пеерйти на следующую итерацию
    break       // закончить цикл 
}

//<< цикл с условием
counter := 0
for (counter <= 3) {
    fmt.Println("Counter:", counter)
    counter++
}

//<< классический цикл
for counter := 0; counter <= 3; counter++ {
    fmt.Println("Counter:", counter)
}

//<< итерация по последовательностям
// здесь каждый символ типа rune
product := "Kayak"
for index, character := range product {
    fmt.Println("Index:", index, "Character:", string(character))
}
// Index: 0 Character: K
// Index: 1 Character: a
// Index: 2 Character: y
// Index: 3 Character: a
// Index: 4 Character: k

//>> НЮАНСЫ FOR RANGE
// при такой итерации всегда возвращается индекс или ключ и копия значения
// если нужно поменять значение то нужно обращаться по индексу или ключу