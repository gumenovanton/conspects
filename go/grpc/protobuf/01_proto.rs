# PROTOBUF
// контракт между клиентом и сервером
// описывает какие методы может вызвать клиент и типы передаваемых параметров
// один proto файл используется и на клиенте и на сервере

# ЗАЧЕМ
// для быстрого общения между сервисами
// сука очень быстрый

# .PROTO ФАЙЛ
//<< где
// в корне в папке cmd создаем папку с названием сервиса
// в ней создаем файл service.proto

# ПРИМЕР
syntax = "proto3";

package user_v1; // для передачи одного в другой

// этот параметр используется только в GO
option go_package = "package name/pkg/user_v1/;user_v1" 
// package name/pkg/user_v1 - хз что это, надо разобраться
// user_v1 - такое имя получит пакет в генерируемом файле

// это кастомный тип данных
message UserInfo{
    int64 id = 1;
    string name = 2;
    bool is_human = 3;
}

//запрос 
message GetRequest{
    int64 id = 1;
}

// ответ
message GetResponse{
    UserInfo info = 1;
}
// это метод который будет дергать клиент
service UserV1{
    rpc Get(GetRequest) returns(GetResponse)
}

# ТИПЫ ДАННЫХ И ИХ АНАЛОГИ В GO
//  PROTO       GO
//  double      float32
//  float       float64
//  int32       int32
//  int64       int64
//  uint32      uint32
//  uint64      uint64
//  sint32      int32 - uptimized for negative numbers
//  sint64      int32 - uptimized for negative numbers
//  fixed32     fixed32 - uptimized for big numbers
//  fixed64     fixed64 - uptimized for big numbers
//  bool, string
//  bytes       []byte - for files
//  golang nulable type - all types can be this type, it means no value

# ENUM
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

# СООБЩЕНИЕ С ENUM
message Section {
    string title = 1; 
    string content = 2; 
    DayOfWeek type = 3; // здесь могут лежать только значения определенные в enum
}

# МАССИВ С ВЛОЖЕННЫМ СООБЩЕНИЕМ
// здесь я передаю массив сообщений Section
// для обозначения массива ставлю слово repeated
message TechnicalReview {
    repeated Section sections = 1; // Массив разделов обзора
} 

# МАПА
message Environment {
    map<string, string> variables = 1;
}

# ОПЦИОНАЛЬНОЕ ПОЛЕ
message Message {
    optional string bar = 1;//если поле не будет определено, в него будет помещено значение типа по умолчанию
}

# РЕЗЕРВИРОВАНИЕ ПОЛЕЙ ДЛЯ БУДУЩЕГО ИСПОЛЬЗОВАНИЯ
// имена резервировать вообще нет смысла
// но когда я думаю что в сообщении будет что то добавлено, я могу зарезервировать несколько полей
// и если я меняю API и нужно удалить поле, нужно его зарезервировать
// тогда мое взаимодействие не сломается 
message Foo {
    reserved 2, 15, 9 to 11, 40 to max;
    reserved "foo", "bar";
}

# ВЛОЖЕННЫЕ СООБЩЕНИЯ
message SearchResponse {
    message Result {
      string url = 1;
      string title = 2;
      repeated string snippets = 3;
    }
    repeated Result results = 1;
}

# ПЕРЕИСПОЛЬЗОВАНИЕ 
message SomeOtherMessage {
    SearchResponse.Result result = 1;
}

# ТИП УСТАНАВЛИВАЕМОГО СОЕДИНЕНИЯ
service MyService {
  // Унарный метод (запрос-ответ)
  rpc UnaryMethod(Request) returns (Response) {};
  
  // Серверный поток
  rpc ServerStreamMethod(Request) returns (stream Response) {};
  
  // Клиентский поток
  rpc ClientStreamMethod(stream Request) returns (Response) {};
  
  // Двунаправленный поток
  rpc BidirectionalStreamMethod(stream Request) returns (stream Response) {};
}