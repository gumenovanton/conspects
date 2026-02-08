//>> errgroup
// it's like WaitGroup
// but cancel context if one goroutine returns an error

//<< when to use 
// if a want to sum results of many goroutines
// if one gorutine returned error
// i do not want continue other goroutines
// with ErrorGroup i just cancel them

func main() {
    // to use it i must create an error group
    // it returns context that will be caneled 
    // if one gorutine returns error
	g, gctx := errgroup.WithContext(context.Background())

    // then i starts all my goroutines
    // where i send reguests for example
	for i := 0; i < 64; i++ {
		g.Go(func() error {
            // i must pass error group ctx to cancel it
			return someFuncThatDoRequest(gctx)
		})
	}

    // anh just wait all gorutines
    // only first error will be retirned
	if err := g.Wait(); err != nil {
		fmt.Println("Ошибка:", err)
	}
}
