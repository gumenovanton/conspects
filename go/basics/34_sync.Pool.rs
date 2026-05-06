//>> sync.Pool
// когда нужно постоянно выделять память и стирать
// сборщик мусора постоянно запускается и жрет ресурсы
// вместо выделения памяти
// испольуй sync.Pool
// Создание пула.

//<< создание пула
// пул создается под опрееленный тип
var dataPool = sync.Pool{
    New: func() any {
       return make([]int, 0, 10000)
    },
}

// Работа с пулом.
func processPool() {
    data := dataPool.Get().([]int)
    // Некоторая обработка данных
    for i := 0; i < 10000; i++ {
       data = append(data, i)
    }

    // Очистка.
    data = data[:0]
    dataPool.Put(data)
}

SHORT
// так я могу задать размер буфера быстро
1<<10 = 1 024 (1 Килобайт)
1<<20 = 1 048 576 (1 Мегабайт)
1<<30 = 1 073 741 824 (1 Гигабайт)

BAST PRACTICES
// when i get chunks of the pool i will have new allocation on every get
// 1000 get -> 1000 allocations
// when i put chunks back to reuse it, when i reuse it i have no allocations
// 1000 put -> 1000 get without allocations
// but
// under high load when pool get GS launch point, GS clears all unused chunks from the pool
// and every new get will couse to new allocation

// nessesary
// when use pool, try to define size of data with input constraints, pagination etc...
// to choose the pool chunk size to get it, and to put it back, to be able to reuse it
// if some how you need to get memory for big object, just do not use pool with it
// to get rid of putting it back

// create chunck sizes
const (
	SmallSize  = 1024        // 1 КB for meta
	MediumSize = 64 * 1024   // 64 КB for posts
	LargeSize  = 512 * 1024  // 512 КB for haevy requestst
)

// MultiPool handle multiple pools
type MultiPool struct {
	small  sync.Pool
	medium sync.Pool
	large  sync.Pool
}

func NewMultiPool() *MultiPool {
    // create diffent pools
	return &MultiPool{
		small:  sync.Pool{New: func() any { return bytes.NewBuffer(make([]byte, 0, SmallSize)) }},
		medium: sync.Pool{New: func() any { return bytes.NewBuffer(make([]byte, 0, MediumSize)) }},
		large:  sync.Pool{New: func() any { return bytes.NewBuffer(make([]byte, 0, LargeSize)) }},
	}
}

// Get choose the best by size
func (mp *MultiPool) Get(size int) *bytes.Buffer {
	var buf *bytes.Buffer
	switch {
	case size <= SmallSize:
		buf = mp.small.Get().(*bytes.Buffer)
	case size <= MediumSize:
		buf = mp.medium.Get().(*bytes.Buffer)
	case size <= LargeSize:
		buf = mp.large.Get().(*bytes.Buffer)
	default:
	    // if data is too big, do not use pool
		return bytes.NewBuffer(make([]byte, 0, size))
	}
	buf.Reset()
	return buf
}

// Put chunks back to use it again
func (mp *MultiPool) Put(buf *bytes.Buffer) {
	c := buf.Cap()
	switch {
	case c <= SmallSize:
		mp.small.Put(buf)
	case c <= MediumSize:
		mp.medium.Put(buf)
	case c <= LargeSize:
		mp.large.Put(buf)
	default:
	//  do not put back too big buffer and hold it for GS
	}
}

var globalPool = NewMultiPool()

func main() {
    // example
	expectedSize := 40000
	buf := globalPool.Get(expectedSize)

	// write into buffer
	buf.WriteString("some data from db")

	// put chunk back
	globalPool.Put(buf)
}
