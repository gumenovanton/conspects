//>> FETCH GET запрос
// на клиенте можно не писать абсолютный адрес, можно только относительный
let res = await fetch('/api/login')

// из ответа можно читать статус ответа и его тексе
res.status
res.statusText

// могу проверить успешность статуса ответа
res.ok

// могу посмотреть заголовки
res.headers.get('Content-Type')
res.headers.get('Date')

// могу посмотреть тип запроса, то есть окуда пришли
// basic - внутри сайтовые запросы, на них нет ограничений
// cors - междоменный запрос, накладывает ограничения на получения метаданных("Cash-Control", "Content-Language", "Content-Type", "Expires", "Last-Modified", "Pragma")
// opaque - запросы таких типов приходят с других доменов, и я не могу судить об их статусе
res.type

// могу посмотреть тип url
res.url

// могу распарсить тело в строку
let text = await res.text()

// могу распарсить тело в обьект, или массив
let data = await res.json()

// могу распарсить FormData
let formData = await res.formData()

// и еще кое что (смотри в коде пакета)

//<< установка mode запроса
// можно установить ожидаемый тип
// "same-origin" - запросы на другой origin будут отклонены
// "cors" - то же самое, но добавляет возможность делать запросы к сторонним сайтам и указывать cors заголовки
// "cors-with-forced-preflight" - то же что и cors, только преед запростом отсылает запрос на проверку
// "no-cors" - для запросов к origin который не присылает cors заголовков 
res = await fetch('/api/login', { mode: 'cors' })


//>> FETCH POST запрос
res = await fetch('/api/login', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
    },
    body: '{"name":"samuel"}'
})

// если хочу послать куки(аутентификации, но это нне точно)
// надо почитать 
fetch('/api/login', {
    credentials: 'include'
})