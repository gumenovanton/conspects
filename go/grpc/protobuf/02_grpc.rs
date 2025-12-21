//>> УСТАНОВКА
- go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
- go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
- export GO_PATH=~/go
- export PATH=$PATH:/$GO_PATH/bin
- go get google.golang.org/grpc

//>> В .proto ОПИСЫВАЕМ СЕРВИС
//<< где
// в корне в папке api создаем папку с названием сервиса
// в ней создаем файл service_grpc.proto

//<< начало
// версия протокола
syntax = "proto3";
// пакет
package some_name;// нужно для импорта одного в другой
// куда генерим код
option go_package = "/pkg/gen";

//<< как
// gRPC сервис MetadataService
service MetadataService {
    // дергалка функции получения данных
    rpc GetMetadata(GetMetadataRequest) returns (GetMetadataResponse);
    // дергалка функции записи данных
    rpc PutMetadata(PutMetadataRequest) returns (PutMetadataResponse);
}
// имя запроса формирую из дергалки плюс Request
// имя ответа формирую из дергалки плюс Response
//! в grpc только один параметр в запросе и только один в ответе

//>> И СООБЩЕНИЯ
// GetMetadata просит идентификатор
message GetMetadataRequest {
    string movie_id = 1;
}

// GetMetadata возвращает Metadata
message GetMetadataResponse {
    Metadata metadata = 1;
}

// PutMetadata просит Metadata
message PutMetadataRequest {
    Metadata metadata = 1;
}

// PutMetadata возвращает чет не че не возвращает
message PutMetadataResponse {
}