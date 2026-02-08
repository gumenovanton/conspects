//>> SELECT
// аналог switch case только для каналов

//>> ПОЛУЧЕНИЕ БЕЗ БЛОКИРОВКИ
// сколько каналов будем слушать
openChannals := 2
for {
    select {
        // если в этом канале что то есть    
        case val, ok := <- chanOne:
            if ok {
                // если канал открыт и есть значение
                // выводим его
                fmt.Println(val)
            } else {
                // если канал закрыт
                fmt.Println("Channel has been closed")
                // делаем этот канал недоступным для select
                // чтобы он больше не проверял этот канал
                dispatchChannel = nil
                // уменбшаем количесво прослушивваемых каналов
                openChannels--
            }
            // если в этом канале что то есть    
        case val, ok := <- chanTwo:
            if ok {
                // если канал открыт и есть значение
                // выводим его
                fmt.Println(val)
            } else {
                // если канал закрыт
                fmt.Println("Channel has been closed")
                // делаем этот канал недоступным для select
                // чтобы он больше не проверял этот канал
                dispatchChannel = nil
                // уменбшаем количесво прослушивваемых каналов
                openChannels--
            }
        // если не по одному каналу не пришли данные
        default:
            // если они оба закрыты
            if (openChannels == 0) {
                // выходим из цикла
                goto alldone
            }
            // если открыты, значит нет значения, подождем
            //! ВМЕСТО ОЖИДАНИЯ МОЖНО ДЕЛАТЬ КАКИЕ ТО ДРУГИЕ ДЕЙСТВИЯ
            fmt.Println("-- No message ready to be  received")
            time.Sleep(time.Millisecond * 500)
            //! default выполняется только 1 раз
    }
}
alldone: fmt.Println("All values received")

//>> ОТПРАВКА БЕЗ БЛОКИРОВКИ
// так же я могу отправлять без блокировки
func enumerateProducts(channel chan<- *Product) {
    for _, p := range ProductList {
        // типа если канал свободен отправляю
        select {
            case channel <- p:
            fmt.Println("Sent product:", p.Name)
        // иначе посплю
        default:
            fmt.Println("Discarding product:", p.Name)
            time.Sleep(time.Second)
        }
    }
    close(channel)
}

//>> DEFAULT
// если не поставить секцию default
// select заблокирует горутину 
// пока по какому то каналу не прийдут данные