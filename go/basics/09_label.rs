//>> LABEL
// переход по метке
counter := 0
target: fmt.Println("Counter", counter)
counter++
if (counter < 5) {
    goto target
}   