//>> ОТЧИСТКА ТЕРМИНАЛА
for i := 0; i < 5001; i++ {
    time.Sleep(1 * time.Millisecond)
    fmt.Print("\033[H\033[2J") // очистка терминала
    fmt.Printf("Live application running... %d\n", i)
}