// _ PACKAGE flag
// helps to get app flags from console
// i can define flags with syntax
// -flag=x

// for example i have config structure
type AppConfig struct {
	url      string
	port     int
	someI64  int64
	someF64  float64
	someBool bool
	duration time.Duration
}

// _ PUT FLAGS INTO POINTERS
// at start i have to define variables for flags by type
// its like containers for variables
// if some are not entered into console, the default value will be inserted
// 		first param - name of flag
// 		second param - default value
// 		third param - description
// remember, all this flags is pointers
url := flag.String("url", "http://localhost:", "server url")
port := flag.Int("port", 3000, "server port")
someI64 := flag.Int64("someI64", 55, "int64 valiable")
someF64 := flag.Float64("someF64", 55.25, "float64 valiable")
someBool := flag.Bool("someBool", false, "some bool var")
duration := flag.Duration("duration", time.Second, "duration Var")
// ~  slog althogh has this functions
// flag.Uint()
// flag.UintVar()
// flag.Uint64()
// flag.Uint64Var()

// _ PUT FLAGS INTO VARS
var config AppConfig
flag.StringVar(&config.url, "url_2", "some_url", "url")
flag.IntVar(&config.port, "port_2", 3000, "port")
flag.Int64Var(&config.someI64, "someI64_2", 22, "i64")
flag.Float64Var(&config.someF64, "someF64_2", 22.22, "f64")
flag.BoolVar(&config.someBool, "someBool_2", true, "bool")
flag.DurationVar(&config.duration, "duration_2", time.Minute, "duration")

// now the vars have default values
// to owerwrite them with the entered values
// i need to call Parse func
flag.Parse()
fmt.Println(*url, *port, *someI64, *someF64, *someBool, *duration)
fmt.Println(config)
// ~ output without defined flags
// http://localhost: 3000 55 55.25 false 1s
// {some_url 3000 22 22.22 true}

// ~ output within defined flags
// go run ./basics/main.go -url="defined_url" -port_2=6666
// defined_url 3000 55 55.25 false
// {some_url 6666 22 22.22 true 60000000000}
