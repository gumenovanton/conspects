//>> ДЖЕНЕРИКИ
// это обобщения
// в го используются только в функциях и в типах
// при этом обобщая тип я должен указать ограничения
// какие типы могут прилетать в функцию

//! РАБОТАЮТ ГОРАЗДО БЫСТРЕЕ ИНТЕРФЕЙСОВ

//>> ПРИМЕНЕНИЕ 
// когда нужен один и тот же функционал для разных типов

//<< создание функции с жденериками
func Sum[T int64 | float64](numbers ...T) T {
	var sum T
	for _, v := range numbers {
		sum = v + sum
	}
	return sum
}
func main() {
	var q, w, e, r int64 = 3, 2, 1, 5
	fmt.Println(Sum(q, w, e, r)) // 11

	var q1, w1, e1, r1 float64 = 3, 2, 1, 5
	fmt.Println(Sum(q1, w1, e1, r1)) // 11
}

//>> ЯВНОЕ УКАЗАНИЕ ТИПА
// можно явно указать тип передаваемых значений
sum1 := Sum[float64]([]float64{3, 2, 1, 5}...)
fmt.Println(sum1) // 11

//>> ОГРАНИЧЕНИЯ ТИПА
// я могу просто пречислить типы, 
// которые может принимать функции через | (оператор union)

// я могу написать тип интерфейс или несколько интерфейсов, 
// которые где то обьявил, которые может принять функция
// это могут быть обычные интерфейсы 
// так и интерфейсы типов 
// например

type Number interface{
    int64 | ~float64
}
// тильда означает что возможен как тип float64 так и любые производные из float64 типы

//<< встроенные интерфейсы
// compariable - типы которые можно сравнивать
// any - любой тип

//<< пакет "golang.org/x/exp/constraints"
// имеет следующие интерфейсы с огораничениями
// Signed - ~int | ~int8 | ~int16 | ~int32 | ~int64
// Unsigned - ~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr
// Integer - Signed | Unsigned
// Float - ~float32 | ~float64
// Complex - ~complex64 | ~complex128
// Ordered - Integer | Float | ~string

//>> ОБОБЩЕННЫЕ ТИПЫ
// например массив в котором лежат либо int64 либо float64
// но только один тип одновременно, по этому тип нужно уточнять
type Numbers[T int64 | float64] []T

ints := make(Numbers[int64], 10)
ints = append(ints, []int64{3, 3, 2, 5, 8}...)
fmt.Println(ints) // [0 0 0 0 0 0 0 0 0 0 3 3 2 5 8]

floats := make(Numbers[float64], 10)
floats = append(floats, []float64{3, 3, 2, 5, 8}...)
fmt.Println(floats) // [0 0 0 0 0 0 0 0 0 0 3 3 2 5 8]

//<< можно сразу инициализировать так
floats2:=Numbers[float64]{2.2, 6, 5}

//>> ДЖЕНЕРИКИ В СТРУКТУРАХ
// обьявление
type cache[T cacheable] struct {
    data[string]T
}

// методы
func (c *cache[T]) Set(key string, value T) {
    c.data[key] = value
}

func (c *cache[T]) Get(key string) (v T) {
    if v, ok := c.data[key]; ok {
        return v
    }
    return
}

