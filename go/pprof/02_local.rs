CREATE PROFILE INTO A FILE
// i can create app like this
func main() {
	wd, err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}

	// create a file for profiles
	f, err := os.Create(filepath.Join(wd, "cpu.out"))
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	// start CPU profiling
	if err := pprof.StartCPUProfile(f); err != nil {
		log.Fatal(err)
	}
	// end profiling at the and of the app
	defer pprof.StopCPUProfile()

	wg := new(sync.WaitGroup)
	for i := 0; i < runtime.NumCPU()/2; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()

			for range int(10e6) {
				root()
			}
		}()
	}

	wg.Wait()
}

var v atomic.Int32

//go:noinline
func root() {
	f2()
	f3()
	f4()
}

//go:noinline
func f2() {
	v.Add(1)
}

//go:noinline
func f3() {
	v.Add(1)
}

//go:noinline
func f4() {
	v.Add(1)
}

HOW TO WATCH A PROFILE FILE
$ go tool pporf cpu.out

COMMANDS
help - help
top - top entries
    // gives output with columns:
        // flat - resources used by this func without inner funcs
        // cum - resources used by this func and all inner funcs
peek func_name - info about the concreate func
disasm furc_name - assembler code of the func
tree - tree of calls

// the better way is to use veb version with comand
web - open web version
// in this interface
// - flat - size of block
// - cum - color of block
