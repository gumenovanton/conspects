//>> SWITCH
character := 'k'
switch character {
    case 'r':
        fallthrough // перейти к следующему
    case 'k':
    	fmt.Println("k")
    case 'y':
        break // выйти
    default:
        fmt.Println("nothing run") // сюда если не один не соответствует
}

//<< исключение значений
switch {
    case counter == 0:
        fmt.Println("Zero value")
    case counter < 3:
        fmt.Println(counter, "is < 3")
    case counter >= 3 && counter < 7:
        fmt.Println(counter, "is >= 3 && < 7")
    default:
        fmt.Println(counter, "is >= 7")
}

//>> SWITCH c ИНИЦИАЛИЗАЦИЕЙ
// инициализированное значение можно использовать в блоках
switch val := counter / 2; val {
    case 2, 3, 5, 7:
        fmt.Println("Prime value:", val)
    default:
        fmt.Println("Non-prime value:", val)
}