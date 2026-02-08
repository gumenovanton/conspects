//>> НАЗНАЧЕНИЕ
// для динамического изменения вывода в консоль
github.com/gosuri/uilive

//>> СУТЬ
// представляет собой writer, который пишет буфер по умолчанию в Stdout
// указатель на который находится в поле writer.Out
writer := uilive.New()

// мне нужно заустить цикл рендера
// который обновляет буфер каждую милисекунду
// это значение можно переопределить
// writer.RefreshInterval = time.Duration(time.Millisecond)
writer.Start()

// я могу писать в буфер столько строк отражающих нужное мне состояние
// сколько успею за RefreshInterval
// они сразу выведутся в консоль
// и они не будут меняться
writer.Write([]byte("val 1: -\n"))
writer.Write([]byte("val 2: -\n"))
// чтобы их заменить нужно подождать минимум RefreshInterval
// когда буфер сбросится, максимум сколько угодно
time.Sleep(time.Millisecond * 1100)

// затем когда состояние изменилось можем поменять текст
writer.Write([]byte("val 1: --\n"))
writer.Write([]byte("val 2: --\n"))
time.Sleep(time.Millisecond * 1100)

// и еще раз
writer.Write([]byte("val 1: ---\n"))
writer.Write([]byte("val 2: ---\n"))
time.Sleep(time.Millisecond * 1100)

// я могу не ждать RefreshInterval
// а сбросить буфер ручками
writer.Write([]byte("val 1: ----\n"))
writer.Write([]byte("val 2: ----\n"))
time.Sleep(time.Millisecond * 100)
writer.Flush()
writer.Write([]byte("val 1: -----\n"))
writer.Write([]byte("val 2: -----\n"))
time.Sleep(time.Millisecond * 1100)

// эта запись будт записана на всегда, так как идет мимо буфера
writer.Bypass().Write([]byte("bypas\n"))
// тут будет пауза на RefreshInterval, зачем не знаю но она всегда есть
// по этому больой интервал будет давать паузу на появление следующих двух линий
writer.Write([]byte("val 1: ------\n"))
writer.Write([]byte("val 2: ------\n"))
time.Sleep(time.Millisecond * 1100)

// остановка работы
writer.Stop() // flush and stop rendering

//>> ЗАПИСЬ С ПОМОЩЬЮ Fprint
// coздаю writer
writer := uilive.New()
	writer.Start()

_, _ = fmt.Fprintf(writer, "-\n")
_, _ = fmt.Fprintf(writer, "-\n")
time.Sleep(time.Millisecond * 1000)
_, _ = fmt.Fprintf(writer, "--\n")
_, _ = fmt.Fprintf(writer, "--\n")
time.Sleep(time.Millisecond * 1000)
_, _ = fmt.Fprintf(writer, "---\n")
_, _ = fmt.Fprintf(writer, "---\n")
// эта запись выйдет в начало и отсанется навсегда
_, _ = fmt.Fprintf(writer.Bypass(), "bypas\n")
time.Sleep(time.Millisecond * 1000)
// остановка работы
writer.Stop() // flush and stop rendering