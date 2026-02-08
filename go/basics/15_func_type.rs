//>> ТИП ФУНКЦИЯ
// позволяет определять тип передаваемой и возвращаемой функции
// позволяет создавать переменные типа функции

// функция
func display(message string){
}
// имеет тип func(string)

//<< создание переменной типа функции
// пусть есть функция
func add(x int, y int) int{
    return x + y
}

// создаю переменную ее типа которая равна этой функции
var f func(int, int) int = add

// теперь могу создать экземпляр этой функции с конкретными значениями
var x = f(4, 5) // 9

//>> ФУНКЦИЯ КАК ПАРАМЕТР ДРУГИХ ФУНКЦИЙ
func add(x int, y int) int {
     
    return x + y
}
func multiply(x int, y int) int {
     
    return x * y
}
func action(n1 int, n2 int, operation func(int, int) int){
 
    result := operation(n1, n2)
    fmt.Println(result)
}
func main() {
     
    action(10, 25, add)     // 35
    action(5, 6, multiply)  // 30
}

//>> ФУНКЦИЯ КАК РЕЗУЛЬТАТ ДРУГОЙ ФУНКЦИИ
func add(x int, y int) int{ return x + y}
func subtract(x int, y int) int{ return x - y}
func multiply(x int, y int) int{ return x * y}
 
func selectFn(n int) (func(int, int) int){
    if n==1 {
        return add
    }else if n==2{
        return subtract
    }else{
        return multiply
    }
}
 
func main() {
     
    f := selectFn(1)
    fmt.Println(f(3, 4))        // 7
     
    f = selectFn(3)
    fmt.Println(f(3, 4))        // 12
}

//>> СОЗДАНИЕ ПСЕВДАНИМОВ ФУНКЦИОНАЛЬНЫХ ТИПОВ
type calcFunc func(float64) float64