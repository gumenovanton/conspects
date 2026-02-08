//>> КОМПОНЕНТЫ Request
fmt.Println(request.Method)           // GET
fmt.Println(request.URL)              // /
fmt.Println(request.Proto)            // HTTP/1.1
fmt.Println(request.ProtoMajor)       // 1
fmt.Println(request.ProtoMinor)       // 1
fmt.Println(request.Host)             // localhost:5000
fmt.Println(request.Header)           // map[Accept:[text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7] Accept-Encoding:[gzip, deflate, br, zstd] Accept-Language:[ru,en;q=0.9,fil;q=0.8] Cache-Control:[max-age=0] Connection:[keep-alive] Sec-Ch-Ua:["Chromium";v="124", "YaBrowser";v="24.6", "Not-A.Brand";v="99", "Yowser";v="2.5"] Sec-Ch-Ua-Mobile:[?0] Sec-Ch-Ua-Platform:["Linux"] Sec-Fetch-Dest:[document] Sec-Fetch-Mode:[navigate] Sec-Fetch-Site:[none] Sec-Fetch-User:[?1] Upgrade-Insecure-Requests:[1] User-Agent:[Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 YaBrowser/24.6.0.0 Safari/537.36]]
fmt.Println(request.Trailer)          // map[]
fmt.Println(request.ContentLength)    // 0
fmt.Println(request.RemoteAddr)       // [::1]:59882
fmt.Println(request.TransferEncoding) // []
fmt.Println(request.Body)             // {}

//>> ПОЛУЧЕНИЕ Body
// Body можно прочитать любыми методами чтения из Reader
data, _ := io.ReadAll(request.Body)
fmt.Println(string(data))
defer request.Body.Close()

// а можно и отправить в Writer
io.Copy(writer, request.Body)

//>> ПОЛУЧЕНИЕ Query ПАРАМЕТРОВ
// /snippet/view?id=1
request.URL.Query().Get("id")