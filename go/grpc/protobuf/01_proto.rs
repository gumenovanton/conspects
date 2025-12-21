//>> ЗАДАЧА
// для быстрого общения между сервисами

//>> УСТАНОВКА
- go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
- go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
- export GO_PATH=~/go
- export PATH=$PATH:/$GO_PATH/bin
- go get google.golang.org/grpc

//>> .PROTO файл
//<< где
// в корне в папке api создаем папку с названием сервиса
// в ней создаем файл service.proto

//<< начало
// версия протокола
syntax = "proto3";
// пакет
package some_name;// нужно для импорта одного в другой
// куда генерим код
option go_package = "/pkg/gen";

//<< как определить сообщение
// в сообщении  нужно перечислить поля
// указать тип, имя и порядковый номер (все переменные будут летать под номерами) 
message Section {
    string title = 1; // поля начинаются с единицы
    string content = 2; 
}

//<< основные типы
// PROTO - GOLANG
// int - int32
// int64 - int64
// uint32 - uint32
// uint64 - uint64
// float - float32
// double - float64
// bool - bool
// string - string
// bytes - []bytes

//<< полем сообщения может быть перечисление
// это набор строго определенных вариантов значений которые вообще можно передать в поле
enum DayOfWeek {
    DAY_OF_WEEK_UNSPECIFIED = 0;// начинаются с 0 но в ноль всегда кладем заглушку
    MONDAY = 1;
    TUESDAY = 2;
    WEDNESDAY = 3;
    THURSDAY = 4;
    FRIDAY = 5;
    SATURDAY = 6;
    SUNDAY = 7;
}

//<< как определить сообщение с полем перечисления
message Section {
    string title = 1; 
    string content = 2; 
    DayOfWeek type = 3; // здесь могут лежать только значения определенные в enum
}

//<< передача массивов и вложенные сообщения
// здесь я передаю массив сообщений Section
// для обозначения массива ставлю слово repeated
message TechnicalReview {
    repeated Section sections = 1; // Массив разделов обзора
} 

//<< могу передать мапу
message Environment {
    map<string, string> variables = 1;
}

//<< Могу указать поле как опционное
message Message {
    optional string bar = 1;//если поле не будет определено, в него будет помещено значение типа по умолчанию
}

//<< могу зарезервировать номера полей и имена
// имена резервировать вообще нет смысла
// но когда я думаю что в сообщении будет что то добавлено, я могу зарезервировать несколько полей
// и если я меняю API и нужно удалить поле, нужно его зарезервировать
// тогда мое взаимодействие не сломается 
message Foo {
    reserved 2, 15, 9 to 11, 40 to max;
    reserved "foo", "bar";
}

//<< могу задавать сообщения как вложенные
message SearchResponse {
    message Result {
      string url = 1;
      string title = 2;
      repeated string snippets = 3;
    }
    repeated Result results = 1;
}

// переиспользование 
message SomeOtherMessage {
    SearchResponse.Result result = 1;
}

//>> КОДОГЕНЕРАЦИЯ
protoc -I=api --go_out=. --go-grpc_out=. movie.proto 
// -I=api - положить в папку api  
// --go_out=. - флаг говорящий что надо генерировать protobuf и положить в gen(эта папка указана в файле .proto в строке - option go_package = "/gen";)
// --go-grpc_out=. - флаг говорящий что надо генерировать gRPC и положить в gen(эта папка указана в файле .proto в строке - option go_package = "/gen";)
// movie.proto - имя файла с которого генерируем код

//>> ЛУЧШИЕ ПРАКТИКИ
// - не меняй порядок полей, номера, имена, посто сделай новую версию    
// - удаляя резервируй поля
// - используй один файл на клиенте и на сервере, чтобы все было одинактво
