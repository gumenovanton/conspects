//>> НАЗНАЧЕНИЕ
// помогает работать с слайсами байтов
// практически полный аналог strings пакета
// например я могу создать буфур и читать из него
// буфер при этом опустошается

//<< NewBuffer()
// create a new buffer
buffer := bytes.NewBuffer([]byte("Hello, world!"))

//<< String()
// bytes from buffer to a string
fmt.Println(buffer.String())

//<< Write()
// erite to buffet
// merge slices
slice1 := []byte("Hello, ")
slice2 := []byte("world!")
buffer := bytes.NewBuffer(slice1)
buffer.Write(slice2)
fmt.Println(buffer.String())	
                            	
//<< Read()           
// read a part of buffer to another slice
//<< slice := Bytes()
// get buffer slice from buffer
slice := []byte("Hello, world!")
buffer := bytes.NewBuffer(slice)
part1 := make([]byte, 5)
buffer.Read(part1)
part2 := buffer.Bytes()
fmt.Println(string(part1))// Hello
fmt.Println(string(part2))//, world!                	

//<< WriteString()
// write string to byte buffer
slice := []byte("Hello, ")
buffer := bytes.NewBuffer(slice)
buffer.WriteString("world!")
fmt.Println(buffer.String())
                            	
//<< Index()
// index of first in a slice 
slice := []byte("Hello, world!")
buffer := bytes.NewBuffer(slice)
index := bytes.Index(buffer.Bytes(), []byte("world"))
fmt.Println(index)
