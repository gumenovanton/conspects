//>> АТОМАРНАЯ ОПЕРАЦИЯ
// операция считается атомарной, 
// если она выполняется как единое целое 
// без возможности прерывания

// то есть в момент операции не один поток 
// и ни одна горутина не могут получить доступ к данным
// кроме потоки и горутины которые выполняют ее
// что полностью исключает data rase

//>> ПЛЮСЫ И МИНУСЫ
//<< ПЛЮСЫ
// пиздец быстро
// не требует мьютексов при расшаривании данных между потоками
// потокобезовасны

//<< МИНУСЫ
// работает только с int32, int64, uint32, uint64, uintptr
// можно сделать только одну операцию 
// так как в промежутке ктото может сделать еще одну

//>> ИСПОЛЬЗОВАНИЕ
// если есть расшариваемые данные типов int32, int64, uint32, uint64, uintptr
// ели они используются в разных потоках, читаются и изменяются
// но над ними делается только одна операция в каждом потоке
// то их можно менять и читать с помощью атомарных операций
// не очкуя, то есь безопасно

//>> ФУНКЦИИ

//<< атомарное добавление 
var i int32
atomic.AddInt32(&i, 10)

var j int64
atomic.AddInt64(&j, 10)

var l uint32
atomic.AddUint32(&l, 10)

var k uint64
atomic.AddUint64(&k, 10)

//<< атомарное чтение
fmt.Println(atomic.LoadInt32(&i))  // 10
fmt.Println(atomic.LoadInt64(&j))  // -10
fmt.Println(atomic.LoadUint32(&l)) // 5
fmt.Println(atomic.LoadUint64(&k)) // 25

//<< атомарная записть конкретного значения
atomic.StoreInt32(&i, 100)
atomic.StoreInt64(&j, 50)
atomic.StoreUint32(&l, 25)
atomic.StoreUint64(&k, 12)

fmt.Println(atomic.LoadInt32(&i))  // 100
fmt.Println(atomic.LoadInt64(&j))  // 50
fmt.Println(atomic.LoadUint32(&l)) // 25
fmt.Println(atomic.LoadUint64(&k)) // 12

//<< атомарно записать и достать старое значение
fmt.Println(atomic.SwapInt32(&i, 200)) // 100
fmt.Println(atomic.SwapInt64(&j, 100)) // 50
fmt.Println(atomic.SwapUint32(&l, 50)) // 25
fmt.Println(atomic.SwapUint64(&k, 24)) // 12

fmt.Println(atomic.LoadInt32(&i))  // 200
fmt.Println(atomic.LoadInt64(&j))  // 100
fmt.Println(atomic.LoadUint32(&l)) // 50
fmt.Println(atomic.LoadUint64(&k)) // 24

//<< атомарная вставка но только при условии
// сделает атомарную запись, если старое значение равно определенной величине
// вернет true если записть произошла, иначе false
result1 := atomic.CompareAndSwapInt32(&i, 100, 200) // если сейчас переменная равна 100 то запистаь в ее значение 200
result2 := atomic.CompareAndSwapInt64(&j, 50, 100)  // если сейчас переменная равна 50 то запистаь в ее значение 100
result3 := atomic.CompareAndSwapUint32(&l, 25, 50)  // если сейчас переменная равна 25 то запистаь в ее значение 50
result4 := atomic.CompareAndSwapUint64(&k, 12, 24)  // если сейчас переменная равна 12 то запистаь в ее значение 24