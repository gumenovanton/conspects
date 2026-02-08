//>> ПАКЕТ ДЛЯ РИСОВАНИЯ ГРАФИКОВ В ТЕРМИНАЛ
//go get -u github.com/guptarohit/asciigraph@latest

//<< ВЫВОД ОДНОГО ГРАФИКА
data := []float64{3, 4, 9, 6, 2, 4, 5, 8, 5, 10, 2, 7, 2, 5, 6}
graph := asciigraph.Plot(data)
fmt.Println(graph)

10.00 ┤        ╭╮
9.00 ┤ ╭╮     ││
8.00 ┤ ││   ╭╮││
7.00 ┤ ││   ││││╭╮
6.00 ┤ │╰╮  ││││││ ╭
5.00 ┤ │ │ ╭╯╰╯│││╭╯
4.00 ┤╭╯ │╭╯   ││││
3.00 ┼╯  ││    ││││
2.00 ┤   ╰╯    ╰╯╰╯

//<< ВЫВОД НЕСКОЛЬКИХ ГРАФИКОВ
data := [][]float64{{0, 1, 2, 3, 3, 3, 2, 0}, {5, 4, 2, 1, 4, 6, 6}}
graph := asciigraph.PlotMany(data)
fmt.Println(graph)

6.00 ┤    ╭─
5.00 ┼╮   │
4.00 ┤╰╮ ╭╯
3.00 ┤ │╭│─╮
2.00 ┤ ╰╮│ ╰╮
1.00 ┤╭╯╰╯  │
0.00 ┼╯     ╰

//<< ЦВЕТНЫЕ ГРАФИКИ
data := [][]float64{{0, 1, 2, 3, 3, 3, 2, 0}, {5, 4, 2, 1, 4, 6, 6}}
// здесь задаю точость, нет нулей на графике
graph := asciigraph.PlotMany(data, asciigraph.Precision(0), asciigraph.SeriesColors(
    asciigraph.Red,
    asciigraph.Yellow,
))
fmt.Println(graph)

// будут цветными
6┤    ╭─
5┼╮   │
4┤╰╮ ╭╯
3┤ │╭│─╮
2┤ ╰╮│ ╰╮
1┤╭╯╰╯  │
0┼╯     ╰

//<< ПОДПИСИ И ДРУГИЕ ПРАМЕТРЫ
data := [][]float64{{0, 1, 2, 3, 3, 3, 2, 0}, {5, 4, 2, 1, 4, 6, 6}}
graph := asciigraph.PlotMany(data,
    asciigraph.LowerBound(5), // мин и макс значение, если превышения то это будет игнорировано
    asciigraph.UpperBound(5),
    asciigraph.Precision(3), // точность знаков после запятой
    asciigraph.Width(10),    // масштабы
    asciigraph.Height(10),
    asciigraph.SeriesColors(asciigraph.Red, asciigraph.Green), // цвета графиков
    asciigraph.SeriesLegends("Red Label", "Green Label"),      // лейблы подписей
    asciigraph.AxisColor(asciigraph.Red),                      // цвет оси
    asciigraph.LabelColor(asciigraph.Red),                     // цвет оси
    asciigraph.Caption("Series with legends"),                 // название графика
    asciigraph.CaptionColor(asciigraph.Red))                   // цвет название графика
fmt.Println(graph)
// будут цветными
6.000 ┤      ╭╴
5.400 ┤     ╭╯
4.800 ┼╮    │
4.200 ┤╰╮   │
3.600 ┤ │  ╭╯
3.000 ┤ ╰╮╭│─╮
2.400 ┤  │╯│ ╰╮
1.800 ┤ ╭╰╮│  ╰╮
1.200 ┤ │ ╰╯   │
0.600 ┤╭╯      │
0.000 ┼╯       ╰
       Series with legends

       ■ Red Label   ■ Green Label

  //<< CONFIG
  type config struct {
	Width, Height          int          // масштаб, по умолчанию 
	LowerBound, UpperBound *float64
	Offset                 int
	Caption                string
	Precision              uint
	CaptionColor           AnsiColor
	AxisColor              AnsiColor
	LabelColor             AnsiColor
	SeriesColors           []AnsiColor      // массив цветов
	SeriesLegends          []string         // массив подписей
}